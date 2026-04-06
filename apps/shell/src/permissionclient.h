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
    Q_PROPERTY(QString aiMode READ aiMode NOTIFY aiStateChanged)
    Q_PROPERTY(QString aiProvider READ aiProvider NOTIFY aiStateChanged)
    Q_PROPERTY(QString aiModelName READ aiModelName NOTIFY aiStateChanged)
    Q_PROPERTY(QString aiModelProfile READ aiModelProfile NOTIFY aiStateChanged)
    Q_PROPERTY(QString aiSelectionMode READ aiSelectionMode NOTIFY aiStateChanged)
    Q_PROPERTY(QString aiRuntimeBackend READ aiRuntimeBackend NOTIFY aiStateChanged)
    Q_PROPERTY(QString aiRoutingReason READ aiRoutingReason NOTIFY aiStateChanged)
    Q_PROPERTY(QString aiFallbackReason READ aiFallbackReason NOTIFY aiStateChanged)
    Q_PROPERTY(bool aiModelAvailable READ aiModelAvailable NOTIFY aiStateChanged)
    Q_PROPERTY(QString aiLastSummary READ aiLastSummary NOTIFY aiStateChanged)
    Q_PROPERTY(QString aiSuggestionMessage READ aiSuggestionMessage NOTIFY aiStateChanged)
    Q_PROPERTY(QString aiSuggestionActionType READ aiSuggestionActionType NOTIFY aiStateChanged)
    Q_PROPERTY(QString aiSuggestionReason READ aiSuggestionReason NOTIFY aiStateChanged)
    Q_PROPERTY(double aiSuggestionConfidence READ aiSuggestionConfidence NOTIFY aiStateChanged)
    Q_PROPERTY(bool aiSuggestionAvailable READ aiSuggestionAvailable NOTIFY aiStateChanged)
    Q_PROPERTY(QString aiLastError READ aiLastError NOTIFY aiStateChanged)
    Q_PROPERTY(QString assistantMode READ assistantMode NOTIFY assistantStateChanged)
    Q_PROPERTY(QString assistantExecutionStatus READ assistantExecutionStatus NOTIFY assistantStateChanged)
    Q_PROPERTY(QString assistantLastRequest READ assistantLastRequest NOTIFY assistantStateChanged)
    Q_PROPERTY(QString assistantLastResponse READ assistantLastResponse NOTIFY assistantStateChanged)
    Q_PROPERTY(bool assistantPendingApproval READ assistantPendingApproval NOTIFY assistantStateChanged)
    Q_PROPERTY(QString assistantPendingRequestId READ assistantPendingRequestId NOTIFY assistantStateChanged)
    Q_PROPERTY(QString assistantPendingSummary READ assistantPendingSummary NOTIFY assistantStateChanged)
    Q_PROPERTY(QString assistantPendingDetails READ assistantPendingDetails NOTIFY assistantStateChanged)
    Q_PROPERTY(QVariantList assistantHistory READ assistantHistory NOTIFY assistantStateChanged)
    Q_PROPERTY(bool devModeEnabled READ devModeEnabled NOTIFY devModeStateChanged)
    Q_PROPERTY(QString devAgentMode READ devAgentMode NOTIFY devModeStateChanged)
    Q_PROPERTY(QString devOverlayPath READ devOverlayPath NOTIFY devModeStateChanged)
    Q_PROPERTY(QString devLastChange READ devLastChange NOTIFY devModeStateChanged)
    Q_PROPERTY(QString devChangeClass READ devChangeClass NOTIFY devModeStateChanged)
    Q_PROPERTY(QString devApplyStrategy READ devApplyStrategy NOTIFY devModeStateChanged)
    Q_PROPERTY(QString devScope READ devScope NOTIFY devModeStateChanged)
    Q_PROPERTY(QString devApprovalLevel READ devApprovalLevel NOTIFY devModeStateChanged)
    Q_PROPERTY(bool devVisualFeedbackActive READ devVisualFeedbackActive NOTIFY devModeStateChanged)
    Q_PROPERTY(bool devAutoRefine READ devAutoRefine NOTIFY devModeStateChanged)
    Q_PROPERTY(QString devLastScreenshotPath READ devLastScreenshotPath NOTIFY devModeStateChanged)
    Q_PROPERTY(QString devPreviousScreenshotPath READ devPreviousScreenshotPath NOTIFY devModeStateChanged)
    Q_PROPERTY(QString devVisualSummary READ devVisualSummary NOTIFY devModeStateChanged)
    Q_PROPERTY(QString devVisualRecommendation READ devVisualRecommendation NOTIFY devModeStateChanged)
    Q_PROPERTY(QString devPendingRefinement READ devPendingRefinement NOTIFY devModeStateChanged)
    Q_PROPERTY(bool firstBootRequired READ firstBootRequired NOTIFY firstBootStateChanged)
    Q_PROPERTY(QString firstBootStep READ firstBootStep NOTIFY firstBootStateChanged)
    Q_PROPERTY(QString firstBootVersion READ firstBootVersion NOTIFY firstBootStateChanged)
    Q_PROPERTY(QString firstBootInstallMode READ firstBootInstallMode NOTIFY firstBootStateChanged)
    Q_PROPERTY(QString firstBootNetworkState READ firstBootNetworkState NOTIFY firstBootStateChanged)
    Q_PROPERTY(bool firstBootSystemReady READ firstBootSystemReady NOTIFY firstBootStateChanged)
    Q_PROPERTY(QString firstBootAiMode READ firstBootAiMode NOTIFY firstBootStateChanged)
    Q_PROPERTY(QString firstBootModelSelectionMode READ firstBootModelSelectionMode NOTIFY firstBootStateChanged)
    Q_PROPERTY(QString firstBootDefaultSpace READ firstBootDefaultSpace NOTIFY firstBootStateChanged)
    Q_PROPERTY(QString firstBootPredictiveMode READ firstBootPredictiveMode NOTIFY firstBootStateChanged)
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
    Q_INVOKABLE void refreshAiState();
    Q_INVOKABLE void refreshAssistantState();
    Q_INVOKABLE void refreshDevModeState();
    Q_INVOKABLE void refreshFirstBootState();
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
    Q_INVOKABLE void setAiMode(const QString &mode);
    Q_INVOKABLE void runAiSummary();
    Q_INVOKABLE void runAiExplain();
    Q_INVOKABLE void runAiSuggest();
    Q_INVOKABLE void applyAiSuggestion();
    Q_INVOKABLE void dismissAiSuggestion();
    Q_INVOKABLE void blockAiSuggestion();
    Q_INVOKABLE void setModelSelectionMode(const QString &mode);
    Q_INVOKABLE void detectModelHardware();
    Q_INVOKABLE void setFirstBootAiMode(const QString &mode);
    Q_INVOKABLE void setFirstBootStep(const QString &step);
    Q_INVOKABLE void setFirstBootModelSelectionMode(const QString &mode);
    Q_INVOKABLE void setFirstBootDefaultSpace(const QString &spaceId);
    Q_INVOKABLE void setFirstBootPredictiveMode(const QString &mode);
    Q_INVOKABLE void rerunFirstBootChecks();
    Q_INVOKABLE void completeFirstBoot();
    Q_INVOKABLE void runRecoveryFlow();
    Q_INVOKABLE void exportDiagnostics();
    Q_INVOKABLE void askAssistant(const QString &query);
    Q_INVOKABLE void approveAssistant(const QString &requestId);
    Q_INVOKABLE void denyAssistant(const QString &requestId);
    Q_INVOKABLE void setAssistantMode(const QString &mode);
    Q_INVOKABLE void enableDevMode();
    Q_INVOKABLE void disableDevMode();
    Q_INVOKABLE void rollbackDevMode();
    Q_INVOKABLE void restartShellDev();
    Q_INVOKABLE void setDevAutoRefine(bool enabled);
    Q_INVOKABLE void applyNextDevRefinement();
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
    QString aiMode() const;
    QString aiProvider() const;
    QString aiModelName() const;
    QString aiModelProfile() const;
    QString aiSelectionMode() const;
    QString aiRuntimeBackend() const;
    QString aiRoutingReason() const;
    QString aiFallbackReason() const;
    bool aiModelAvailable() const;
    QString aiLastSummary() const;
    QString aiSuggestionMessage() const;
    QString aiSuggestionActionType() const;
    QString aiSuggestionReason() const;
    double aiSuggestionConfidence() const;
    bool aiSuggestionAvailable() const;
    QString aiLastError() const;
    QString assistantMode() const;
    QString assistantExecutionStatus() const;
    QString assistantLastRequest() const;
    QString assistantLastResponse() const;
    bool assistantPendingApproval() const;
    QString assistantPendingRequestId() const;
    QString assistantPendingSummary() const;
    QString assistantPendingDetails() const;
    QVariantList assistantHistory() const;
    bool devModeEnabled() const;
    QString devAgentMode() const;
    QString devOverlayPath() const;
    QString devLastChange() const;
    QString devChangeClass() const;
    QString devApplyStrategy() const;
    QString devScope() const;
    QString devApprovalLevel() const;
    bool devVisualFeedbackActive() const;
    bool devAutoRefine() const;
    QString devLastScreenshotPath() const;
    QString devPreviousScreenshotPath() const;
    QString devVisualSummary() const;
    QString devVisualRecommendation() const;
    QString devPendingRefinement() const;
    bool firstBootRequired() const;
    QString firstBootStep() const;
    QString firstBootVersion() const;
    QString firstBootInstallMode() const;
    QString firstBootNetworkState() const;
    bool firstBootSystemReady() const;
    QString firstBootAiMode() const;
    QString firstBootModelSelectionMode() const;
    QString firstBootDefaultSpace() const;
    QString firstBootPredictiveMode() const;
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
    void aiStateChanged();
    void assistantStateChanged();
    void devModeStateChanged();
    void firstBootStateChanged();

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
    QString m_aiMode {"off"};
    QString m_aiProvider {"local"};
    QString m_aiModelName;
    QString m_aiModelProfile;
    QString m_aiSelectionMode {"manual"};
    QString m_aiRuntimeBackend {"stub"};
    QString m_aiRoutingReason;
    QString m_aiFallbackReason;
    bool m_aiModelAvailable {false};
    QString m_aiLastSummary;
    QString m_aiSuggestionMessage;
    QString m_aiSuggestionActionType;
    QString m_aiSuggestionReason;
    double m_aiSuggestionConfidence {0.0};
    bool m_aiSuggestionAvailable {false};
    QString m_aiLastError;
    QString m_assistantMode {"suggest"};
    QString m_assistantExecutionStatus {"idle"};
    QString m_assistantLastRequest;
    QString m_assistantLastResponse;
    bool m_assistantPendingApproval {false};
    QString m_assistantPendingRequestId;
    QString m_assistantPendingSummary;
    QString m_assistantPendingDetails;
    QVariantList m_assistantHistory;
    bool m_devModeEnabled {false};
    QString m_devAgentMode {"disabled"};
    QString m_devOverlayPath;
    QString m_devLastChange;
    QString m_devChangeClass;
    QString m_devApplyStrategy;
    QString m_devScope;
    QString m_devApprovalLevel;
    bool m_devVisualFeedbackActive {false};
    bool m_devAutoRefine {false};
    QString m_devLastScreenshotPath;
    QString m_devPreviousScreenshotPath;
    QString m_devVisualSummary;
    QString m_devVisualRecommendation;
    QString m_devPendingRefinement;
    bool m_firstBootRequired {false};
    QString m_firstBootStep {"welcome"};
    QString m_firstBootVersion;
    QString m_firstBootInstallMode {"standard_preview"};
    QString m_firstBootNetworkState {"unknown"};
    bool m_firstBootSystemReady {false};
    QString m_firstBootAiMode {"off"};
    QString m_firstBootModelSelectionMode {"auto_hardware"};
    QString m_firstBootDefaultSpace {"general"};
    QString m_firstBootPredictiveMode {"off"};
    QString m_activeSpaceId;
    QString m_activeSpaceName;
    QString m_activeSpaceState {"unknown"};
    QString m_activeSpaceSecurityMode;
    QString m_activeSpacePreferredApp;
    QStringList m_activeSpaceApps;
};
