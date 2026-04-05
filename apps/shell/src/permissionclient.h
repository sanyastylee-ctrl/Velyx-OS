#pragma once

#include <QObject>
#include <QString>
#include <QVariantList>
#include <QVariantMap>

class PermissionClient : public QObject
{
    Q_OBJECT
    Q_PROPERTY(QVariantList apps READ apps NOTIFY appsChanged)
    Q_PROPERTY(QVariantMap selectedAppInfo READ selectedAppInfo NOTIFY selectedAppInfoChanged)
    Q_PROPERTY(QString selectedAppId READ selectedAppId NOTIFY selectedAppInfoChanged)
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
    Q_INVOKABLE void refreshRuntimeStatus();
    Q_INVOKABLE void selectApp(const QString &appId);
    Q_INVOKABLE void refreshSelectedAppRuntime();
    Q_INVOKABLE void launchSelectedApp();
    Q_INVOKABLE void stopSelectedApp();
    Q_INVOKABLE void restartSelectedApp();
    Q_INVOKABLE void startLaunch(
        const QString &appId,
        const QString &appName,
        const QString &permission);
    Q_INVOKABLE void submitDecision(const QString &appId, const QString &appName, const QString &permission, bool allowed);
    Q_INVOKABLE void resetPermissions(const QString &appId);

    QVariantList apps() const;
    QVariantMap selectedAppInfo() const;
    QString selectedAppId() const;
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
    void selectedAppInfoChanged();
    void launchStatusChanged();
    void launchResultMessageChanged();
    void statusDetailsChanged();
    void runtimeStatusChanged();

private:
    QVariantMap fetchAppInfo(const QString &appId);
    void requestLaunchFromLauncher(
        const QString &appId,
        const QString &appName,
        const QString &permission);
    QVariantMap fetchAppRuntime(const QString &appId);
    void updateLaunchState(const QString &status, const QString &message);
    void updateStatusDetails(
        const QString &action,
        const QString &result,
        const QString &reason,
        const QString &nextAction);
    void handleAllowedLaunch(const QString &message);
    void handleDeniedLaunch(const QString &appName, const QString &permissionDisplayName);

    QVariantList m_apps;
    QVariantMap m_selectedAppInfo;
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
