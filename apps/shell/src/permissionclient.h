#pragma once

#include <QObject>
#include <QHash>
#include <QString>
#include <QStringList>
#include <QVariantList>
#include <QVariantMap>

class PermissionClient : public QObject
{
    Q_OBJECT
    Q_PROPERTY(QVariantList apps READ apps NOTIFY appsChanged)
    Q_PROPERTY(QVariantList openApps READ openApps NOTIFY openAppsChanged)
    Q_PROPERTY(QVariantList spaces READ spaces NOTIFY spacesChanged)
    Q_PROPERTY(QVariantList intents READ intents NOTIFY intentsChanged)
    Q_PROPERTY(QVariantList rules READ rules NOTIFY rulesChanged)
    Q_PROPERTY(QVariantMap selectedAppInfo READ selectedAppInfo NOTIFY selectedAppInfoChanged)
    Q_PROPERTY(QString selectedAppId READ selectedAppId NOTIFY selectedAppInfoChanged)
    Q_PROPERTY(QString activeAppId READ activeAppId NOTIFY activeAppChanged)
    Q_PROPERTY(QString activeAppTitle READ activeAppTitle NOTIFY activeAppChanged)
    Q_PROPERTY(QString activeWindowId READ activeWindowId NOTIFY activeAppChanged)
    Q_PROPERTY(QString activeWindowTitle READ activeWindowTitle NOTIFY activeAppChanged)
    Q_PROPERTY(QString activeRuntimeState READ activeRuntimeState NOTIFY activeAppChanged)
    Q_PROPERTY(QString inputControlMode READ inputControlMode NOTIFY inputStatusChanged)
    Q_PROPERTY(QString shortcutFeedback READ shortcutFeedback NOTIFY inputStatusChanged)
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
    Q_PROPERTY(QString currentVersion READ currentVersion NOTIFY runtimeStatusChanged)
    Q_PROPERTY(QString updateState READ updateState NOTIFY runtimeStatusChanged)
    Q_PROPERTY(QString lastUpdateResult READ lastUpdateResult NOTIFY runtimeStatusChanged)
    Q_PROPERTY(bool recoveryNeeded READ recoveryNeeded NOTIFY runtimeStatusChanged)
    Q_PROPERTY(QString lastIntentId READ lastIntentId NOTIFY intentsChanged)
    Q_PROPERTY(QString lastIntentResult READ lastIntentResult NOTIFY intentsChanged)
    Q_PROPERTY(QString lastRuleId READ lastRuleId NOTIFY rulesChanged)
    Q_PROPERTY(QString lastRuleResult READ lastRuleResult NOTIFY rulesChanged)
    Q_PROPERTY(QString agentSummary READ agentSummary NOTIFY agentStateChanged)
    Q_PROPERTY(QString lastAgentAction READ lastAgentAction NOTIFY agentStateChanged)
    Q_PROPERTY(QString lastAgentResult READ lastAgentResult NOTIFY agentStateChanged)
    Q_PROPERTY(QString activeSpaceId READ activeSpaceId NOTIFY spacesChanged)
    Q_PROPERTY(QString activeSpaceName READ activeSpaceName NOTIFY spacesChanged)
    Q_PROPERTY(QString activeSpaceState READ activeSpaceState NOTIFY spacesChanged)
    Q_PROPERTY(QString activeSpaceSecurityMode READ activeSpaceSecurityMode NOTIFY spacesChanged)
    Q_PROPERTY(QString activeSpacePreferredApp READ activeSpacePreferredApp NOTIFY spacesChanged)

public:
    explicit PermissionClient(QObject *parent = nullptr);

    Q_INVOKABLE void refreshApps();
    Q_INVOKABLE void refreshOpenApps();
    Q_INVOKABLE void refreshSpaces();
    Q_INVOKABLE void refreshIntents();
    Q_INVOKABLE void refreshRules();
    Q_INVOKABLE void refreshAgentState();
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
    Q_INVOKABLE void activateNextApp();
    Q_INVOKABLE void closeActiveApp();
    Q_INVOKABLE void restartActiveInstance();
    Q_INVOKABLE void activateAppByIndex(int index);
    Q_INVOKABLE void activateSpace(const QString &spaceId);
    Q_INVOKABLE void runIntent(const QString &intentId);
    Q_INVOKABLE void runRule(const QString &ruleId);
    Q_INVOKABLE void runAgentCommand(const QString &command);
    void setInputControlMode(const QString &mode, const QString &details);

    QVariantList apps() const;
    QVariantList openApps() const;
    QVariantList spaces() const;
    QVariantList intents() const;
    QVariantList rules() const;
    QVariantMap selectedAppInfo() const;
    QString selectedAppId() const;
    QString activeAppId() const;
    QString activeAppTitle() const;
    QString activeWindowId() const;
    QString activeWindowTitle() const;
    QString activeRuntimeState() const;
    QString inputControlMode() const;
    QString shortcutFeedback() const;
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
    QString currentVersion() const;
    QString updateState() const;
    QString lastUpdateResult() const;
    bool recoveryNeeded() const;
    QString lastIntentId() const;
    QString lastIntentResult() const;
    QString lastRuleId() const;
    QString lastRuleResult() const;
    QString agentSummary() const;
    QString lastAgentAction() const;
    QString lastAgentResult() const;
    QString activeSpaceId() const;
    QString activeSpaceName() const;
    QString activeSpaceState() const;
    QString activeSpaceSecurityMode() const;
    QString activeSpacePreferredApp() const;

signals:
    void permissionPromptRequired(
        const QString &appId,
        const QString &appName,
        const QString &permission,
        const QString &permissionDisplayName,
        const QString &explanation);
    void appsChanged();
    void openAppsChanged();
    void spacesChanged();
    void intentsChanged();
    void rulesChanged();
    void selectedAppInfoChanged();
    void launchStatusChanged();
    void launchResultMessageChanged();
    void statusDetailsChanged();
    void runtimeStatusChanged();
    void activeAppChanged();
    void inputStatusChanged();
    void agentStateChanged();

private:
    QVariantMap cachedAppInfo(const QString &appId) const;
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
    QVariantList m_spaces;
    QVariantList m_intents;
    QVariantList m_rules;
    QVariantMap m_sessionApps;
    QVariantMap m_selectedAppInfo;
    QString m_activeAppId;
    QString m_activeAppTitle;
    QString m_activeWindowId;
    QString m_activeWindowTitle;
    QString m_activeRuntimeState;
    QString m_inputControlMode {"disabled"};
    QString m_shortcutFeedback;
    QHash<QString, QString> m_windowAuditState;
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
    QString m_currentVersion;
    QString m_updateState {"unknown"};
    QString m_lastUpdateResult;
    bool m_recoveryNeeded {false};
    QString m_lastIntentId;
    QString m_lastIntentResult;
    QString m_lastRuleId;
    QString m_lastRuleResult;
    QString m_agentSummary;
    QString m_lastAgentAction;
    QString m_lastAgentResult;
    QString m_activeSpaceId;
    QString m_activeSpaceName;
    QString m_activeSpaceState {"unknown"};
    QString m_activeSpaceSecurityMode;
    QString m_activeSpacePreferredApp;
    QStringList m_activeSpaceApps;
};
