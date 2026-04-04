#pragma once

#include <QObject>
#include <QString>

class PermissionClient : public QObject
{
    Q_OBJECT
    Q_PROPERTY(QString launchStatus READ launchStatus NOTIFY launchStatusChanged)
    Q_PROPERTY(QString launchResultMessage READ launchResultMessage NOTIFY launchResultMessageChanged)

public:
    explicit PermissionClient(QObject *parent = nullptr);

    Q_INVOKABLE void startLaunch(
        const QString &appId,
        const QString &appName,
        const QString &permission);
    Q_INVOKABLE void submitDecision(const QString &appId, const QString &appName, const QString &permission, bool allowed);
    Q_INVOKABLE void resetPermissions(const QString &appId);

    QString launchStatus() const;
    QString launchResultMessage() const;

signals:
    void permissionPromptRequired(
        const QString &appId,
        const QString &appName,
        const QString &permission,
        const QString &permissionDisplayName,
        const QString &explanation);
    void launchStatusChanged();
    void launchResultMessageChanged();

private:
    void requestLaunchFromLauncher(
        const QString &appId,
        const QString &appName,
        const QString &permission);
    void updateLaunchState(const QString &status, const QString &message);
    void handleAllowedLaunch(const QString &message);
    void handleDeniedLaunch(const QString &appName, const QString &permissionDisplayName);

    QString m_pendingAppId;
    QString m_pendingAppName;
    QString m_pendingPermission;
    QString m_launchStatus;
    QString m_launchResultMessage;
};
