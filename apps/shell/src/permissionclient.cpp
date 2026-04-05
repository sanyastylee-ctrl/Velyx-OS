#include "permissionclient.h"

#include <QDBusConnection>
#include <QDBusInterface>
#include <QDBusMessage>
#include <QDBusReply>
#include <QVariantList>
#include <QVariantMap>

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
}

PermissionClient::PermissionClient(QObject *parent)
    : QObject(parent)
{
}

QVariantList PermissionClient::apps() const
{
    return m_apps;
}

QVariantMap PermissionClient::selectedAppInfo() const
{
    return m_selectedAppInfo;
}

QString PermissionClient::selectedAppId() const
{
    return m_selectedAppInfo.value("app_id").toString();
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

    m_apps = apps;
    emit appsChanged();
    updateStatusDetails("list_apps", "ok", QString("count=%1").arg(m_apps.size()), "select_app");

    if (!m_apps.isEmpty() && m_selectedAppInfo.isEmpty()) {
        selectApp(m_apps.constFirst().toMap().value("app_id").toString());
    }
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
    const QVariantMap info = fetchAppInfo(appId);
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
    m_selectedAppInfo = updated;
    emit selectedAppInfoChanged();
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
        refreshSelectedAppRuntime();
        refreshApps();
        return;
    }

    if (status == "deny") {
        updateStatusDetails("launch_denied", status, reason, nextAction);
        handleDeniedLaunch(appName, permissionDisplayName(permission));
        return;
    }

    if (status == "failed") {
        updateLaunchState("error", payload.value("message").toString());
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
    updateLaunchState("allowed", message);
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
