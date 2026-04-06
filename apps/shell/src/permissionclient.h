#pragma once

#include <QObject>
#include <QString>
#include <QVariantList>
#include <QVariantMap>

class PermissionClient : public QObject
{
    Q_OBJECT
    Q_PROPERTY(QVariantList apps READ apps NOTIFY appsChanged)
    Q_PROPERTY(QVariantList openApps READ openApps NOTIFY openAppsChanged)
    Q_PROPERTY(QVariantMap selectedAppInfo READ selectedAppInfo NOTIFY selectedAppInfoChanged)
    Q_PROPERTY(QString selectedAppId READ selectedAppId NOTIFY selectedAppInfoChanged)
    Q_PROPERTY(QString activeAppId READ activeAppId NOTIFY activeAppChanged)
    Q_PROPERTY(QString activeAppTitle READ activeAppTitle NOTIFY activeAppChanged)
    Q_PROPERTY(QString activeWindowId READ activeWindowId NOTIFY activeAppChanged)
    Q_PROPERTY(QString activeWindowTitle READ activeWindowTitle NOTIFY activeAppChanged)
    Q_PROPERTY(QString activeRuntimeState READ activeRuntimeState NOTIFY activeAppChanged)
    Q_PROPERTY(QString launchStatus READ launchStatus NOTIFY launchStatusChanged)
    Q_PROPERTY(QString launchResultMessage READ launchResultMessage NOTIFY launchResultMessageChanged)
    Q_PROPERTY(QString lastAction READ lastAction NOTIFY statusDetailsChanged)
    Q_PROPERTY(QString lastResult READ lastResult NOTIFY statusDetailsChanged)
    Q_PROPERTY(QString lastReason READ lastReason NOTIFY statusDetailsChanged)
    Q_PROPERTY(QString nextAction READ nextAction NOTIFY statusDetailsChanged)
    Q_PROPERTY(QString launcherAvailability READ launcherAvailability NOTIFY runtimeStatusChanged)
    Q_PROPERTY(QString permissionsAvailability READ permissionsAvailability NOTIFY runtimeStatusChanged)
    Q_PROPERTY(QString sessionAvailability READ sessionAvailability NOTIFY runtimeStatusChanged)
    Q_PROPERTY(QString sessionState READ sessionState NOTIFY runtimeStatusChanged)
    Q_PROPERTY(QString sessionHealth READ sessionHealth NOTIFY runtimeStatusChanged)

public:
    explicit PermissionClient(QObject *parent = nullptr);

    Q_INVOKABLE void refreshApps();
    Q_INVOKABLE void refreshOpenApps();
    Q_INVOKABLE void refreshRuntimeStatus();
    Q_INVOKABLE void selectApp(const QString &appId);
    Q_INVOKABLE void selectActiveApp(const QString &appId);
    Q_INVOKABLE void refreshSelectedAppRuntime();
    Q_INVOKABLE void launchSelectedApp();
    Q_INVOKABLE void stopSelectedApp();
    Q_INVOKABLE void restartSelectedApp();
    Q_INVOKABLE void closeOpenApp(const QString &appId);
    Q_INVOKABLE void restartOpenApp(const QString &appId);
    Q_INVOKABLE void startLaunch(
        const QString &appId,
        const QString &appName,
        const QString &permission);
    Q_INVOKABLE void submitDecision(const QString &appId, const QString &appName, const QString &permission, bool allowed);
    Q_INVOKABLE void resetPermissions(const QString &appId);

    QVariantList apps() const;
    QVariantList openApps() const;
    QVariantMap selectedAppInfo() const;
    QString selectedAppId() const;
    QString activeAppId() const;
    QString activeAppTitle() const;
    QString activeWindowId() const;
    QString activeWindowTitle() const;
    QString activeRuntimeState() const;
    QString launchStatus() const;
    QString launchResultMessage() const;
    QString lastAction() const;
    QString lastResult() const;
    QString lastReason() const;
    QString nextAction() const;
    QString launcherAvailability() const;
    QString permissionsAvailability() const;
    QString sessionAvailability() const;
    QString sessionState() const;
    QString sessionHealth() const;

signals:
    void permissionPromptRequired(
        const QString &appId,
        const QString &appName,
        const QString &permission,
        const QString &permissionDisplayName,
        const QString &explanation);
    void appsChanged();
    void openAppsChanged();
    void selectedAppInfoChanged();
    void launchStatusChanged();
    void launchResultMessageChanged();
    void statusDetailsChanged();
    void runtimeStatusChanged();
    void activeAppChanged();

private:
    QVariantMap fetchAppInfo(const QString &appId);
    void requestLaunchFromLauncher(
        const QString &appId,
        const QString &appName,
        const QString &permission);
    QVariantMap fetchAppRuntime(const QString &appId);
    QVariantMap discoverWindowForPid(const QString &pid) const;
    QString querySystemActiveWindowId() const;
    bool focusWindow(const QString &windowId) const;
    void updateActiveApp(const QString &appId, bool userInitiated);
    void reconcileActiveApp();
    void logShellEvent(const QString &action, const QString &appId, const QString &details);
    void updateLaunchState(const QString &status, const QString &message);
    void updateStatusDetails(
        const QString &action,
        const QString &result,
        const QString &reason,
        const QString &nextAction);
    void handleAllowedLaunch(const QString &message);
    void handleDeniedLaunch(const QString &appName, const QString &permissionDisplayName);

    QVariantList m_apps;
    QVariantList m_openApps;
    QVariantMap m_sessionApps;
    QVariantMap m_selectedAppInfo;
    QString m_activeAppId;
    QString m_activeAppTitle;
    QString m_activeWindowId;
    QString m_activeWindowTitle;
    QString m_activeRuntimeState;
    QString m_pendingAppId;
    QString m_pendingAppName;
    QString m_pendingPermission;
    QString m_lastAction;
    QString m_lastResult;
    QString m_lastReason;
    QString m_nextAction;
    QString m_launchStatus;
    QString m_launchResultMessage;
    QString m_launcherAvailability {"unknown"};
    QString m_permissionsAvailability {"unknown"};
    QString m_sessionAvailability {"unknown"};
    QString m_sessionState {"unknown"};
    QString m_sessionHealth {"unknown"};
};
