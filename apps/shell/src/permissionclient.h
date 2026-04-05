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

public:
    explicit PermissionClient(QObject *parent = nullptr);

    Q_INVOKABLE void refreshApps();
    Q_INVOKABLE void selectApp(const QString &appId);
    Q_INVOKABLE void launchSelectedApp();
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

private:
    QVariantMap fetchAppInfo(const QString &appId);
    void requestLaunchFromLauncher(
        const QString &appId,
        const QString &appName,
        const QString &permission);
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
};
