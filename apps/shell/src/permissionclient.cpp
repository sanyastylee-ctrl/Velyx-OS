#include "permissionclient.h"

#include <QDBusConnection>
#include <QDBusInterface>
#include <QDBusReply>
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

QString PermissionClient::launchStatus() const
{
    return m_launchStatus;
}

QString PermissionClient::launchResultMessage() const
{
    return m_launchResultMessage;
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
    QDBusInterface launcher(kLauncherService, kLauncherPath, kLauncherInterface, QDBusConnection::sessionBus());
    if (!launcher.isValid()) {
        updateLaunchState("error", "launcher-service недоступен");
        return;
    }

    QDBusReply<QVariantMap> launchReply = launcher.call("Launch", appId);
    if (!launchReply.isValid()) {
        updateLaunchState("error", "Не удалось выполнить запрос к secure launcher");
        return;
    }

    const QVariantMap payload = launchReply.value();
    const QString status = payload.value("status").toString();
    if (status == "started") {
        handleAllowedLaunch(payload.value("message").toString());
        return;
    }

    if (status == "deny") {
        handleDeniedLaunch(appName, permissionDisplayName(permission));
        return;
    }

    if (status == "prompt") {
        emit permissionPromptRequired(
            payload.value("app_id", appId).toString(),
            appName,
            payload.value("permission", permission).toString(),
            payload.value("permission_display", permission).toString(),
            payload.value("explanation").toString());
        return;
    }

    updateLaunchState("error", payload.value("message", "Secure launcher вернул неизвестный статус").toString());
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
        return;
    }

    const QString decision = allowed ? "allow" : "deny";
    QDBusReply<bool> storeReply = permissions.call("StoreDecision", appId, permission, decision);
    if (!storeReply.isValid() || !storeReply.value()) {
        updateLaunchState("error", "Не удалось сохранить решение по разрешению");
        return;
    }

    if (allowed) {
        requestLaunchFromLauncher(
            m_pendingAppId.isEmpty() ? appId : m_pendingAppId,
            m_pendingAppName.isEmpty() ? appName : m_pendingAppName,
            m_pendingPermission.isEmpty() ? permission : m_pendingPermission);
    } else {
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
        return;
    }

    QDBusReply<uint> resetReply = permissions.call("ResetPermissions", appId);
    if (!resetReply.isValid()) {
        updateLaunchState("error", "Не удалось сбросить разрешения");
        return;
    }

    updateLaunchState("idle", QString("Разрешения для %1 сброшены: %2").arg(appId).arg(resetReply.value()));
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

void PermissionClient::handleDeniedLaunch(const QString &appName, const QString &permissionDisplayName)
{
    updateLaunchState("denied", QString("%1 не запущено. Доступ запрещен: %2")
                                    .arg(appName, permissionDisplayName));
}
