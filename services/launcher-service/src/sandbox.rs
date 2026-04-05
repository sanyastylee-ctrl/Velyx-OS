use crate::manifest::TrustLevel;
use crate::portals::{future_device_portal_contract, future_file_portal_contract};
use crate::seccomp::placeholder_for_profile;
use chrono::Utc;
use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Clone, Debug)]
pub enum MountAccess {
    ReadOnly,
    ReadWrite,
}

impl MountAccess {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ReadOnly => "ro",
            Self::ReadWrite => "rw",
        }
    }
}

#[derive(Clone, Debug)]
pub struct MountRule {
    pub host_path: PathBuf,
    pub guest_path: String,
    pub access: MountAccess,
    pub source: String,
}

#[derive(Clone, Debug)]
pub struct SandboxPolicy {
    pub profile_name: String,
    pub mount_policy: String,
    pub tmp_policy: String,
    pub network_access: bool,
    pub visible_host_paths: Vec<MountRule>,
    pub env_allowlist: Vec<String>,
    pub seccomp_profile: String,
}

#[derive(Clone, Debug)]
pub struct SandboxLaunchRequest {
    pub app_id: String,
    pub display_name: String,
    pub executable_path: String,
    pub sandbox_profile: String,
    pub trust_level: TrustLevel,
    pub permission_context: HashMap<String, String>,
    pub launched_by: String,
}

#[derive(Clone, Debug)]
pub struct ProcessIdentity {
    pub pid: u32,
    pub app_id: String,
    pub sandbox_profile: String,
    pub launch_time: String,
    pub launch_status: String,
    pub sandbox_id: String,
    pub launched_by: String,
}

pub struct SandboxLaunchResult {
    pub identity: ProcessIdentity,
    pub runtime_path: String,
    pub applied_mounts: Vec<String>,
    pub filtered_env: Vec<String>,
}

pub struct SandboxRunner;

impl SandboxRunner {
    pub fn launch(request: &SandboxLaunchRequest) -> Result<SandboxLaunchResult, String> {
        #[cfg(not(target_os = "linux"))]
        {
            let _ = request;
            Err("sandbox enforcement v2 доступен только на Linux".to_string())
        }

        #[cfg(target_os = "linux")]
        {
            Self::launch_linux(request)
        }
    }

    pub fn build_policy(request: &SandboxLaunchRequest) -> Result<SandboxPolicy, String> {
        let profile_name = if request.sandbox_profile.is_empty() {
            "minimal".to_string()
        } else {
            request.sandbox_profile.clone()
        };
        let profile_defaults = profile_defaults(&profile_name)?;
        let mut visible_host_paths = Vec::new();

        if profile_defaults.allow_documents && has_allow(&request.permission_context, "filesystem") {
            visible_host_paths.extend(permission_paths_for_profile(&profile_name));
        }

        Ok(SandboxPolicy {
            profile_name: profile_name.clone(),
            mount_policy: profile_defaults.mount_policy,
            tmp_policy: profile_defaults.tmp_policy,
            network_access: profile_defaults.network_access,
            visible_host_paths,
            env_allowlist: profile_defaults.env_allowlist,
            seccomp_profile: placeholder_for_profile(&profile_name).id,
        })
    }

    #[cfg(target_os = "linux")]
    fn launch_linux(request: &SandboxLaunchRequest) -> Result<SandboxLaunchResult, String> {
        let policy = Self::build_policy(request)?;
        let executable = PathBuf::from(&request.executable_path)
            .canonicalize()
            .map_err(|err| format!("invalid executable path: {err}"))?;
        let executable_parent = executable
            .parent()
            .ok_or_else(|| "executable parent is missing".to_string())?;
        let executable_name = executable
            .file_name()
            .ok_or_else(|| "executable file name is missing".to_string())?
            .to_string_lossy()
            .to_string();
        let sandbox_id = format!("sandbox-{}-{}", request.app_id, Utc::now().timestamp_millis());
        let mut applied_mounts = vec![
            format!("mount_policy={}", policy.mount_policy),
            format!("tmp_policy={}", policy.tmp_policy),
            format!("seccomp={}", policy.seccomp_profile),
            format!("display_name={}", request.display_name),
            format!("trust_level={}", request.trust_level.as_str()),
        ];

        let mut command = Command::new("bwrap");
        command.arg("--die-with-parent");
        command.arg("--new-session");
        command.arg("--unshare-pid");
        command.arg("--unshare-uts");
        command.arg("--proc").arg("/proc");
        command.arg("--dev").arg("/dev");
        if !policy.network_access {
            command.arg("--unshare-net");
        }
        command.arg("--tmpfs").arg("/tmp");
        command.arg("--dir").arg("/app");
        command.arg("--clearenv");
        command.arg("--setenv").arg("VELYX_APP_ID").arg(&request.app_id);
        command
            .arg("--setenv")
            .arg("VELYX_SANDBOX_PROFILE")
            .arg(&policy.profile_name);
        command
            .arg("--setenv")
            .arg("VELYX_LAUNCHED_BY")
            .arg(&request.launched_by);
        command
            .arg("--setenv")
            .arg("VELYX_SANDBOX_ID")
            .arg(&sandbox_id);

        let allowed_env = apply_env_allowlist(&mut command, &policy.env_allowlist);
        apply_system_mounts(&mut command, &mut applied_mounts);
        command
            .arg("--ro-bind")
            .arg(executable_parent)
            .arg("/app/runtime");
        applied_mounts.push(format!(
            "runtime:{}->/app/runtime:{}",
            executable_parent.display(),
            MountAccess::ReadOnly.as_str()
        ));

        for rule in &policy.visible_host_paths {
            if rule.host_path.exists() {
                match rule.access {
                    MountAccess::ReadOnly => {
                        command
                            .arg("--ro-bind")
                            .arg(&rule.host_path)
                            .arg(&rule.guest_path);
                    }
                    MountAccess::ReadWrite => {
                        command.arg("--bind").arg(&rule.host_path).arg(&rule.guest_path);
                    }
                }
                applied_mounts.push(format!(
                    "{}:{}->{}:{}",
                    rule.source,
                    rule.host_path.display(),
                    rule.guest_path,
                    rule.access.as_str()
                ));
            }
        }

        let _ = future_file_portal_contract();
        let _ = future_device_portal_contract();

        let runtime_target = format!("/app/runtime/{executable_name}");
        command.arg(runtime_target.clone());

        let child = command
            .spawn()
            .map_err(|err| format!("failed to launch sandboxed process: {err}"))?;

        Ok(SandboxLaunchResult {
            identity: ProcessIdentity {
                pid: child.id(),
                app_id: request.app_id.clone(),
                sandbox_profile: policy.profile_name,
                launch_time: Utc::now().to_rfc3339(),
                launch_status: "launched".to_string(),
                sandbox_id,
                launched_by: request.launched_by.clone(),
            },
            runtime_path: runtime_target,
            applied_mounts,
            filtered_env: allowed_env,
        })
    }
}

fn has_allow(permission_context: &HashMap<String, String>, permission: &str) -> bool {
    permission_context
        .get(permission)
        .map(|decision| decision == "allow")
        .unwrap_or(false)
}

fn home_dir() -> PathBuf {
    let home = env::var("HOME")
        .or_else(|_| env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home)
}

fn permission_paths_for_profile(profile: &str) -> Vec<MountRule> {
    let home = home_dir();
    match profile {
        "files" => vec![
            MountRule {
                host_path: home.join("Documents"),
                guest_path: "/workspace/Documents".to_string(),
                access: MountAccess::ReadWrite,
                source: "permission:filesystem".to_string(),
            },
            MountRule {
                host_path: home.join("Downloads"),
                guest_path: "/workspace/Downloads".to_string(),
                access: MountAccess::ReadWrite,
                source: "permission:filesystem".to_string(),
            },
        ],
        "browser" => vec![MountRule {
            host_path: home.join("Downloads"),
            guest_path: "/workspace/Downloads".to_string(),
            access: MountAccess::ReadWrite,
            source: "permission:filesystem".to_string(),
        }],
        "trusted-system" => vec![
            MountRule {
                host_path: home.join("Documents"),
                guest_path: "/workspace/Documents".to_string(),
                access: MountAccess::ReadWrite,
                source: "permission:filesystem".to_string(),
            },
            MountRule {
                host_path: home.join("Pictures"),
                guest_path: "/workspace/Pictures".to_string(),
                access: MountAccess::ReadOnly,
                source: "permission:filesystem".to_string(),
            },
        ],
        _ => vec![MountRule {
            host_path: home.join("Documents"),
            guest_path: "/workspace/Documents".to_string(),
            access: MountAccess::ReadOnly,
            source: "permission:filesystem".to_string(),
        }],
    }
}

struct ProfileDefaults {
    mount_policy: String,
    tmp_policy: String,
    network_access: bool,
    env_allowlist: Vec<String>,
    allow_documents: bool,
}

fn profile_defaults(profile: &str) -> Result<ProfileDefaults, String> {
    let base_env = vec![
        "LANG".to_string(),
        "LC_ALL".to_string(),
        "PATH".to_string(),
        "TZ".to_string(),
        "DISPLAY".to_string(),
        "WAYLAND_DISPLAY".to_string(),
        "XDG_RUNTIME_DIR".to_string(),
        "DBUS_SESSION_BUS_ADDRESS".to_string(),
        "PULSE_SERVER".to_string(),
    ];

    let defaults = match profile {
        "minimal" => ProfileDefaults {
            mount_policy: "system_ro_only".to_string(),
            tmp_policy: "private_tmpfs".to_string(),
            network_access: false,
            env_allowlist: base_env,
            allow_documents: false,
        },
        "desktop-basic" => ProfileDefaults {
            mount_policy: "system_ro_with_desktop_io".to_string(),
            tmp_policy: "private_tmpfs".to_string(),
            network_access: false,
            env_allowlist: base_env,
            allow_documents: true,
        },
        "browser" => ProfileDefaults {
            mount_policy: "system_ro_with_downloads".to_string(),
            tmp_policy: "private_tmpfs".to_string(),
            network_access: true,
            env_allowlist: base_env,
            allow_documents: true,
        },
        "files" => ProfileDefaults {
            mount_policy: "system_ro_with_workspace_rw".to_string(),
            tmp_policy: "private_tmpfs".to_string(),
            network_access: false,
            env_allowlist: base_env,
            allow_documents: true,
        },
        "trusted-system" => ProfileDefaults {
            mount_policy: "system_ro_extended_host_paths".to_string(),
            tmp_policy: "private_tmpfs".to_string(),
            network_access: true,
            env_allowlist: base_env,
            allow_documents: true,
        },
        other => {
            return Err(format!(
                "sandbox profile '{}' не зарегистрирован; запуск запрещен",
                other
            ))
        }
    };

    Ok(defaults)
}

#[cfg(target_os = "linux")]
fn apply_system_mounts(command: &mut Command, applied_mounts: &mut Vec<String>) {
    for path in ["/usr", "/bin", "/lib", "/lib64", "/sbin", "/etc"] {
        if Path::new(path).exists() {
            command.arg("--ro-bind").arg(path).arg(path);
            applied_mounts.push(format!("profile:{}->{}:ro", path, path));
        }
    }
}

#[cfg(target_os = "linux")]
fn apply_env_allowlist(command: &mut Command, keys: &[String]) -> Vec<String> {
    let mut allowed = Vec::new();
    for key in keys {
        if let Ok(value) = env::var(key) {
            command.arg("--setenv").arg(key).arg(&value);
            allowed.push(key.clone());
        }
    }
    allowed
}
