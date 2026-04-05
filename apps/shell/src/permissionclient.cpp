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

void PermissionClient::refreshApps()
{
    QDBusInterface launcher(kLauncherService, kLauncherPath, kLauncherInterface, QDBusConnection::sessionBus());
    if (!launcher.isValid()) {
        updateLaunchState("error", "launcher-service недоступен");
        updateStatusDetails("list_apps", "error", "launcher_unavailable", "retry");
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
        const QVariantMap map = item.toMap();
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
    updateStatusDetails("get_app_info", "ok", "app_selected", "launch");
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
        return;
    }

    QDBusReply<QVariantMap> launchReply = launcher.call("Launch", appId);
    if (!launchReply.isValid()) {
        updateLaunchState("error", "Не удалось выполнить запрос к secure launcher");
        updateStatusDetails("launch_requested", "error", "launch_call_failed", "retry");
        return;
    }

    const QVariantMap payload = launchReply.value();
    const QString status = payload.value("status").toString();
    const QString reason = payload.value("reason").toString();
    const QString nextAction = payload.value("next_action").toString();
    if (status == "launched" || status == "started") {
        updateStatusDetails("launch_allowed", status, reason, nextAction);
        const QString message = payload.value("pid").toString().isEmpty()
            ? payload.value("message").toString()
            : QString("%1 (pid=%2)")
                  .arg(payload.value("message").toString(), payload.value("pid").toString());
        handleAllowedLaunch(message);
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
