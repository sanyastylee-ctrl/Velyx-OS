#include "permissionclient.h"

#include <QDBusConnection>
#include <QDBusInterface>
#include <QDBusMessage>
#include <QDBusReply>
#include <QDateTime>
#include <QJsonArray>
#include <QJsonDocument>
#include <QJsonObject>
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

QVariantList PermissionClient::spaces() const
{
    return m_spaces;
}

QVariantList PermissionClient::intents() const
{
    return m_intents;
}

QVariantList PermissionClient::rules() const
{
    return m_rules;
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

QString PermissionClient::currentVersion() const
{
    return m_currentVersion;
}

QString PermissionClient::updateState() const
{
    return m_updateState;
}

QString PermissionClient::lastUpdateResult() const
{
    return m_lastUpdateResult;
}

bool PermissionClient::recoveryNeeded() const
{
    return m_recoveryNeeded;
}

QString PermissionClient::lastIntentId() const
{
    return m_lastIntentId;
}

QString PermissionClient::lastIntentResult() const
{
    return m_lastIntentResult;
}

QString PermissionClient::lastRuleId() const
{
    return m_lastRuleId;
}

QString PermissionClient::lastRuleResult() const
{
    return m_lastRuleResult;
}

QString PermissionClient::agentSummary() const
{
    return m_agentSummary;
}

QString PermissionClient::lastAgentAction() const
{
    return m_lastAgentAction;
}

QString PermissionClient::lastAgentResult() const
{
    return m_lastAgentResult;
}

QString PermissionClient::aiMode() const
{
    return m_aiMode;
}

QString PermissionClient::aiProvider() const
{
    return m_aiProvider;
}

QString PermissionClient::aiModelName() const
{
    return m_aiModelName;
}

QString PermissionClient::aiModelProfile() const
{
    return m_aiModelProfile;
}

QString PermissionClient::aiSelectionMode() const
{
    return m_aiSelectionMode;
}

QString PermissionClient::aiRuntimeBackend() const
{
    return m_aiRuntimeBackend;
}

QString PermissionClient::aiRoutingReason() const
{
    return m_aiRoutingReason;
}

QString PermissionClient::aiFallbackReason() const
{
    return m_aiFallbackReason;
}

bool PermissionClient::aiModelAvailable() const
{
    return m_aiModelAvailable;
}

QString PermissionClient::aiLastSummary() const
{
    return m_aiLastSummary;
}

QString PermissionClient::aiSuggestionMessage() const
{
    return m_aiSuggestionMessage;
}

QString PermissionClient::aiSuggestionActionType() const
{
    return m_aiSuggestionActionType;
}

QString PermissionClient::aiSuggestionReason() const
{
    return m_aiSuggestionReason;
}

double PermissionClient::aiSuggestionConfidence() const
{
    return m_aiSuggestionConfidence;
}

bool PermissionClient::aiSuggestionAvailable() const
{
    return m_aiSuggestionAvailable;
}

QString PermissionClient::aiLastError() const
{
    return m_aiLastError;
}

QString PermissionClient::assistantMode() const
{
    return m_assistantMode;
}

QString PermissionClient::assistantExecutionStatus() const
{
    return m_assistantExecutionStatus;
}

QString PermissionClient::assistantLastRequest() const
{
    return m_assistantLastRequest;
}

QString PermissionClient::assistantLastResponse() const
{
    return m_assistantLastResponse;
}

bool PermissionClient::assistantPendingApproval() const
{
    return m_assistantPendingApproval;
}

QString PermissionClient::assistantPendingRequestId() const
{
    return m_assistantPendingRequestId;
}

QString PermissionClient::assistantPendingSummary() const
{
    return m_assistantPendingSummary;
}

QString PermissionClient::assistantPendingDetails() const
{
    return m_assistantPendingDetails;
}

QVariantList PermissionClient::assistantHistory() const
{
    return m_assistantHistory;
}

bool PermissionClient::devModeEnabled() const
{
    return m_devModeEnabled;
}

QString PermissionClient::devOverlayPath() const
{
    return m_devOverlayPath;
}

QString PermissionClient::devLastChange() const
{
    return m_devLastChange;
}

bool PermissionClient::devVisualFeedbackActive() const
{
    return m_devVisualFeedbackActive;
}

bool PermissionClient::devAutoRefine() const
{
    return m_devAutoRefine;
}

QString PermissionClient::devLastScreenshotPath() const
{
    return m_devLastScreenshotPath;
}

QString PermissionClient::devPreviousScreenshotPath() const
{
    return m_devPreviousScreenshotPath;
}

QString PermissionClient::devVisualSummary() const
{
    return m_devVisualSummary;
}

QString PermissionClient::devVisualRecommendation() const
{
    return m_devVisualRecommendation;
}

QString PermissionClient::devPendingRefinement() const
{
    return m_devPendingRefinement;
}

bool PermissionClient::firstBootRequired() const
{
    return m_firstBootRequired;
}

QString PermissionClient::firstBootStep() const
{
    return m_firstBootStep;
}

QString PermissionClient::firstBootVersion() const
{
    return m_firstBootVersion;
}

QString PermissionClient::firstBootInstallMode() const
{
    return m_firstBootInstallMode;
}

QString PermissionClient::firstBootNetworkState() const
{
    return m_firstBootNetworkState;
}

bool PermissionClient::firstBootSystemReady() const
{
    return m_firstBootSystemReady;
}

QString PermissionClient::firstBootAiMode() const
{
    return m_firstBootAiMode;
}

QString PermissionClient::firstBootModelSelectionMode() const
{
    return m_firstBootModelSelectionMode;
}

QString PermissionClient::firstBootDefaultSpace() const
{
    return m_firstBootDefaultSpace;
}

QString PermissionClient::firstBootPredictiveMode() const
{
    return m_firstBootPredictiveMode;
}

QString PermissionClient::activeSpaceId() const
{
    return m_activeSpaceId;
}

QString PermissionClient::activeSpaceName() const
{
    return m_activeSpaceName;
}

QString PermissionClient::activeSpaceState() const
{
    return m_activeSpaceState;
}

QString PermissionClient::activeSpaceSecurityMode() const
{
    return m_activeSpaceSecurityMode;
}

QString PermissionClient::activeSpacePreferredApp() const
{
    return m_activeSpacePreferredApp;
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
    refreshSpaces();
    refreshIntents();
    refreshRules();
    refreshAgentState();
    refreshAiState();
    refreshAssistantState();
    refreshDevModeState();
    refreshFirstBootState();

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
            map.insert("in_active_space", sessionApp.value("in_active_space", false));
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

void PermissionClient::refreshSpaces()
{
    QDBusInterface sessionManager(
        kSessionManagerService,
        kSessionManagerPath,
        kSessionManagerInterface,
        QDBusConnection::sessionBus());
    if (!sessionManager.isValid()) {
        if (!m_spaces.isEmpty()) {
            m_spaces.clear();
            emit spacesChanged();
        }
        return;
    }

    const QDBusMessage reply = sessionManager.call("ListSpaces");
    if (reply.type() == QDBusMessage::ErrorMessage || reply.arguments().isEmpty()) {
        return;
    }

    QVariantList spaces;
    for (const QVariant &item : reply.arguments().constFirst().toList()) {
        const QVariantMap map = item.toMap();
        if (!map.isEmpty()) {
            spaces.push_back(map);
        }
    }

    if (m_spaces != spaces) {
        m_spaces = spaces;
        emit spacesChanged();
    }
}

void PermissionClient::refreshIntents()
{
    const QString intentsRegistryPath = QDir::home().filePath(".velyx/intents_registry.json");
    const QString intentStatePath = QDir::home().filePath(".velyx/intent_state.json");

    QVariantList intents;
    QFile registryFile(intentsRegistryPath);
    if (registryFile.open(QIODevice::ReadOnly)) {
        const QJsonDocument document = QJsonDocument::fromJson(registryFile.readAll());
        if (document.isObject()) {
            const QJsonArray entries = document.object().value("intents").toArray();
            for (const QJsonValue &entry : entries) {
                if (entry.isObject()) {
                    intents.push_back(entry.toObject().toVariantMap());
                }
            }
        }
    }

    QString lastIntentId;
    QString lastIntentResult;
    QFile stateFile(intentStatePath);
    if (stateFile.open(QIODevice::ReadOnly)) {
        const QJsonDocument document = QJsonDocument::fromJson(stateFile.readAll());
        if (document.isObject()) {
            const QJsonObject object = document.object();
            lastIntentId = object.value("last_intent_id").toString();
            lastIntentResult = object.value("last_result").toString();
        }
    }

    bool changed = false;
    if (m_intents != intents) {
        m_intents = intents;
        changed = true;
    }
    if (m_lastIntentId != lastIntentId) {
        m_lastIntentId = lastIntentId;
        changed = true;
    }
    if (m_lastIntentResult != lastIntentResult) {
        m_lastIntentResult = lastIntentResult;
        changed = true;
    }
    if (changed) {
        emit intentsChanged();
    }
}

void PermissionClient::refreshRules()
{
    const QString rulesRegistryPath = QDir::home().filePath(".velyx/rules_registry.json");
    const QString rulesStatePath = QDir::home().filePath(".velyx/rules_state.json");

    QVariantList rules;
    QFile registryFile(rulesRegistryPath);
    if (registryFile.open(QIODevice::ReadOnly)) {
        const QJsonDocument document = QJsonDocument::fromJson(registryFile.readAll());
        if (document.isObject()) {
            const QJsonArray entries = document.object().value("rules").toArray();
            for (const QJsonValue &entry : entries) {
                if (entry.isObject()) {
                    rules.push_back(entry.toObject().toVariantMap());
                }
            }
        }
    }

    QString lastRuleId;
    QString lastRuleResult;
    QFile stateFile(rulesStatePath);
    if (stateFile.open(QIODevice::ReadOnly)) {
        const QJsonDocument document = QJsonDocument::fromJson(stateFile.readAll());
        if (document.isObject()) {
            const QJsonObject object = document.object();
            lastRuleId = object.value("last_rule_id").toString();
            lastRuleResult = object.value("last_result").toString();
        }
    }

    bool changed = false;
    if (m_rules != rules) {
        m_rules = rules;
        changed = true;
    }
    if (m_lastRuleId != lastRuleId) {
        m_lastRuleId = lastRuleId;
        changed = true;
    }
    if (m_lastRuleResult != lastRuleResult) {
        m_lastRuleResult = lastRuleResult;
        changed = true;
    }
    if (changed) {
        emit rulesChanged();
    }
}

void PermissionClient::refreshAgentState()
{
    const QString agentStatePath = QDir::home().filePath(".velyx/agent_state.json");

    QString summary;
    QString lastAction;
    QString lastResult;

    QFile stateFile(agentStatePath);
    if (stateFile.open(QIODevice::ReadOnly)) {
        const QJsonDocument document = QJsonDocument::fromJson(stateFile.readAll());
        if (document.isObject()) {
            const QJsonObject object = document.object();
            summary = object.value("last_summary").toString();
            lastAction = object.value("last_action").toString();
            lastResult = object.value("last_result").toString();
        }
    }

    bool changed = false;
    if (m_agentSummary != summary) {
        m_agentSummary = summary;
        changed = true;
    }
    if (m_lastAgentAction != lastAction) {
        m_lastAgentAction = lastAction;
        changed = true;
    }
    if (m_lastAgentResult != lastResult) {
        m_lastAgentResult = lastResult;
        changed = true;
    }
    if (changed) {
        emit agentStateChanged();
    }
}

void PermissionClient::refreshAiState()
{
    QProcess tickProcess;
    tickProcess.start("velyx-ai", {"tick"});
    if (tickProcess.waitForStarted(200)) {
        tickProcess.waitForFinished(1200);
    }

    const QString configPath = QDir::home().filePath(".velyx/ai_config.json");
    const QString statePath = QDir::home().filePath(".velyx/ai_state.json");
    const QString modelStatePath = QDir::home().filePath(".velyx/model_state.json");

    QString mode {"off"};
    QString provider {"local"};
    QString modelName;
    QString modelProfile;
    QString selectionMode {"manual"};
    QString runtimeBackend {"stub"};
    QString routingReason;
    QString fallbackReason;
    bool modelAvailable = false;
    QString lastSummary;
    QString suggestionMessage;
    QString suggestionActionType;
    QString suggestionReason;
    double suggestionConfidence = 0.0;
    bool suggestionAvailable = false;
    QString lastError;

    QFile configFile(configPath);
    if (configFile.open(QIODevice::ReadOnly)) {
        const QJsonDocument document = QJsonDocument::fromJson(configFile.readAll());
        if (document.isObject()) {
            const QJsonObject object = document.object();
            mode = object.value("mode").toString(mode);
            provider = object.value("provider").toString(provider);
            modelName = object.value("model_name").toString(modelName);
            runtimeBackend = object.value("endpoint_type").toString(runtimeBackend);
        }
    }

    QFile stateFile(statePath);
    if (stateFile.open(QIODevice::ReadOnly)) {
        const QJsonDocument document = QJsonDocument::fromJson(stateFile.readAll());
        if (document.isObject()) {
            const QJsonObject object = document.object();
            mode = object.value("current_mode").toString(mode);
            modelName = object.value("current_model").toString(modelName);
            modelProfile = object.value("current_model_profile").toString(modelProfile);
            selectionMode = object.value("selection_mode").toString(selectionMode);
            runtimeBackend = object.value("runtime_backend").toString(runtimeBackend);
            modelAvailable = object.value("model_available").toBool(false);
            lastSummary = object.value("last_summary").toString();
            lastError = object.value("last_error").toString();
            routingReason = object.value("last_routing_reason").toString();
            fallbackReason = object.value("last_fallback_reason").toString();

            const QJsonObject suggestion = object.value("last_suggestion").toObject();
            if (!suggestion.isEmpty()) {
                suggestionMessage = suggestion.value("user_message").toString();
                suggestionActionType = suggestion.value("recommended_action_type").toString();
                suggestionReason = suggestion.value("reason").toString();
                suggestionConfidence = suggestion.value("confidence").toDouble(0.0);
                suggestionAvailable = suggestion.value("dismissed").toBool(false) == false
                    && (suggestionActionType != "none" || !suggestionMessage.isEmpty());
            }
        }
    }

    QFile modelStateFile(modelStatePath);
    if (modelStateFile.open(QIODevice::ReadOnly)) {
        const QJsonDocument document = QJsonDocument::fromJson(modelStateFile.readAll());
        if (document.isObject()) {
            const QJsonObject object = document.object();
            const QJsonObject route = object.value("last_route").toObject();
            if (modelProfile.isEmpty()) {
                modelProfile = object.value("active_profile").toString(modelProfile);
            }
            if (selectionMode == "manual") {
                selectionMode = object.value("selection_mode").toString(selectionMode);
            }
            if (runtimeBackend == "stub") {
                runtimeBackend = route.value("runtime_backend").toString(runtimeBackend);
            }
            if (modelName.isEmpty()) {
                modelName = object.value("active_model").toString(modelName);
            }
            if (!modelAvailable) {
                modelAvailable = object.value("model_available").toBool(modelAvailable);
            }
            if (fallbackReason.isEmpty()) {
                fallbackReason = route.value("fallback_reason").toString(fallbackReason);
                routingReason = route.value("explanation").toString(routingReason);
            }
        }
    }

    bool changed = false;
    if (m_aiMode != mode) {
        m_aiMode = mode;
        changed = true;
    }
    if (m_aiProvider != provider) {
        m_aiProvider = provider;
        changed = true;
    }
    if (m_aiModelName != modelName) {
        m_aiModelName = modelName;
        changed = true;
    }
    if (m_aiModelProfile != modelProfile) {
        m_aiModelProfile = modelProfile;
        changed = true;
    }
    if (m_aiSelectionMode != selectionMode) {
        m_aiSelectionMode = selectionMode;
        changed = true;
    }
    if (m_aiRuntimeBackend != runtimeBackend) {
        m_aiRuntimeBackend = runtimeBackend;
        changed = true;
    }
    if (m_aiRoutingReason != routingReason) {
        m_aiRoutingReason = routingReason;
        changed = true;
    }
    if (m_aiFallbackReason != fallbackReason) {
        m_aiFallbackReason = fallbackReason;
        changed = true;
    }
    if (m_aiModelAvailable != modelAvailable) {
        m_aiModelAvailable = modelAvailable;
        changed = true;
    }
    if (m_aiLastSummary != lastSummary) {
        m_aiLastSummary = lastSummary;
        changed = true;
    }
    if (m_aiSuggestionMessage != suggestionMessage) {
        m_aiSuggestionMessage = suggestionMessage;
        changed = true;
    }
    if (m_aiSuggestionActionType != suggestionActionType) {
        m_aiSuggestionActionType = suggestionActionType;
        changed = true;
    }
    if (m_aiSuggestionReason != suggestionReason) {
        m_aiSuggestionReason = suggestionReason;
        changed = true;
    }
    if (m_aiSuggestionConfidence != suggestionConfidence) {
        m_aiSuggestionConfidence = suggestionConfidence;
        changed = true;
    }
    if (m_aiSuggestionAvailable != suggestionAvailable) {
        m_aiSuggestionAvailable = suggestionAvailable;
        changed = true;
    }
    if (m_aiLastError != lastError) {
        m_aiLastError = lastError;
        changed = true;
    }

    if (changed) {
        emit aiStateChanged();
    }
}

void PermissionClient::refreshAssistantState()
{
    const QString configPath = QDir::home().filePath(".velyx/assistant_config.json");
    const QString statePath = QDir::home().filePath(".velyx/assistant_state.json");

    QString mode {"suggest"};
    QString executionStatus {"idle"};
    QString lastRequest;
    QString lastResponse;
    bool pendingApproval = false;
    QString pendingRequestId;
    QString pendingSummary;
    QString pendingDetails;
    QVariantList history;

    QFile configFile(configPath);
    if (configFile.open(QIODevice::ReadOnly)) {
        const QJsonDocument document = QJsonDocument::fromJson(configFile.readAll());
        if (document.isObject()) {
            mode = document.object().value("mode").toString(mode);
        }
    }

    QFile stateFile(statePath);
    if (stateFile.open(QIODevice::ReadOnly)) {
        const QJsonDocument document = QJsonDocument::fromJson(stateFile.readAll());
        if (document.isObject()) {
            const QJsonObject object = document.object();
            executionStatus = object.value("current_execution_status").toString(executionStatus);
            lastRequest = object.value("last_user_request").toString();
            lastResponse = object.value("last_assistant_response").toString();

            const QJsonObject pending = object.value("pending_approval").toObject();
            if (!pending.isEmpty()) {
                pendingApproval = true;
                pendingRequestId = pending.value("request_id").toString();
                pendingSummary = pending.value("summary").toString();
                const QJsonObject step = pending.value("step").toObject();
                pendingDetails = step.value("tool_id").toString();
            }

            const QJsonArray historyArray = object.value("history").toArray();
            for (const QJsonValue &value : historyArray) {
                history.push_back(value.toObject().toVariantMap());
            }
        }
    }

    bool changed = false;
    if (m_assistantMode != mode) {
        m_assistantMode = mode;
        changed = true;
    }
    if (m_assistantExecutionStatus != executionStatus) {
        m_assistantExecutionStatus = executionStatus;
        changed = true;
    }
    if (m_assistantLastRequest != lastRequest) {
        m_assistantLastRequest = lastRequest;
        changed = true;
    }
    if (m_assistantLastResponse != lastResponse) {
        m_assistantLastResponse = lastResponse;
        changed = true;
    }
    if (m_assistantPendingApproval != pendingApproval) {
        m_assistantPendingApproval = pendingApproval;
        changed = true;
    }
    if (m_assistantPendingRequestId != pendingRequestId) {
        m_assistantPendingRequestId = pendingRequestId;
        changed = true;
    }
    if (m_assistantPendingSummary != pendingSummary) {
        m_assistantPendingSummary = pendingSummary;
        changed = true;
    }
    if (m_assistantPendingDetails != pendingDetails) {
        m_assistantPendingDetails = pendingDetails;
        changed = true;
    }
    if (m_assistantHistory != history) {
        m_assistantHistory = history;
        changed = true;
    }

    if (changed) {
        emit assistantStateChanged();
    }
}

void PermissionClient::refreshDevModeState()
{
    const QString statePath = QDir::home().filePath(".velyx/dev_mode.json");

    bool enabled = false;
    QString overlayPath;
    QString lastChange;
    bool visualFeedbackActive = false;
    bool autoRefine = false;
    QString lastScreenshotPath;
    QString previousScreenshotPath;
    QString visualSummary;
    QString visualRecommendation;
    QString pendingRefinement;

    QFile stateFile(statePath);
    if (stateFile.open(QIODevice::ReadOnly)) {
        const QJsonDocument document = QJsonDocument::fromJson(stateFile.readAll());
        if (document.isObject()) {
            const QJsonObject object = document.object();
            enabled = object.value("enabled").toBool(false);
            overlayPath = object.value("overlay_root").toString();
            lastChange = object.value("last_change_summary").toString();
            visualFeedbackActive = object.value("visual_feedback_active").toBool(false);
            autoRefine = object.value("auto_refine").toBool(false);
            lastScreenshotPath = object.value("last_screenshot").toString();
            previousScreenshotPath = object.value("previous_screenshot").toString();
            visualSummary = object.value("last_visual_summary").toString();
            visualRecommendation = object.value("last_visual_recommendation").toString();
            pendingRefinement = object.value("pending_refinement_request").toString();
        }
    }

    bool changed = false;
    if (m_devModeEnabled != enabled) {
        m_devModeEnabled = enabled;
        changed = true;
    }
    if (m_devOverlayPath != overlayPath) {
        m_devOverlayPath = overlayPath;
        changed = true;
    }
    if (m_devLastChange != lastChange) {
        m_devLastChange = lastChange;
        changed = true;
    }
    if (m_devVisualFeedbackActive != visualFeedbackActive) {
        m_devVisualFeedbackActive = visualFeedbackActive;
        changed = true;
    }
    if (m_devAutoRefine != autoRefine) {
        m_devAutoRefine = autoRefine;
        changed = true;
    }
    if (m_devLastScreenshotPath != lastScreenshotPath) {
        m_devLastScreenshotPath = lastScreenshotPath;
        changed = true;
    }
    if (m_devPreviousScreenshotPath != previousScreenshotPath) {
        m_devPreviousScreenshotPath = previousScreenshotPath;
        changed = true;
    }
    if (m_devVisualSummary != visualSummary) {
        m_devVisualSummary = visualSummary;
        changed = true;
    }
    if (m_devVisualRecommendation != visualRecommendation) {
        m_devVisualRecommendation = visualRecommendation;
        changed = true;
    }
    if (m_devPendingRefinement != pendingRefinement) {
        m_devPendingRefinement = pendingRefinement;
        changed = true;
    }
    if (changed) {
        emit devModeStateChanged();
    }
}

void PermissionClient::refreshFirstBootState()
{
    QProcess tickProcess;
    tickProcess.start("velyx-firstboot", {"status"});
    if (tickProcess.waitForStarted(200)) {
        tickProcess.waitForFinished(1200);
    }

    const QString statePath = QDir::home().filePath(".velyx/first_boot_state.json");
    bool required = false;
    QString step {"welcome"};
    QString version;
    QString installMode {"standard_preview"};
    QString networkState {"unknown"};
    bool systemReady = false;
    QString aiMode {"off"};
    QString modelSelectionMode {"auto_hardware"};
    QString defaultSpace {"general"};
    QString predictiveMode {"off"};

    QFile stateFile(statePath);
    if (stateFile.open(QIODevice::ReadOnly)) {
        const QJsonDocument document = QJsonDocument::fromJson(stateFile.readAll());
        if (document.isObject()) {
            const QJsonObject object = document.object();
            required = object.value("first_boot_required").toBool(required);
            step = object.value("current_step").toString(step);
            version = object.value("version").toString(version);
            installMode = object.value("install_mode").toString(installMode);
            networkState = object.value("network_state").toString(networkState);
            systemReady = object.value("system_ready").toBool(systemReady);
            aiMode = object.value("ai_mode").toString(aiMode);
            modelSelectionMode = object.value("model_selection_mode").toString(modelSelectionMode);
            defaultSpace = object.value("default_space").toString(defaultSpace);
            predictiveMode = object.value("predictive_mode").toString(predictiveMode);
        }
    }

    bool changed = false;
    if (m_firstBootRequired != required) {
        m_firstBootRequired = required;
        changed = true;
    }
    if (m_firstBootStep != step) {
        m_firstBootStep = step;
        changed = true;
    }
    if (m_firstBootVersion != version) {
        m_firstBootVersion = version;
        changed = true;
    }
    if (m_firstBootInstallMode != installMode) {
        m_firstBootInstallMode = installMode;
        changed = true;
    }
    if (m_firstBootNetworkState != networkState) {
        m_firstBootNetworkState = networkState;
        changed = true;
    }
    if (m_firstBootSystemReady != systemReady) {
        m_firstBootSystemReady = systemReady;
        changed = true;
    }
    if (m_firstBootAiMode != aiMode) {
        m_firstBootAiMode = aiMode;
        changed = true;
    }
    if (m_firstBootModelSelectionMode != modelSelectionMode) {
        m_firstBootModelSelectionMode = modelSelectionMode;
        changed = true;
    }
    if (m_firstBootDefaultSpace != defaultSpace) {
        m_firstBootDefaultSpace = defaultSpace;
        changed = true;
    }
    if (m_firstBootPredictiveMode != predictiveMode) {
        m_firstBootPredictiveMode = predictiveMode;
        changed = true;
    }
    if (changed) {
        emit firstBootStateChanged();
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
        runtime.insert("in_active_space", sessionApp.value("in_active_space", false));
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
    QString activeSpaceId;
    QString activeSpaceName;
    QString activeSpaceState;
    QString activeSpaceSecurityMode;
    QString activeSpacePreferredApp;
    QStringList activeSpaceApps;
    QString currentVersion;
    QString updateState = "unknown";
    QString lastUpdateResult;
    bool recoveryNeeded = false;
    if (sessionManager.isValid()) {
        QDBusReply<QVariantMap> reply = sessionManager.call("GetSessionState");
        if (reply.isValid()) {
            const QVariantMap payload = reply.value();
            sessionState = payload.value("state", "unknown").toString();
            sessionHealth = payload.value("health", "unknown").toString();
            activeSpaceId = payload.value("active_space_id").toString();
            activeSpaceName = payload.value("active_space_name").toString();
            activeSpaceState = payload.value("active_space_state").toString();
            activeSpaceSecurityMode = payload.value("active_space_security_mode").toString();
            activeSpacePreferredApp = payload.value("active_space_preferred_active_app").toString();
            activeSpaceApps = payload.value("active_space_apps").toString().split(',', Qt::SkipEmptyParts);
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
                app.insert("in_active_space", activeSpaceApps.contains(parts.value(0)));
                sessionApps.insert(parts.value(0), app);
            }
        } else {
            sessionState = "error";
            sessionHealth = "error";
        }
    }

    const QString updateStatePath = QDir::home().filePath(".velyx/update_state.json");
    QFile updateFile(updateStatePath);
    if (updateFile.open(QIODevice::ReadOnly)) {
        const QJsonDocument document = QJsonDocument::fromJson(updateFile.readAll());
        if (document.isObject()) {
            const QJsonObject object = document.object();
            currentVersion = object.value("current_version").toString();
            updateState = object.value("update_state").toString(updateState);
            lastUpdateResult = object.value("last_update_result").toString();
            recoveryNeeded = object.value("recovery_needed").toBool(false);
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
    if (m_currentVersion != currentVersion) {
        m_currentVersion = currentVersion;
        changed = true;
    }
    if (m_updateState != updateState) {
        m_updateState = updateState;
        changed = true;
    }
    if (m_lastUpdateResult != lastUpdateResult) {
        m_lastUpdateResult = lastUpdateResult;
        changed = true;
    }
    if (m_recoveryNeeded != recoveryNeeded) {
        m_recoveryNeeded = recoveryNeeded;
        changed = true;
    }
    if (m_activeSpaceId != activeSpaceId) {
        m_activeSpaceId = activeSpaceId;
        changed = true;
    }
    if (m_activeSpaceName != activeSpaceName) {
        m_activeSpaceName = activeSpaceName;
        changed = true;
    }
    if (m_activeSpaceState != activeSpaceState) {
        m_activeSpaceState = activeSpaceState;
        changed = true;
    }
    if (m_activeSpaceSecurityMode != activeSpaceSecurityMode) {
        m_activeSpaceSecurityMode = activeSpaceSecurityMode;
        changed = true;
    }
    if (m_activeSpacePreferredApp != activeSpacePreferredApp) {
        m_activeSpacePreferredApp = activeSpacePreferredApp;
        changed = true;
    }
    if (m_activeSpaceApps != activeSpaceApps) {
        m_activeSpaceApps = activeSpaceApps;
        changed = true;
    }

    if (changed) {
        emit runtimeStatusChanged();
        emit spacesChanged();
        refreshIntents();
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
        updated.insert("in_active_space", sessionApp.value("in_active_space", false));
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

void PermissionClient::activateSpace(const QString &spaceId)
{
    if (spaceId.isEmpty()) {
        return;
    }

    QDBusInterface sessionManager(
        kSessionManagerService,
        kSessionManagerPath,
        kSessionManagerInterface,
        QDBusConnection::sessionBus());
    if (!sessionManager.isValid()) {
        updateLaunchState("error", "session-manager недоступен");
        updateStatusDetails("space_activate", "error", "session_manager_unavailable", "retry");
        refreshRuntimeStatus();
        return;
    }

    QDBusReply<QVariantMap> reply = sessionManager.call("ActivateSpace", spaceId);
    if (!reply.isValid()) {
        updateLaunchState("error", QString("Не удалось активировать space %1").arg(spaceId));
        updateStatusDetails("space_activate", "error", "space_activate_failed", "retry");
        return;
    }

    const QVariantMap payload = reply.value();
    updateStatusDetails(
        "space_activate",
        payload.value("active", "false").toString() == "true" ? "ok" : "pending",
        payload.value("runtime_state").toString(),
        "refresh_space");
    updateLaunchState(
        "ok",
        QString("Активирован space %1").arg(payload.value("display_name", spaceId).toString()));
    refreshRuntimeStatus();
    refreshSpaces();
    refreshApps();
    refreshOpenApps();

    const QString preferredApp = payload.value("preferred_active_app").toString();
    if (!preferredApp.isEmpty()) {
        for (const QVariant &entry : std::as_const(m_openApps)) {
            const QVariantMap app = entry.toMap();
            if (app.value("app_id").toString() == preferredApp) {
                selectActiveApp(preferredApp);
                return;
            }
        }
    }

    for (const QVariant &entry : std::as_const(m_openApps)) {
        const QVariantMap app = entry.toMap();
        if (app.value("in_active_space").toBool() && app.value("state").toString() == "running") {
            selectActiveApp(app.value("app_id").toString());
            return;
        }
    }
}

void PermissionClient::runIntent(const QString &intentId)
{
    if (intentId.isEmpty()) {
        return;
    }

    QString output;
    QProcess process;
    process.start("velyx-intent", { "run", intentId });
    if (process.waitForStarted(400) && process.waitForFinished(3000)) {
        output = QString::fromUtf8(process.readAllStandardOutput()).trimmed();
        const QString errorOutput = QString::fromUtf8(process.readAllStandardError()).trimmed();
        if (process.exitStatus() == QProcess::NormalExit && process.exitCode() == 0) {
            updateLaunchState("ok", output.isEmpty() ? QString("Intent %1 выполнен").arg(intentId) : output);
            updateStatusDetails("intent_run", "ok", intentId, "space_ready");
            refreshRuntimeStatus();
            refreshSpaces();
            refreshIntents();
            refreshApps();
            refreshOpenApps();
            return;
        }
        updateLaunchState("error", errorOutput.isEmpty() ? QString("Не удалось выполнить intent %1").arg(intentId) : errorOutput);
        updateStatusDetails("intent_run", "failed", intentId, "retry");
        refreshIntents();
        return;
    }

    for (const QVariant &entry : std::as_const(m_intents)) {
        const QVariantMap intent = entry.toMap();
        if (intent.value("intent_id").toString() == intentId) {
            const QString spaceId = intent.value("target_space").toString();
            if (!spaceId.isEmpty()) {
                activateSpace(spaceId);
                updateStatusDetails("intent_run", "fallback", intentId, "space_activate");
                updateLaunchState("ok", QString("Intent %1 переведён в activateSpace(%2)").arg(intentId, spaceId));
                return;
            }
        }
    }

    updateLaunchState("error", QString("Intent %1 недоступен").arg(intentId));
    updateStatusDetails("intent_run", "failed", intentId, "retry");
}

void PermissionClient::runRule(const QString &ruleId)
{
    if (ruleId.isEmpty()) {
        return;
    }

    QProcess process;
    process.start("velyx-rule", {"run", ruleId});
    if (process.waitForStarted(400) && process.waitForFinished(3000)) {
        const QString output = QString::fromUtf8(process.readAllStandardOutput()).trimmed();
        const QString errorOutput = QString::fromUtf8(process.readAllStandardError()).trimmed();
        if (process.exitStatus() == QProcess::NormalExit && process.exitCode() == 0) {
            updateLaunchState("ok", output.isEmpty() ? QString("Rule %1 выполнен").arg(ruleId) : output);
            updateStatusDetails("rule_run", "ok", ruleId, "refresh_runtime");
            refreshRules();
            refreshRuntimeStatus();
            refreshSpaces();
            refreshIntents();
            refreshApps();
            refreshOpenApps();
            return;
        }
        updateLaunchState("error", errorOutput.isEmpty() ? QString("Не удалось выполнить rule %1").arg(ruleId) : errorOutput);
        updateStatusDetails("rule_run", "failed", ruleId, "retry");
        refreshRules();
        return;
    }

    updateLaunchState("error", QString("Rule %1 недоступен").arg(ruleId));
    updateStatusDetails("rule_run", "failed", ruleId, "retry");
}

void PermissionClient::runAgentCommand(const QString &command)
{
    const QString trimmed = command.trimmed();
    if (trimmed.isEmpty()) {
        return;
    }

    QProcess process;
    process.start("velyx-agent", {"command", trimmed});
    if (process.waitForStarted(400) && process.waitForFinished(5000)) {
        const QString output = QString::fromUtf8(process.readAllStandardOutput()).trimmed();
        const QString errorOutput = QString::fromUtf8(process.readAllStandardError()).trimmed();
        if (process.exitStatus() == QProcess::NormalExit && process.exitCode() == 0) {
            updateLaunchState("ok", output.isEmpty() ? QString("Agent command выполнен: %1").arg(trimmed) : output);
            updateStatusDetails("agent_command", "ok", trimmed, "refresh_runtime");
            refreshAgentState();
            refreshAiState();
            refreshAssistantState();
            refreshRuntimeStatus();
            refreshSpaces();
            refreshIntents();
            refreshRules();
            refreshApps();
            refreshOpenApps();
            return;
        }
        updateLaunchState("error", errorOutput.isEmpty() ? QString("Не удалось выполнить agent command: %1").arg(trimmed) : errorOutput);
        updateStatusDetails("agent_command", "failed", trimmed, "retry");
        refreshAgentState();
        refreshAiState();
        refreshAssistantState();
        return;
    }

    updateLaunchState("error", QString("Agent layer недоступен для команды: %1").arg(trimmed));
    updateStatusDetails("agent_command", "failed", trimmed, "retry");
}

void PermissionClient::setAiMode(const QString &mode)
{
    const QString trimmed = mode.trimmed();
    if (trimmed.isEmpty()) {
        return;
    }

    QProcess process;
    process.start("velyx-ai", {"set-mode", trimmed});
    if (process.waitForStarted(400) && process.waitForFinished(3000)) {
        const QString output = QString::fromUtf8(process.readAllStandardOutput()).trimmed();
        const QString errorOutput = QString::fromUtf8(process.readAllStandardError()).trimmed();
        refreshAiState();
        if (process.exitStatus() == QProcess::NormalExit && process.exitCode() == 0) {
            updateLaunchState("ok", output.isEmpty() ? QString("AI mode switched to %1").arg(trimmed) : output);
            updateStatusDetails("ai_mode", "ok", trimmed, "observe_suggestion");
            return;
        }
        updateLaunchState("error", errorOutput.isEmpty() ? QString("Не удалось включить AI mode %1").arg(trimmed) : errorOutput);
        updateStatusDetails("ai_mode", "failed", trimmed, "retry");
        return;
    }

    updateLaunchState("error", QString("AI layer недоступен для mode %1").arg(trimmed));
    updateStatusDetails("ai_mode", "failed", trimmed, "retry");
}

void PermissionClient::runAiSummary()
{
    QProcess process;
    process.start("velyx-ai", {"summary"});
    if (process.waitForStarted(400) && process.waitForFinished(5000)) {
        const QString output = QString::fromUtf8(process.readAllStandardOutput()).trimmed();
        const QString errorOutput = QString::fromUtf8(process.readAllStandardError()).trimmed();
        refreshAiState();
        if (process.exitStatus() == QProcess::NormalExit && process.exitCode() == 0) {
            updateLaunchState("ok", output.isEmpty() ? "AI summary generated" : output);
            updateStatusDetails("ai_summary", "ok", m_aiMode, "review");
            return;
        }
        updateLaunchState("error", errorOutput.isEmpty() ? "Не удалось получить AI summary" : errorOutput);
        updateStatusDetails("ai_summary", "failed", m_aiMode, "retry");
        return;
    }

    updateLaunchState("error", "AI summary unavailable");
    updateStatusDetails("ai_summary", "failed", "ai_unavailable", "retry");
}

void PermissionClient::runAiExplain()
{
    QProcess process;
    process.start("velyx-ai", {"explain"});
    if (process.waitForStarted(400) && process.waitForFinished(5000)) {
        const QString output = QString::fromUtf8(process.readAllStandardOutput()).trimmed();
        const QString errorOutput = QString::fromUtf8(process.readAllStandardError()).trimmed();
        refreshAiState();
        if (process.exitStatus() == QProcess::NormalExit && process.exitCode() == 0) {
            updateLaunchState("ok", output.isEmpty() ? "AI explanation generated" : output);
            updateStatusDetails("ai_explain", "ok", m_aiMode, "review");
            return;
        }
        updateLaunchState("error", errorOutput.isEmpty() ? "Не удалось получить AI explanation" : errorOutput);
        updateStatusDetails("ai_explain", "failed", m_aiMode, "retry");
        return;
    }

    updateLaunchState("error", "AI explanation unavailable");
    updateStatusDetails("ai_explain", "failed", "ai_unavailable", "retry");
}

void PermissionClient::runAiSuggest()
{
    QProcess process;
    process.start("velyx-ai", {"suggest"});
    if (process.waitForStarted(400) && process.waitForFinished(5000)) {
        const QString output = QString::fromUtf8(process.readAllStandardOutput()).trimmed();
        const QString errorOutput = QString::fromUtf8(process.readAllStandardError()).trimmed();
        refreshAiState();
        refreshRuntimeStatus();
        refreshSpaces();
        refreshIntents();
        refreshRules();
        refreshAgentState();
        refreshApps();
        refreshOpenApps();
        if (process.exitStatus() == QProcess::NormalExit && process.exitCode() == 0) {
            updateLaunchState("ok", output.isEmpty() ? "AI suggestion updated" : output);
            updateStatusDetails("ai_suggest", "ok", m_aiSuggestionActionType, m_aiSuggestionAvailable ? "review_suggestion" : "none");
            return;
        }
        updateLaunchState("error", errorOutput.isEmpty() ? "Не удалось получить AI suggestion" : errorOutput);
        updateStatusDetails("ai_suggest", "failed", m_aiMode, "retry");
        return;
    }

    updateLaunchState("error", "AI suggestion unavailable");
    updateStatusDetails("ai_suggest", "failed", "ai_unavailable", "retry");
}

void PermissionClient::applyAiSuggestion()
{
    QProcess process;
    process.start("velyx-ai", {"run-last-suggestion"});
    if (process.waitForStarted(400) && process.waitForFinished(6000)) {
        const QString output = QString::fromUtf8(process.readAllStandardOutput()).trimmed();
        const QString errorOutput = QString::fromUtf8(process.readAllStandardError()).trimmed();
        refreshAiState();
        refreshRuntimeStatus();
        refreshSpaces();
        refreshIntents();
        refreshRules();
        refreshAgentState();
        refreshApps();
        refreshOpenApps();
        if (process.exitStatus() == QProcess::NormalExit && process.exitCode() == 0) {
            updateLaunchState("ok", output.isEmpty() ? "AI suggestion applied" : output);
            updateStatusDetails("ai_apply", "ok", m_aiSuggestionActionType, "observe_result");
            return;
        }
        updateLaunchState("error", errorOutput.isEmpty() ? "Не удалось применить AI suggestion" : errorOutput);
        updateStatusDetails("ai_apply", "failed", m_aiSuggestionActionType, "retry");
        return;
    }

    updateLaunchState("error", "AI suggestion apply unavailable");
    updateStatusDetails("ai_apply", "failed", "ai_unavailable", "retry");
}

void PermissionClient::dismissAiSuggestion()
{
    QProcess process;
    process.start("velyx-ai", {"dismiss-last-suggestion"});
    if (process.waitForStarted(400) && process.waitForFinished(3000)) {
        const QString output = QString::fromUtf8(process.readAllStandardOutput()).trimmed();
        const QString errorOutput = QString::fromUtf8(process.readAllStandardError()).trimmed();
        refreshAiState();
        if (process.exitStatus() == QProcess::NormalExit && process.exitCode() == 0) {
            updateLaunchState("ok", output.isEmpty() ? "AI suggestion dismissed" : output);
            updateStatusDetails("ai_dismiss", "ok", "dismissed", "none");
            return;
        }
        updateLaunchState("error", errorOutput.isEmpty() ? "Не удалось dismiss AI suggestion" : errorOutput);
        updateStatusDetails("ai_dismiss", "failed", "dismiss", "retry");
        return;
    }

    updateLaunchState("error", "AI dismissal unavailable");
    updateStatusDetails("ai_dismiss", "failed", "ai_unavailable", "retry");
}

void PermissionClient::blockAiSuggestion()
{
    QProcess process;
    process.start("velyx-ai", {"block-last-suggestion"});
    if (process.waitForStarted(400) && process.waitForFinished(3000)) {
        const QString output = QString::fromUtf8(process.readAllStandardOutput()).trimmed();
        const QString errorOutput = QString::fromUtf8(process.readAllStandardError()).trimmed();
        refreshAiState();
        if (process.exitStatus() == QProcess::NormalExit && process.exitCode() == 0) {
            updateLaunchState("ok", output.isEmpty() ? "AI suggestion type blocked" : output);
            updateStatusDetails("ai_block", "ok", m_aiSuggestionActionType, "none");
            return;
        }
        updateLaunchState("error", errorOutput.isEmpty() ? "Не удалось заблокировать AI suggestion" : errorOutput);
        updateStatusDetails("ai_block", "failed", m_aiSuggestionActionType, "retry");
        return;
    }

    updateLaunchState("error", "AI suggestion block unavailable");
    updateStatusDetails("ai_block", "failed", "ai_unavailable", "retry");
}

void PermissionClient::setModelSelectionMode(const QString &mode)
{
    const QString trimmed = mode.trimmed();
    if (trimmed.isEmpty()) {
        return;
    }

    QProcess process;
    process.start("velyx-model", {"set-selection-mode", trimmed});
    if (process.waitForStarted(400) && process.waitForFinished(6000)) {
        const QString output = QString::fromUtf8(process.readAllStandardOutput()).trimmed();
        const QString errorOutput = QString::fromUtf8(process.readAllStandardError()).trimmed();
        refreshAiState();
        if (process.exitStatus() == QProcess::NormalExit && process.exitCode() == 0) {
            updateLaunchState("ok", output.isEmpty() ? QString("Model selection set to %1").arg(trimmed) : output);
            updateStatusDetails("model_selection_mode", "ok", trimmed, "observe_runtime");
            return;
        }
        updateLaunchState("error", errorOutput.isEmpty() ? QString("Не удалось переключить model selection mode: %1").arg(trimmed) : errorOutput);
        updateStatusDetails("model_selection_mode", "failed", trimmed, "retry");
        return;
    }

    updateLaunchState("error", "Model runtime unavailable");
    updateStatusDetails("model_selection_mode", "failed", trimmed, "retry");
}

void PermissionClient::detectModelHardware()
{
    QProcess process;
    process.start("velyx-model", {"detect-hardware"});
    if (process.waitForStarted(400) && process.waitForFinished(6000)) {
        const QString output = QString::fromUtf8(process.readAllStandardOutput()).trimmed();
        const QString errorOutput = QString::fromUtf8(process.readAllStandardError()).trimmed();
        refreshAiState();
        if (process.exitStatus() == QProcess::NormalExit && process.exitCode() == 0) {
            updateLaunchState("ok", output.isEmpty() ? "Hardware profile updated" : output);
            updateStatusDetails("model_detect_hardware", "ok", m_aiSelectionMode, "review");
            return;
        }
        updateLaunchState("error", errorOutput.isEmpty() ? "Не удалось определить hardware profile" : errorOutput);
        updateStatusDetails("model_detect_hardware", "failed", "detect", "retry");
        return;
    }

    updateLaunchState("error", "Model hardware detection unavailable");
    updateStatusDetails("model_detect_hardware", "failed", "detect", "retry");
}

void PermissionClient::setFirstBootAiMode(const QString &mode)
{
    QProcess process;
    process.start("velyx-firstboot", {"set-ai-mode", mode.trimmed()});
    if (process.waitForStarted(400) && process.waitForFinished(5000)) {
        refreshFirstBootState();
        refreshAiState();
        updateLaunchState(process.exitCode() == 0 ? "ok" : "error", process.exitCode() == 0 ? "First Boot AI mode updated" : QString::fromUtf8(process.readAllStandardError()).trimmed());
        updateStatusDetails("first_boot_ai_mode", process.exitCode() == 0 ? "ok" : "failed", mode, "review");
    }
}

void PermissionClient::setFirstBootStep(const QString &step)
{
    QProcess process;
    process.start("velyx-firstboot", {"set-step", step.trimmed()});
    if (process.waitForStarted(400) && process.waitForFinished(4000)) {
        refreshFirstBootState();
    }
}

void PermissionClient::setFirstBootModelSelectionMode(const QString &mode)
{
    QProcess process;
    process.start("velyx-firstboot", {"set-model-selection", mode.trimmed()});
    if (process.waitForStarted(400) && process.waitForFinished(6000)) {
        refreshFirstBootState();
        refreshAiState();
        updateLaunchState(process.exitCode() == 0 ? "ok" : "error", process.exitCode() == 0 ? "First Boot model selection updated" : QString::fromUtf8(process.readAllStandardError()).trimmed());
        updateStatusDetails("first_boot_model_selection", process.exitCode() == 0 ? "ok" : "failed", mode, "review");
    }
}

void PermissionClient::setFirstBootDefaultSpace(const QString &spaceId)
{
    QProcess process;
    process.start("velyx-firstboot", {"set-default-space", spaceId.trimmed()});
    if (process.waitForStarted(400) && process.waitForFinished(5000)) {
        refreshFirstBootState();
        updateLaunchState(process.exitCode() == 0 ? "ok" : "error", process.exitCode() == 0 ? "Default space updated" : QString::fromUtf8(process.readAllStandardError()).trimmed());
        updateStatusDetails("first_boot_default_space", process.exitCode() == 0 ? "ok" : "failed", spaceId, "review");
    }
}

void PermissionClient::setFirstBootPredictiveMode(const QString &mode)
{
    QProcess process;
    process.start("velyx-firstboot", {"set-predictive", mode.trimmed()});
    if (process.waitForStarted(400) && process.waitForFinished(5000)) {
        refreshFirstBootState();
        updateLaunchState(process.exitCode() == 0 ? "ok" : "error", process.exitCode() == 0 ? "Predictive mode updated" : QString::fromUtf8(process.readAllStandardError()).trimmed());
        updateStatusDetails("first_boot_predictive", process.exitCode() == 0 ? "ok" : "failed", mode, "review");
    }
}

void PermissionClient::rerunFirstBootChecks()
{
    QProcess process;
    process.start("velyx-firstboot", {"rerun-checks"});
    if (process.waitForStarted(400) && process.waitForFinished(6000)) {
        refreshFirstBootState();
        refreshRuntimeStatus();
        refreshAiState();
        updateLaunchState(process.exitCode() == 0 ? "ok" : "error", process.exitCode() == 0 ? "Velyx checks refreshed" : QString::fromUtf8(process.readAllStandardError()).trimmed());
        updateStatusDetails("first_boot_checks", process.exitCode() == 0 ? "ok" : "failed", "first_boot", "review");
    }
}

void PermissionClient::completeFirstBoot()
{
    QProcess process;
    process.start("velyx-firstboot", {"complete"});
    if (process.waitForStarted(400) && process.waitForFinished(5000)) {
        refreshFirstBootState();
        updateLaunchState(process.exitCode() == 0 ? "ok" : "error", process.exitCode() == 0 ? "Welcome to Velyx." : QString::fromUtf8(process.readAllStandardError()).trimmed());
        updateStatusDetails("first_boot_complete", process.exitCode() == 0 ? "ok" : "failed", m_firstBootDefaultSpace, "continue_work");
    }
}

void PermissionClient::runRecoveryFlow()
{
    QProcess process;
    process.start("velyx-recovery");
    if (process.waitForStarted(400) && process.waitForFinished(12000)) {
        refreshRuntimeStatus();
        refreshFirstBootState();
        updateLaunchState(process.exitCode() == 0 ? "ok" : "error", process.exitCode() == 0 ? "Velyx Recovery completed" : QString::fromUtf8(process.readAllStandardError()).trimmed());
        updateStatusDetails("recovery", process.exitCode() == 0 ? "ok" : "failed", "recovery", "review");
    }
}

void PermissionClient::exportDiagnostics()
{
    QProcess process;
    process.start("velyx-diagnostics", {"export"});
    if (process.waitForStarted(400) && process.waitForFinished(10000)) {
        const QString output = QString::fromUtf8(process.readAllStandardOutput()).trimmed();
        updateLaunchState(process.exitCode() == 0 ? "ok" : "error", output.isEmpty() ? "Diagnostics export finished" : output);
        updateStatusDetails("diagnostics_export", process.exitCode() == 0 ? "ok" : "failed", "diagnostics", "review");
    }
}

void PermissionClient::askAssistant(const QString &query)
{
    const QString trimmed = query.trimmed();
    if (trimmed.isEmpty()) {
        return;
    }

    QProcess process;
    process.start("velyx-assistant", {"ask", trimmed});
    if (process.waitForStarted(400) && process.waitForFinished(12000)) {
        const QString output = QString::fromUtf8(process.readAllStandardOutput()).trimmed();
        const QString errorOutput = QString::fromUtf8(process.readAllStandardError()).trimmed();
        refreshAssistantState();
        refreshAiState();
        refreshDevModeState();
        refreshAgentState();
        refreshRuntimeStatus();
        refreshSpaces();
        refreshIntents();
        refreshRules();
        refreshApps();
        refreshOpenApps();
        if (process.exitStatus() == QProcess::NormalExit && process.exitCode() == 0) {
            updateLaunchState("ok", output.isEmpty() ? "Assistant request handled" : output);
            updateStatusDetails("assistant_ask", "ok", trimmed, m_assistantPendingApproval ? "review_approval" : "review_result");
            return;
        }
        updateLaunchState("error", errorOutput.isEmpty() ? "Assistant request failed" : errorOutput);
        updateStatusDetails("assistant_ask", "failed", trimmed, "retry");
        return;
    }

    updateLaunchState("error", "Assistant backend unavailable");
    updateStatusDetails("assistant_ask", "failed", trimmed, "retry");
}

void PermissionClient::approveAssistant(const QString &requestId)
{
    const QString trimmed = requestId.trimmed();
    if (trimmed.isEmpty()) {
        return;
    }

    QProcess process;
    process.start("velyx-assistant", {"approve", trimmed});
    if (process.waitForStarted(400) && process.waitForFinished(12000)) {
        const QString output = QString::fromUtf8(process.readAllStandardOutput()).trimmed();
        const QString errorOutput = QString::fromUtf8(process.readAllStandardError()).trimmed();
        refreshAssistantState();
        refreshAiState();
        refreshDevModeState();
        refreshAgentState();
        refreshRuntimeStatus();
        refreshSpaces();
        refreshIntents();
        refreshRules();
        refreshApps();
        refreshOpenApps();
        if (process.exitStatus() == QProcess::NormalExit && process.exitCode() == 0) {
            updateLaunchState("ok", output.isEmpty() ? "Assistant approval applied" : output);
            updateStatusDetails("assistant_approve", "ok", trimmed, "observe_result");
            return;
        }
        updateLaunchState("error", errorOutput.isEmpty() ? "Assistant approval failed" : errorOutput);
        updateStatusDetails("assistant_approve", "failed", trimmed, "retry");
        return;
    }

    updateLaunchState("error", "Assistant approval unavailable");
    updateStatusDetails("assistant_approve", "failed", trimmed, "retry");
}

void PermissionClient::denyAssistant(const QString &requestId)
{
    const QString trimmed = requestId.trimmed();
    if (trimmed.isEmpty()) {
        return;
    }

    QProcess process;
    process.start("velyx-assistant", {"deny", trimmed});
    if (process.waitForStarted(400) && process.waitForFinished(5000)) {
        const QString output = QString::fromUtf8(process.readAllStandardOutput()).trimmed();
        const QString errorOutput = QString::fromUtf8(process.readAllStandardError()).trimmed();
        refreshAssistantState();
        refreshDevModeState();
        if (process.exitStatus() == QProcess::NormalExit && process.exitCode() == 0) {
            updateLaunchState("ok", output.isEmpty() ? "Assistant action denied" : output);
            updateStatusDetails("assistant_deny", "ok", trimmed, "none");
            return;
        }
        updateLaunchState("error", errorOutput.isEmpty() ? "Assistant deny failed" : errorOutput);
        updateStatusDetails("assistant_deny", "failed", trimmed, "retry");
        return;
    }

    updateLaunchState("error", "Assistant deny unavailable");
    updateStatusDetails("assistant_deny", "failed", trimmed, "retry");
}

void PermissionClient::setAssistantMode(const QString &mode)
{
    const QString trimmed = mode.trimmed();
    if (trimmed.isEmpty()) {
        return;
    }

    QProcess process;
    process.start("velyx-assistant", {"set-mode", trimmed});
    if (process.waitForStarted(400) && process.waitForFinished(5000)) {
        const QString output = QString::fromUtf8(process.readAllStandardOutput()).trimmed();
        const QString errorOutput = QString::fromUtf8(process.readAllStandardError()).trimmed();
        refreshAssistantState();
        refreshAiState();
        if (process.exitStatus() == QProcess::NormalExit && process.exitCode() == 0) {
            updateLaunchState("ok", output.isEmpty() ? QString("Assistant mode switched to %1").arg(trimmed) : output);
            updateStatusDetails("assistant_mode", "ok", trimmed, "observe_assistant");
            return;
        }
        updateLaunchState("error", errorOutput.isEmpty() ? QString("Failed to switch assistant mode to %1").arg(trimmed) : errorOutput);
        updateStatusDetails("assistant_mode", "failed", trimmed, "retry");
        return;
    }

    updateLaunchState("error", "Assistant mode control unavailable");
    updateStatusDetails("assistant_mode", "failed", trimmed, "retry");
}

void PermissionClient::enableDevMode()
{
    QProcess process;
    process.start("velyx-dev", {"enable"});
    if (process.waitForStarted(400) && process.waitForFinished(5000)) {
        const QString output = QString::fromUtf8(process.readAllStandardOutput()).trimmed();
        const QString errorOutput = QString::fromUtf8(process.readAllStandardError()).trimmed();
        refreshDevModeState();
        if (process.exitStatus() == QProcess::NormalExit && process.exitCode() == 0) {
            updateLaunchState("ok", output.isEmpty() ? "Dev Mode enabled" : output);
            updateStatusDetails("dev_mode_enable", "ok", "dev_mode", "restart_shell");
            return;
        }
        updateLaunchState("error", errorOutput.isEmpty() ? "Failed to enable Dev Mode" : errorOutput);
        updateStatusDetails("dev_mode_enable", "failed", "dev_mode", "retry");
        return;
    }

    updateLaunchState("error", "Dev Mode control unavailable");
    updateStatusDetails("dev_mode_enable", "failed", "dev_mode", "retry");
}

void PermissionClient::disableDevMode()
{
    QProcess process;
    process.start("velyx-dev", {"disable"});
    if (process.waitForStarted(400) && process.waitForFinished(5000)) {
        const QString output = QString::fromUtf8(process.readAllStandardOutput()).trimmed();
        const QString errorOutput = QString::fromUtf8(process.readAllStandardError()).trimmed();
        refreshDevModeState();
        if (process.exitStatus() == QProcess::NormalExit && process.exitCode() == 0) {
            updateLaunchState("ok", output.isEmpty() ? "Dev Mode disabled" : output);
            updateStatusDetails("dev_mode_disable", "ok", "dev_mode", "observe_shell");
            return;
        }
        updateLaunchState("error", errorOutput.isEmpty() ? "Failed to disable Dev Mode" : errorOutput);
        updateStatusDetails("dev_mode_disable", "failed", "dev_mode", "retry");
        return;
    }

    updateLaunchState("error", "Dev Mode control unavailable");
    updateStatusDetails("dev_mode_disable", "failed", "dev_mode", "retry");
}

void PermissionClient::rollbackDevMode()
{
    QProcess process;
    process.start("velyx-dev", {"rollback"});
    if (process.waitForStarted(400) && process.waitForFinished(15000)) {
        const QString output = QString::fromUtf8(process.readAllStandardOutput()).trimmed();
        const QString errorOutput = QString::fromUtf8(process.readAllStandardError()).trimmed();
        refreshDevModeState();
        if (process.exitStatus() == QProcess::NormalExit && process.exitCode() == 0) {
            updateLaunchState("ok", output.isEmpty() ? "Dev overlay rolled back" : output);
            updateStatusDetails("dev_mode_rollback", "ok", "dev_overlay", "observe_shell");
            return;
        }
        updateLaunchState("error", errorOutput.isEmpty() ? "Failed to roll back dev overlay" : errorOutput);
        updateStatusDetails("dev_mode_rollback", "failed", "dev_overlay", "retry");
        return;
    }

    updateLaunchState("error", "Dev rollback unavailable");
    updateStatusDetails("dev_mode_rollback", "failed", "dev_overlay", "retry");
}

void PermissionClient::restartShellDev()
{
    QProcess process;
    process.start("velyx-dev", {"restart-shell-dev"});
    if (process.waitForStarted(400) && process.waitForFinished(15000)) {
        const QString output = QString::fromUtf8(process.readAllStandardOutput()).trimmed();
        const QString errorOutput = QString::fromUtf8(process.readAllStandardError()).trimmed();
        if (process.exitStatus() == QProcess::NormalExit && process.exitCode() == 0) {
            updateLaunchState("ok", output.isEmpty() ? "Shell restart requested" : output);
            updateStatusDetails("dev_mode_restart_shell", "ok", "dev_overlay", "observe_shell");
            return;
        }
        updateLaunchState("error", errorOutput.isEmpty() ? "Failed to restart shell" : errorOutput);
        updateStatusDetails("dev_mode_restart_shell", "failed", "dev_overlay", "retry");
        return;
    }

    updateLaunchState("error", "Shell dev restart unavailable");
    updateStatusDetails("dev_mode_restart_shell", "failed", "dev_overlay", "retry");
}

void PermissionClient::setDevAutoRefine(bool enabled)
{
    QProcess process;
    process.start("velyx-dev", {"set-auto-refine", enabled ? "true" : "false"});
    if (process.waitForStarted(400) && process.waitForFinished(5000)) {
        const QString output = QString::fromUtf8(process.readAllStandardOutput()).trimmed();
        const QString errorOutput = QString::fromUtf8(process.readAllStandardError()).trimmed();
        refreshDevModeState();
        if (process.exitStatus() == QProcess::NormalExit && process.exitCode() == 0) {
            updateLaunchState("ok", output.isEmpty() ? "Dev visual auto refine updated" : output);
            updateStatusDetails("dev_mode_auto_refine", "ok", enabled ? "enabled" : "disabled", "observe_visual_loop");
            return;
        }
        updateLaunchState("error", errorOutput.isEmpty() ? "Failed to change auto refine" : errorOutput);
        updateStatusDetails("dev_mode_auto_refine", "failed", enabled ? "enabled" : "disabled", "retry");
        return;
    }

    updateLaunchState("error", "Dev visual controls unavailable");
    updateStatusDetails("dev_mode_auto_refine", "failed", enabled ? "enabled" : "disabled", "retry");
}

void PermissionClient::applyNextDevRefinement()
{
    QProcess process;
    process.start("velyx-dev", {"apply-next-refinement"});
    if (process.waitForStarted(400) && process.waitForFinished(30000)) {
        const QString output = QString::fromUtf8(process.readAllStandardOutput()).trimmed();
        const QString errorOutput = QString::fromUtf8(process.readAllStandardError()).trimmed();
        refreshDevModeState();
        refreshAssistantState();
        if (process.exitStatus() == QProcess::NormalExit && process.exitCode() == 0) {
            updateLaunchState("ok", output.isEmpty() ? "Applied next visual refinement" : output);
            updateStatusDetails("dev_mode_apply_refinement", "ok", "dev_visual_loop", "observe_shell");
            return;
        }
        updateLaunchState("error", errorOutput.isEmpty() ? "Failed to apply next refinement" : errorOutput);
        updateStatusDetails("dev_mode_apply_refinement", "failed", "dev_visual_loop", "retry");
        return;
    }

    updateLaunchState("error", "Dev refinement control unavailable");
    updateStatusDetails("dev_mode_apply_refinement", "failed", "dev_visual_loop", "retry");
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
