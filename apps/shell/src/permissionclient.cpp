#include "permissionclient.h"

#include <QDBusConnection>
#include <QDBusInterface>
#include <QDBusMessage>
#include <QDBusReply>
#include <QDateTime>
#include <QDir>
#include <QFile>
#include <QProcess>
#include <QRegularExpression>
#include <QStandardPaths>
#include <QTextStream>
#include <QVariantList>
#include <QVariantMap>
#include <utility>

namespace {
constexpr auto kPermissionsService = "com.velyx.Permissions";
constexpr auto kPermissionsPath = "/com/velyx/Permissions";
constexpr auto kPermissionsInterface = "com.velyx.Permissions1";
constexpr auto kLauncherService = "com.velyx.Launcher";
constexpr auto kLauncherPath = "/com/velyx/Launcher";
constexpr auto kLauncherInterface = "com.velyx.Launcher1";
constexpr auto kSessionManagerService = "com.velyx.SessionManager";
constexpr auto kSessionManagerPath = "/com/velyx/SessionManager";
constexpr auto kSessionManagerInterface = "com.velyx.SessionManager1";

QString permissionDisplayName(const QString &permission)
{
    if (permission == "camera") {
        return "Доступ к камере";
    }
    if (permission == "microphone") {
        return "Доступ к микрофону";
    }
    if (permission == "filesystem") {
        return "Доступ к файлам";
    }
    if (permission == "screen_capture") {
        return "Захват экрана";
    }
    return permission;
}

QString runTool(const QString &program, const QStringList &arguments)
{
    QProcess process;
    process.start(program, arguments);
    if (!process.waitForStarted(400) || !process.waitForFinished(1500)) {
        return {};
    }
    return QString::fromUtf8(process.readAllStandardOutput()).trimmed();
}

bool runToolSucceeded(const QString &program, const QStringList &arguments)
{
    QProcess process;
    process.start(program, arguments);
    if (!process.waitForStarted(400) || !process.waitForFinished(1500)) {
        return false;
    }
    return process.exitStatus() == QProcess::NormalExit && process.exitCode() == 0;
}

QString extractWindowId(const QString &raw)
{
    static const QRegularExpression pattern("(0x[0-9a-fA-F]+)");
    const QRegularExpressionMatch match = pattern.match(raw);
    return match.hasMatch() ? match.captured(1) : QString();
}
}

PermissionClient::PermissionClient(QObject *parent)
    : QObject(parent)
{
}

QVariantList PermissionClient::apps() const
{
    return m_apps;
}

QVariantList PermissionClient::openApps() const
{
    return m_openApps;
}

QVariantMap PermissionClient::selectedAppInfo() const
{
    return m_selectedAppInfo;
}

QString PermissionClient::selectedAppId() const
{
    return m_selectedAppInfo.value("app_id").toString();
}

QString PermissionClient::activeAppId() const
{
    return m_activeAppId;
}

QString PermissionClient::activeAppTitle() const
{
    return m_activeAppTitle;
}

QString PermissionClient::activeWindowId() const
{
    return m_activeWindowId;
}

QString PermissionClient::activeWindowTitle() const
{
    return m_activeWindowTitle;
}

QString PermissionClient::activeRuntimeState() const
{
    return m_activeRuntimeState;
}

QString PermissionClient::inputControlMode() const
{
    return m_inputControlMode;
}

QString PermissionClient::shortcutFeedback() const
{
    return m_shortcutFeedback;
}

QString PermissionClient::launchStatus() const
{
    return m_launchStatus;
}

QString PermissionClient::launchResultMessage() const
{
    return m_launchResultMessage;
}

QString PermissionClient::lastAction() const
{
    return m_lastAction;
}

QString PermissionClient::lastResult() const
{
    return m_lastResult;
}

QString PermissionClient::lastReason() const
{
    return m_lastReason;
}

QString PermissionClient::nextAction() const
{
    return m_nextAction;
}

QString PermissionClient::launcherAvailability() const
{
    return m_launcherAvailability;
}

QString PermissionClient::permissionsAvailability() const
{
    return m_permissionsAvailability;
}

QString PermissionClient::sessionAvailability() const
{
    return m_sessionAvailability;
}

QString PermissionClient::sessionState() const
{
    return m_sessionState;
}

QString PermissionClient::sessionHealth() const
{
    return m_sessionHealth;
}

QVariantMap PermissionClient::cachedAppInfo(const QString &appId) const
{
    if (m_selectedAppInfo.value("app_id").toString() == appId) {
        return m_selectedAppInfo;
    }

    for (const QVariant &entry : m_apps) {
        const QVariantMap app = entry.toMap();
        if (app.value("app_id").toString() == appId) {
            return app;
        }
    }

    return {};
}

void PermissionClient::refreshApps()
{
    refreshRuntimeStatus();

    QDBusInterface launcher(kLauncherService, kLauncherPath, kLauncherInterface, QDBusConnection::sessionBus());
    if (!launcher.isValid()) {
        updateLaunchState("error", "launcher-service недоступен");
        updateStatusDetails("list_apps", "error", "launcher_unavailable", "retry");
        if (!m_apps.isEmpty()) {
            m_apps.clear();
            emit appsChanged();
        }
        return;
    }

    const QDBusMessage reply = launcher.call("ListApps");
    if (reply.type() == QDBusMessage::ErrorMessage || reply.arguments().isEmpty()) {
        updateLaunchState("error", "Не удалось получить список приложений");
        updateStatusDetails("list_apps", "error", "list_apps_failed", "retry");
        return;
    }

    const QVariantList items = reply.arguments().constFirst().toList();
    QVariantList apps;
    for (const QVariant &item : items) {
        QVariantMap map = item.toMap();
        const QVariantMap sessionApp = m_sessionApps.value(map.value("app_id").toString()).toMap();
        if (!sessionApp.isEmpty()) {
            map.insert("session_required", sessionApp.value("required"));
            map.insert("session_autostart", sessionApp.value("autostart"));
            map.insert("session_retry_count", sessionApp.value("retry_count"));
            map.insert("runtime_state", sessionApp.value("state", map.value("runtime_state")));
            map.insert("runtime_pid", sessionApp.value("pid", map.value("runtime_pid")));
        }
        if (!map.isEmpty()) {
            apps.push_back(map);
        }
    }

    if (m_apps != apps) {
        m_apps = apps;
        emit appsChanged();
    }
    updateStatusDetails("list_apps", "ok", QString("count=%1").arg(m_apps.size()), "select_app");

    if (!m_apps.isEmpty() && m_selectedAppInfo.isEmpty()) {
        selectApp(m_apps.constFirst().toMap().value("app_id").toString());
    }
}

void PermissionClient::refreshOpenApps()
{
    QDBusInterface launcher(kLauncherService, kLauncherPath, kLauncherInterface, QDBusConnection::sessionBus());
    if (!launcher.isValid()) {
        if (!m_openApps.isEmpty()) {
            m_openApps.clear();
            emit openAppsChanged();
        }
        reconcileActiveApp();
        return;
    }

    const QDBusMessage reply = launcher.call("ListRunningApps");
    if (reply.type() == QDBusMessage::ErrorMessage || reply.arguments().isEmpty()) {
        return;
    }

    const QString systemActiveWindowId = querySystemActiveWindowId();
    QVariantList openApps;
    QHash<QString, QString> nextWindowAuditState;
    for (const QVariant &item : reply.arguments().constFirst().toList()) {
        QVariantMap runtime = item.toMap();
        const QString appId = runtime.value("app_id").toString();
        QVariantMap info = cachedAppInfo(appId);
        if (info.isEmpty()) {
            info = fetchAppInfo(appId);
        }
        const QVariantMap sessionApp = m_sessionApps.value(appId).toMap();
        const QVariantMap window = discoverWindowForPid(runtime.value("pid").toString());
        runtime.insert("display_name", info.value("display_name", appId));
        runtime.insert("title", info.value("display_name", appId));
        runtime.insert("trust_level", info.value("trust_level"));
        runtime.insert("required", sessionApp.value("required", false));
        runtime.insert("autostart", sessionApp.value("autostart", false));
        runtime.insert("runtime_state", runtime.value("state"));
        runtime.insert("window_id", window.value("window_id"));
        runtime.insert("window_title", window.value("window_title", info.value("display_name", appId)));
        runtime.insert("window_geometry", window.value("geometry"));
        runtime.insert("is_visible", window.value("is_visible", false));
        runtime.insert("is_mapped", window.value("is_mapped", false));
        runtime.insert("has_window", window.value("has_window", false));
        runtime.insert(
            "window_state",
            window.value("has_window").toBool()
                ? (window.value("is_visible").toBool() ? "visible" : "hidden")
                : "no_window");
        runtime.insert(
            "window_active",
            !systemActiveWindowId.isEmpty() && window.value("window_id").toString() == systemActiveWindowId);
        runtime.insert("closable", true);
        runtime.insert("active", appId == m_activeAppId);
        const QString bindingState = QString("%1|%2|%3|%4")
                                         .arg(runtime.value("window_id").toString(),
                                              runtime.value("window_state").toString(),
                                              runtime.value("pid").toString(),
                                              runtime.value("state").toString());
        nextWindowAuditState.insert(appId, bindingState);
        if (m_windowAuditState.value(appId) != bindingState) {
            logShellEvent(
                "shell_window_discovery_begin",
                appId,
                QString("pid=%1 previous=%2").arg(
                    runtime.value("pid").toString(),
                    m_windowAuditState.value(appId)));
            if (runtime.value("has_window").toBool()) {
                logShellEvent(
                    "shell_window_discovered",
                    appId,
                    QString("pid=%1 window_id=%2 title=%3 state=%4")
                        .arg(runtime.value("pid").toString(),
                             runtime.value("window_id").toString(),
                             runtime.value("window_title").toString(),
                             runtime.value("window_state").toString()));
            } else {
                logShellEvent(
                    "shell_window_not_found",
                    appId,
                    QString("pid=%1 runtime_state=%2")
                        .arg(runtime.value("pid").toString(),
                             runtime.value("state").toString()));
            }
            logShellEvent("shell_window_binding_updated", appId, bindingState);
        }
        openApps.push_back(runtime);
    }

    m_windowAuditState = nextWindowAuditState;
    if (m_openApps != openApps) {
        m_openApps = openApps;
        emit openAppsChanged();
    }
    reconcileActiveApp();
}

void PermissionClient::refreshRuntimeStatus()
{
    bool changed = false;

    const QDBusInterface launcher(kLauncherService, kLauncherPath, kLauncherInterface, QDBusConnection::sessionBus());
    const QString launcherState = launcher.isValid() ? "available" : "unavailable";
    if (m_launcherAvailability != launcherState) {
        m_launcherAvailability = launcherState;
        changed = true;
    }

    const QDBusInterface permissions(
        kPermissionsService,
        kPermissionsPath,
        kPermissionsInterface,
        QDBusConnection::sessionBus());
    const QString permissionsState = permissions.isValid() ? "available" : "unavailable";
    if (m_permissionsAvailability != permissionsState) {
        m_permissionsAvailability = permissionsState;
        changed = true;
    }

    const QDBusInterface sessionManager(
        kSessionManagerService,
        kSessionManagerPath,
        kSessionManagerInterface,
        QDBusConnection::sessionBus());
    const QString sessionAvailability = sessionManager.isValid() ? "available" : "unavailable";
    if (m_sessionAvailability != sessionAvailability) {
        m_sessionAvailability = sessionAvailability;
        changed = true;
    }

    QString sessionState = "unknown";
    QString sessionHealth = "unknown";
    QVariantMap sessionApps;
    if (sessionManager.isValid()) {
        QDBusReply<QVariantMap> reply = sessionManager.call("GetSessionState");
        if (reply.isValid()) {
            const QVariantMap payload = reply.value();
            sessionState = payload.value("state", "unknown").toString();
            sessionHealth = payload.value("health", "unknown").toString();
            const QString appsStatus = payload.value("apps_status").toString();
            for (const QString &entry : appsStatus.split('|', Qt::SkipEmptyParts)) {
                const QStringList parts = entry.split(':');
                if (parts.size() < 6) {
                    continue;
                }
                QVariantMap app;
                app.insert("state", parts.value(1));
                app.insert("pid", parts.value(2));
                app.insert("required", parts.value(3));
                app.insert("autostart", parts.value(4));
                app.insert("retry_count", parts.value(5));
                sessionApps.insert(parts.value(0), app);
            }
        } else {
            sessionState = "error";
            sessionHealth = "error";
        }
    }

    if (m_sessionState != sessionState) {
        m_sessionState = sessionState;
        changed = true;
    }
    if (m_sessionHealth != sessionHealth) {
        m_sessionHealth = sessionHealth;
        changed = true;
    }
    if (m_sessionApps != sessionApps) {
        m_sessionApps = sessionApps;
        changed = true;
    }

    if (changed) {
        emit runtimeStatusChanged();
    }
}

QVariantMap PermissionClient::fetchAppInfo(const QString &appId)
{
    QDBusInterface launcher(kLauncherService, kLauncherPath, kLauncherInterface, QDBusConnection::sessionBus());
    if (!launcher.isValid()) {
        return {};
    }

    QDBusReply<QVariantMap> reply = launcher.call("GetAppInfo", appId);
    if (!reply.isValid()) {
        return {};
    }
    return reply.value();
}

QVariantMap PermissionClient::fetchAppRuntime(const QString &appId)
{
    QDBusInterface launcher(kLauncherService, kLauncherPath, kLauncherInterface, QDBusConnection::sessionBus());
    if (!launcher.isValid()) {
        return {};
    }

    QDBusReply<QVariantMap> reply = launcher.call("GetAppRuntime", appId);
    if (!reply.isValid()) {
        return {};
    }
    return reply.value();
}

void PermissionClient::selectApp(const QString &appId)
{
    QVariantMap info = cachedAppInfo(appId);
    if (info.isEmpty()) {
        info = fetchAppInfo(appId);
    }
    if (info.isEmpty()) {
        updateLaunchState("error", "Не удалось получить информацию о приложении");
        updateStatusDetails("get_app_info", "error", "app_info_unavailable", "retry");
        return;
    }

    m_selectedAppInfo = info;
    m_pendingAppId = info.value("app_id").toString();
    m_pendingAppName = info.value("display_name", m_pendingAppId).toString();
    m_pendingPermission = info.value("requested_permissions").toString().split(',', Qt::SkipEmptyParts).value(0);
    emit selectedAppInfoChanged();
    refreshSelectedAppRuntime();
    updateStatusDetails("get_app_info", "ok", "app_selected", "launch");
}

void PermissionClient::selectActiveApp(const QString &appId)
{
    updateActiveApp(appId, true);
    selectApp(appId);
    for (const QVariant &entry : std::as_const(m_openApps)) {
        const QVariantMap app = entry.toMap();
        if (app.value("app_id").toString() == appId) {
            const QString windowId = app.value("window_id").toString();
            if (!windowId.isEmpty()) {
                logShellEvent("shell_window_activate_requested", appId, QString("window_id=%1").arg(windowId));
                if (focusWindow(windowId)) {
                    logShellEvent("shell_window_activate_ok", appId, QString("window_id=%1").arg(windowId));
                } else {
                    logShellEvent("shell_window_activate_failed", appId, QString("window_id=%1").arg(windowId));
                }
            } else {
                logShellEvent("shell_window_activate_failed", appId, "window_id=missing");
            }
            break;
        }
    }
}

void PermissionClient::refreshSelectedAppRuntime()
{
    if (m_selectedAppInfo.isEmpty()) {
        return;
    }

    const QString appId = m_selectedAppInfo.value("app_id").toString();
    const QVariantMap runtime = fetchAppRuntime(appId);
    if (runtime.isEmpty()) {
        return;
    }

    QVariantMap updated = fetchAppInfo(appId);
    if (updated.isEmpty()) {
        updated = m_selectedAppInfo;
    }
    const QVariantMap sessionApp = m_sessionApps.value(appId).toMap();
    if (!sessionApp.isEmpty()) {
        updated.insert("session_required", sessionApp.value("required"));
        updated.insert("session_autostart", sessionApp.value("autostart"));
        updated.insert("session_retry_count", sessionApp.value("retry_count"));
    }
    updated.insert("runtime_state", runtime.value("state"));
    updated.insert("runtime_pid", runtime.value("pid"));
    updated.insert("runtime_launch_status", runtime.value("launch_status"));
    updated.insert("runtime_exited_at", runtime.value("exited_at"));
    updated.insert("runtime_exit_code", runtime.value("exit_code"));
    updated.insert("runtime_failure_reason", runtime.value("failure_reason"));
    const QVariantMap window = discoverWindowForPid(runtime.value("pid").toString());
    updated.insert("window_id", window.value("window_id"));
    updated.insert("window_title", window.value("window_title"));
    updated.insert("window_geometry", window.value("geometry"));
    updated.insert("window_visible", window.value("is_visible", false));
    updated.insert("window_mapped", window.value("is_mapped", false));
    updated.insert("window_active", window.value("window_id").toString() == querySystemActiveWindowId());
    if (m_selectedAppInfo != updated) {
        m_selectedAppInfo = updated;
        emit selectedAppInfoChanged();
    }
    refreshOpenApps();
}

void PermissionClient::launchSelectedApp()
{
    if (m_selectedAppInfo.isEmpty()) {
        updateLaunchState("error", "Сначала выберите приложение");
        updateStatusDetails("launch", "error", "no_app_selected", "select_app");
        return;
    }

    startLaunch(
        m_selectedAppInfo.value("app_id").toString(),
        m_selectedAppInfo.value("display_name").toString(),
        m_selectedAppInfo.value("requested_permissions").toString().split(',', Qt::SkipEmptyParts).value(0));
}

void PermissionClient::startLaunch(
    const QString &appId,
    const QString &appName,
    const QString &permission)
{
    m_pendingAppId = appId;
    m_pendingAppName = appName;
    m_pendingPermission = permission;

    requestLaunchFromLauncher(appId, appName, permission);
}

void PermissionClient::stopSelectedApp()
{
    if (m_selectedAppInfo.isEmpty()) {
        updateLaunchState("error", "Сначала выберите приложение");
        updateStatusDetails("stop", "error", "no_app_selected", "select_app");
        return;
    }

    const QString appId = m_selectedAppInfo.value("app_id").toString();
    QDBusInterface launcher(kLauncherService, kLauncherPath, kLauncherInterface, QDBusConnection::sessionBus());
    if (!launcher.isValid()) {
        updateLaunchState("error", "launcher-service недоступен");
        updateStatusDetails("stop", "error", "launcher_unavailable", "retry");
        refreshRuntimeStatus();
        return;
    }

    QDBusReply<QVariantMap> reply = launcher.call("StopApp", appId);
    if (!reply.isValid()) {
        updateLaunchState("error", "Не удалось остановить приложение");
        updateStatusDetails("stop", "error", "stop_call_failed", "retry");
        return;
    }

    const QVariantMap payload = reply.value();
    updateLaunchState(payload.value("status").toString(), payload.value("message").toString());
    updateStatusDetails(
        "process_stop",
        payload.value("status").toString(),
        payload.value("reason").toString(),
        payload.value("next_action").toString());
    refreshSelectedAppRuntime();
    refreshApps();
}

void PermissionClient::restartSelectedApp()
{
    if (m_selectedAppInfo.isEmpty()) {
        updateLaunchState("error", "Сначала выберите приложение");
        updateStatusDetails("restart", "error", "no_app_selected", "select_app");
        return;
    }

    const QString appId = m_selectedAppInfo.value("app_id").toString();
    QDBusInterface launcher(kLauncherService, kLauncherPath, kLauncherInterface, QDBusConnection::sessionBus());
    if (!launcher.isValid()) {
        updateLaunchState("error", "launcher-service недоступен");
        updateStatusDetails("restart", "error", "launcher_unavailable", "retry");
        refreshRuntimeStatus();
        return;
    }

    QDBusReply<QVariantMap> reply = launcher.call("RestartApp", appId);
    if (!reply.isValid()) {
        updateLaunchState("error", "Не удалось перезапустить приложение");
        updateStatusDetails("restart", "error", "restart_call_failed", "retry");
        return;
    }

    const QVariantMap payload = reply.value();
    const QString status = payload.value("status").toString();
    updateLaunchState(status, payload.value("message").toString());
    updateStatusDetails(
        "process_restart",
        status,
        payload.value("reason").toString(),
        payload.value("next_action").toString());
    refreshSelectedAppRuntime();
    refreshApps();
}

void PermissionClient::closeOpenApp(const QString &appId)
{
    if (appId.isEmpty()) {
        return;
    }
    selectApp(appId);
    logShellEvent("shell_window_closed", appId, "close requested from open apps list");
    stopSelectedApp();
}

void PermissionClient::restartOpenApp(const QString &appId)
{
    if (appId.isEmpty()) {
        return;
    }
    selectApp(appId);
    updateActiveApp(appId, true);
    logShellEvent("shell_window_restart_requested", appId, "restart requested from open apps list");
    restartSelectedApp();
}

void PermissionClient::activateNextApp()
{
    logShellEvent("shell_input_event", "", "shortcut=Alt+Tab");
    if (m_openApps.isEmpty()) {
        m_shortcutFeedback = "Alt+Tab: нет запущенных приложений";
        emit inputStatusChanged();
        logShellEvent("shell_shortcut_failed", "", "shortcut=Alt+Tab reason=no_running_apps");
        return;
    }

    int currentIndex = -1;
    for (int index = 0; index < m_openApps.size(); ++index) {
        if (m_openApps[index].toMap().value("app_id").toString() == m_activeAppId) {
            currentIndex = index;
            break;
        }
    }

    const int nextIndex = (currentIndex + 1 + m_openApps.size()) % m_openApps.size();
    const QString appId = m_openApps[nextIndex].toMap().value("app_id").toString();
    m_shortcutFeedback = QString("Alt+Tab -> %1").arg(appId);
    emit inputStatusChanged();
    logShellEvent("shell_shortcut_triggered", appId, "shortcut=Alt+Tab");
    logShellEvent("shell_active_switch", appId, "source=shortcut");
    selectActiveApp(appId);
}

void PermissionClient::closeActiveApp()
{
    logShellEvent("shell_input_event", "", "shortcut=Alt+Q");
    if (m_activeAppId.isEmpty()) {
        m_shortcutFeedback = "Alt+Q: нет активного приложения";
        emit inputStatusChanged();
        logShellEvent("shell_shortcut_failed", "", "shortcut=Alt+Q reason=no_active_app");
        return;
    }

    m_shortcutFeedback = QString("Alt+Q -> close %1").arg(m_activeAppId);
    emit inputStatusChanged();
    logShellEvent("shell_shortcut_triggered", m_activeAppId, "shortcut=Alt+Q");
    closeOpenApp(m_activeAppId);
}

void PermissionClient::restartActiveInstance()
{
    logShellEvent("shell_input_event", "", "shortcut=Alt+R");
    if (m_activeAppId.isEmpty()) {
        m_shortcutFeedback = "Alt+R: нет активного приложения";
        emit inputStatusChanged();
        logShellEvent("shell_shortcut_failed", "", "shortcut=Alt+R reason=no_active_app");
        return;
    }

    m_shortcutFeedback = QString("Alt+R -> restart %1").arg(m_activeAppId);
    emit inputStatusChanged();
    logShellEvent("shell_shortcut_triggered", m_activeAppId, "shortcut=Alt+R");
    restartOpenApp(m_activeAppId);
}

void PermissionClient::activateAppByIndex(int index)
{
    logShellEvent("shell_input_event", "", QString("shortcut=Alt+%1").arg(index + 1));
    if (index < 0 || index >= m_openApps.size()) {
        m_shortcutFeedback = QString("Alt+%1: приложение недоступно").arg(index + 1);
        emit inputStatusChanged();
        logShellEvent("shell_shortcut_failed", "", QString("shortcut=Alt+%1 reason=index_out_of_range").arg(index + 1));
        return;
    }

    const QString appId = m_openApps[index].toMap().value("app_id").toString();
    m_shortcutFeedback = QString("Alt+%1 -> %2").arg(index + 1).arg(appId);
    emit inputStatusChanged();
    logShellEvent("shell_shortcut_triggered", appId, QString("shortcut=Alt+%1").arg(index + 1));
    logShellEvent("shell_active_switch", appId, "source=shortcut_index");
    selectActiveApp(appId);
}

void PermissionClient::setInputControlMode(const QString &mode, const QString &details)
{
    bool changed = false;
    if (m_inputControlMode != mode) {
        m_inputControlMode = mode;
        changed = true;
    }
    if (m_shortcutFeedback != details) {
        m_shortcutFeedback = details;
        changed = true;
    }
    if (changed) {
        emit inputStatusChanged();
    }
    logShellEvent("shell_input_event", "", QString("mode=%1 details=%2").arg(mode, details));
}

void PermissionClient::requestLaunchFromLauncher(
    const QString &appId,
    const QString &appName,
    const QString &permission)
{
    updateStatusDetails("launch_requested", "pending", "", "await_result");
    QDBusInterface launcher(kLauncherService, kLauncherPath, kLauncherInterface, QDBusConnection::sessionBus());
    if (!launcher.isValid()) {
        updateLaunchState("error", "launcher-service недоступен");
        updateStatusDetails("launch_requested", "error", "launcher_unavailable", "retry");
        refreshRuntimeStatus();
        return;
    }

    QDBusReply<QVariantMap> launchReply = launcher.call("Launch", appId);
    if (!launchReply.isValid()) {
        updateLaunchState("error", "Не удалось выполнить запрос к secure launcher");
        updateStatusDetails("launch_requested", "error", "launch_call_failed", "retry");
        refreshRuntimeStatus();
        return;
    }

    const QVariantMap payload = launchReply.value();
    const QString status = payload.value("status").toString();
    const QString reason = payload.value("reason").toString();
    const QString nextAction = payload.value("next_action").toString();
    if (status == "launched" || status == "started" || status == "already_running") {
        updateStatusDetails("launch_allowed", status, reason, nextAction);
        const QString message = payload.value("pid").toString().isEmpty()
            ? payload.value("message").toString()
            : QString("%1 (pid=%2)")
                  .arg(payload.value("message").toString(), payload.value("pid").toString());
        handleAllowedLaunch(message);
        updateActiveApp(appId, false);
        logShellEvent("shell_window_opened", appId, QString("status=%1 pid=%2").arg(status, payload.value("pid").toString()));
        refreshSelectedAppRuntime();
        refreshApps();
        return;
    }

    if (status == "deny") {
        updateStatusDetails("launch_denied", status, reason, nextAction);
        handleDeniedLaunch(appName, permissionDisplayName(permission));
        return;
    }

    if (status == "failed"
        || status == "sandbox_failed"
        || status == "security_failed"
        || status == "manifest_invalid"
        || status == "executable_invalid"
        || status == "profile_invalid") {
        updateLaunchState(status, payload.value("message").toString());
        updateStatusDetails("launch_failed", status, reason, nextAction);
        refreshSelectedAppRuntime();
        return;
    }

    if (status == "prompt") {
        updateStatusDetails("launch_prompted", status, reason, nextAction);
        emit permissionPromptRequired(
            payload.value("app_id", appId).toString(),
            appName,
            payload.value("required_permission", payload.value("permission", permission)).toString(),
            payload.value("permission_display", permissionDisplayName(payload.value("required_permission", payload.value("permission", permission)).toString())).toString(),
            payload.value("explanation").toString());
        return;
    }

    updateLaunchState("error", payload.value("message", "Secure launcher вернул неизвестный статус").toString());
    updateStatusDetails("launch_requested", "error", reason, nextAction);
    refreshSelectedAppRuntime();
}

void PermissionClient::submitDecision(
    const QString &appId,
    const QString &appName,
    const QString &permission,
    bool allowed)
{
    QDBusInterface permissions(
        kPermissionsService,
        kPermissionsPath,
        kPermissionsInterface,
        QDBusConnection::sessionBus());
    if (!permissions.isValid()) {
        updateLaunchState("error", "permissions-service недоступен");
        updateStatusDetails("store_decision", "error", "permissions_unavailable", "retry");
        refreshRuntimeStatus();
        return;
    }

    const QString decision = allowed ? "allow" : "deny";
    QDBusReply<bool> storeReply = permissions.call("StoreDecision", appId, permission, decision);
    if (!storeReply.isValid() || !storeReply.value()) {
        updateLaunchState("error", "Не удалось сохранить решение по разрешению");
        updateStatusDetails("store_decision", "error", "store_decision_failed", "retry");
        return;
    }

    if (allowed) {
        updateStatusDetails("permission_granted", "allow", permission, "retry_launch");
        requestLaunchFromLauncher(
            m_pendingAppId.isEmpty() ? appId : m_pendingAppId,
            m_pendingAppName.isEmpty() ? appName : m_pendingAppName,
            m_pendingPermission.isEmpty() ? permission : m_pendingPermission);
    } else {
        updateStatusDetails("permission_denied", "deny", permission, "none");
        handleDeniedLaunch(appName, permissionDisplayName(permission));
    }
}

void PermissionClient::resetPermissions(const QString &appId)
{
    QDBusInterface permissions(
        kPermissionsService,
        kPermissionsPath,
        kPermissionsInterface,
        QDBusConnection::sessionBus());
    if (!permissions.isValid()) {
        updateLaunchState("error", "permissions-service недоступен");
        updateStatusDetails("reset_permissions", "error", "permissions_unavailable", "retry");
        refreshRuntimeStatus();
        return;
    }

    QDBusReply<uint> resetReply = permissions.call("ResetPermissions", appId);
    if (!resetReply.isValid()) {
        updateLaunchState("error", "Не удалось сбросить разрешения");
        updateStatusDetails("reset_permissions", "error", "reset_failed", "retry");
        return;
    }

    updateLaunchState("idle", QString("Разрешения для %1 сброшены: %2").arg(appId).arg(resetReply.value()));
    updateStatusDetails("reset_permissions", "ok", "permissions_reset", "launch");
}

void PermissionClient::updateLaunchState(const QString &status, const QString &message)
{
    if (m_launchStatus != status) {
        m_launchStatus = status;
        emit launchStatusChanged();
    }

    if (m_launchResultMessage != message) {
        m_launchResultMessage = message;
        emit launchResultMessageChanged();
    }
}

void PermissionClient::handleAllowedLaunch(const QString &message)
{
    updateLaunchState("launched", message);
}

void PermissionClient::updateStatusDetails(
    const QString &action,
    const QString &result,
    const QString &reason,
    const QString &nextAction)
{
    bool changed = false;
    if (m_lastAction != action) {
        m_lastAction = action;
        changed = true;
    }
    if (m_lastResult != result) {
        m_lastResult = result;
        changed = true;
    }
    if (m_lastReason != reason) {
        m_lastReason = reason;
        changed = true;
    }
    if (m_nextAction != nextAction) {
        m_nextAction = nextAction;
        changed = true;
    }
    if (changed) {
        emit statusDetailsChanged();
    }
}

void PermissionClient::handleDeniedLaunch(const QString &appName, const QString &permissionDisplayName)
{
    updateLaunchState("denied", QString("%1 не запущено. Доступ запрещен: %2")
                                    .arg(appName, permissionDisplayName));
}

void PermissionClient::updateActiveApp(const QString &appId, bool userInitiated)
{
    if (m_activeAppId == appId) {
        return;
    }
    m_activeAppId = appId;
    m_activeAppTitle.clear();
    m_activeWindowId.clear();
    m_activeWindowTitle.clear();
    m_activeRuntimeState.clear();
    for (const QVariant &entry : std::as_const(m_openApps)) {
        const QVariantMap app = entry.toMap();
        if (app.value("app_id").toString() == appId) {
            m_activeAppTitle = app.value("display_name", appId).toString();
            m_activeWindowId = app.value("window_id").toString();
            m_activeWindowTitle = app.value("window_title").toString();
            m_activeRuntimeState = app.value("state").toString();
            break;
        }
    }
    if (m_activeAppTitle.isEmpty() && m_selectedAppInfo.value("app_id").toString() == appId) {
        m_activeAppTitle = m_selectedAppInfo.value("display_name", appId).toString();
        m_activeWindowId = m_selectedAppInfo.value("window_id").toString();
        m_activeWindowTitle = m_selectedAppInfo.value("window_title").toString();
        m_activeRuntimeState = m_selectedAppInfo.value("runtime_state").toString();
    }
    for (int index = 0; index < m_openApps.size(); ++index) {
        QVariantMap app = m_openApps[index].toMap();
        app.insert("active", app.value("app_id").toString() == appId);
        m_openApps[index] = app;
    }
    emit openAppsChanged();
    emit activeAppChanged();
    if (!appId.isEmpty()) {
        logShellEvent(
            "shell_active_app_changed",
            appId,
            userInitiated ? "user_selected=true" : "user_selected=false");
    }
}

void PermissionClient::reconcileActiveApp()
{
    const QString systemActiveWindowId = querySystemActiveWindowId();
    if (!systemActiveWindowId.isEmpty()) {
        for (const QVariant &entry : std::as_const(m_openApps)) {
            const QVariantMap app = entry.toMap();
            if (app.value("window_id").toString() == systemActiveWindowId) {
                if (m_activeAppId != app.value("app_id").toString()) {
                    updateActiveApp(app.value("app_id").toString(), false);
                    logShellEvent(
                        "shell_window_focus_synced",
                        app.value("app_id").toString(),
                        QString("window_id=%1 source=system_active_window").arg(systemActiveWindowId));
                }
                break;
            }
        }
    }

    QString nextActiveId = m_activeAppId;
    bool foundActiveRunning = false;
    for (int index = 0; index < m_openApps.size(); ++index) {
        QVariantMap app = m_openApps[index].toMap();
        const bool isActive = app.value("app_id").toString() == m_activeAppId;
        const bool shouldBeActive = isActive && !m_activeAppId.isEmpty();
        app.insert("active", shouldBeActive);
        m_openApps[index] = app;
        if (shouldBeActive) {
            foundActiveRunning = true;
        }
    }

    if (!foundActiveRunning) {
        nextActiveId.clear();
        for (const QVariant &entry : std::as_const(m_openApps)) {
            const QVariantMap app = entry.toMap();
            if (app.value("state").toString() == "running") {
                nextActiveId = app.value("app_id").toString();
                break;
            }
        }
        if (nextActiveId != m_activeAppId) {
            m_activeAppId = nextActiveId;
            m_activeAppTitle.clear();
            m_activeWindowId.clear();
            m_activeWindowTitle.clear();
            m_activeRuntimeState.clear();
            for (int index = 0; index < m_openApps.size(); ++index) {
                QVariantMap app = m_openApps[index].toMap();
                const bool isActive = app.value("app_id").toString() == nextActiveId;
                app.insert("active", isActive);
                if (isActive) {
                    m_activeAppTitle = app.value("display_name", nextActiveId).toString();
                    m_activeWindowId = app.value("window_id").toString();
                    m_activeWindowTitle = app.value("window_title").toString();
                    m_activeRuntimeState = app.value("state").toString();
                }
                m_openApps[index] = app;
            }
            if (nextActiveId.isEmpty()) {
                m_activeWindowId.clear();
                m_activeWindowTitle.clear();
                m_activeRuntimeState.clear();
            }
            emit openAppsChanged();
            emit activeAppChanged();
            logShellEvent(
                "shell_active_app_changed",
                nextActiveId,
                nextActiveId.isEmpty() ? "active_cleared=true" : "auto_selected=true");
        }
    }

}

QVariantMap PermissionClient::discoverWindowForPid(const QString &pid) const
{
    if (pid.isEmpty()) {
        return {};
    }

    const QString rootDump = runTool("xprop", {"-root", "_NET_CLIENT_LIST"});
    if (rootDump.isEmpty()) {
        return {};
    }

    const QStringList windowIds = rootDump.split(QRegularExpression("[,\\s]+"), Qt::SkipEmptyParts);
    for (const QString &token : windowIds) {
        const QString windowId = extractWindowId(token);
        if (windowId.isEmpty()) {
            continue;
        }

        const QString pidDump = runTool("xprop", {"-id", windowId, "_NET_WM_PID"});
        if (!pidDump.contains(pid)) {
            continue;
        }

        QVariantMap window;
        const QString titleDump = runTool("xprop", {"-id", windowId, "_NET_WM_NAME", "WM_NAME"});
        const QString mapDump = runTool("xwininfo", {"-id", windowId});
        const QString geometryDump = runTool("xwininfo", {"-id", windowId, "-stats"});
        window.insert("window_id", windowId);
        window.insert("has_window", true);
        window.insert("window_title", titleDump.section('=', 1).trimmed().remove('"'));
        window.insert("is_visible", mapDump.contains("Map State: IsViewable"));
        window.insert("is_mapped", !mapDump.contains("Map State: IsUnMapped"));

        QString geometry;
        const QString width = geometryDump.contains("Width:")
            ? geometryDump.section("Width:", 1).section('\n', 0, 0).trimmed()
            : QString();
        const QString height = geometryDump.contains("Height:")
            ? geometryDump.section("Height:", 1).section('\n', 0, 0).trimmed()
            : QString();
        const QString absX = geometryDump.contains("Absolute upper-left X:")
            ? geometryDump.section("Absolute upper-left X:", 1).section('\n', 0, 0).trimmed()
            : QString();
        const QString absY = geometryDump.contains("Absolute upper-left Y:")
            ? geometryDump.section("Absolute upper-left Y:", 1).section('\n', 0, 0).trimmed()
            : QString();
        if (!width.isEmpty() && !height.isEmpty()) {
            geometry = QString("%1x%2").arg(width, height);
            if (!absX.isEmpty() && !absY.isEmpty()) {
                geometry += QString(" @ %1,%2").arg(absX, absY);
            }
        }
        window.insert("geometry", geometry);
        return window;
    }

    return {};
}

QString PermissionClient::querySystemActiveWindowId() const
{
    return extractWindowId(runTool("xprop", {"-root", "_NET_ACTIVE_WINDOW"}));
}

bool PermissionClient::focusWindow(const QString &windowId) const
{
    if (windowId.isEmpty()) {
        return false;
    }

    if (runToolSucceeded("wmctrl", {"-ia", windowId})) {
        return true;
    }

    return runToolSucceeded("xdotool", {"windowactivate", windowId});
}

void PermissionClient::logShellEvent(const QString &action, const QString &appId, const QString &details)
{
    const QString home = QStandardPaths::writableLocation(QStandardPaths::HomeLocation);
    const QString dirPath = QDir(home).filePath(".velyx");
    QDir().mkpath(dirPath);
    QFile file(QDir(dirPath).filePath("shell_mvp.log"));
    if (!file.open(QIODevice::WriteOnly | QIODevice::Append | QIODevice::Text)) {
        return;
    }
    QTextStream stream(&file);
    stream << QDateTime::currentDateTimeUtc().toString(Qt::ISODate)
           << " action=" << action
           << " app_id=" << appId
           << " details=" << details
           << "\n";
}
