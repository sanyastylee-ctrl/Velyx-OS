#include "aiclient.h"

#include <QDBusConnection>
#include <QDBusInterface>
#include <QDBusReply>
#include <QVariantMap>

namespace {
constexpr auto kAiService = "com.velyx.AI";
constexpr auto kAiPath = "/com/velyx/AI";
constexpr auto kAiInterface = "com.velyx.AI1";
}

AiClient::AiClient(QObject *parent)
    : QObject(parent)
{
}

QString AiClient::understoodIntent() const
{
    return m_understoodIntent;
}

QString AiClient::selectedTool() const
{
    return m_selectedTool;
}

QString AiClient::resultText() const
{
    return m_resultText;
}

QString AiClient::status() const
{
    return m_status;
}

QString AiClient::explanationSource() const
{
    return m_explanationSource;
}

QString AiClient::suggestedAction() const
{
    return m_suggestedAction;
}

bool AiClient::confirmationPending() const
{
    return m_confirmationPending;
}

QString AiClient::confirmationSummary() const
{
    return m_confirmationSummary;
}

QString AiClient::confirmationDetails() const
{
    return m_confirmationDetails;
}

QString AiClient::confirmationRisk() const
{
    return m_confirmationRisk;
}

QString AiClient::confirmationApp() const
{
    return m_confirmationApp;
}

QString AiClient::confirmationPermission() const
{
    return m_confirmationPermission;
}

void AiClient::submitCommand(const QString &text)
{
    QDBusInterface ai(kAiService, kAiPath, kAiInterface, QDBusConnection::sessionBus());
    if (!ai.isValid()) {
        setResult("error", "Unavailable", "none", "ai-service недоступен");
        return;
    }

    QDBusReply<QVariantMap> reply = ai.call("ProcessCommand", "shell-session", "user", text);
    if (!reply.isValid()) {
        setResult("error", "Unknown", "none", "Не удалось вызвать ai-service");
        return;
    }

    const QVariantMap payload = reply.value();
    if (payload.value("status").toString() == "confirmation_required") {
        setConfirmation(
            true,
            payload.value("action_id").toString(),
            payload.value("summary").toString(),
            payload.value("details").toString(),
            payload.value("risk_level").toString(),
            payload.value("affected_app").toString(),
            payload.value("affected_permission").toString());
    } else {
        setConfirmation(false, "", "", "", "", "", "");
    }
    setResult(
        payload.value("status", "unknown").toString(),
        payload.value("intent", "Unknown").toString(),
        payload.value("tool", "none").toString(),
        payload.value("message", "Пустой ответ AI").toString());
    m_explanationSource = payload.value("source", "system").toString();
    m_suggestedAction = payload.value("suggested_action", "").toString();
    emit resultChanged();
}

void AiClient::confirmPendingAction(bool accepted)
{
    if (!m_confirmationPending || m_confirmationActionId.isEmpty()) {
        return;
    }

    QDBusInterface ai(kAiService, kAiPath, kAiInterface, QDBusConnection::sessionBus());
    if (!ai.isValid()) {
        setResult("error", m_understoodIntent, m_selectedTool, "ai-service недоступен");
        return;
    }

    const QString decision = accepted ? "confirm" : "cancel";
    QDBusReply<QVariantMap> reply =
        ai.call("ConfirmAction", "shell-session", "user", m_confirmationActionId, decision);
    if (!reply.isValid()) {
        setResult("error", m_understoodIntent, m_selectedTool, "Не удалось завершить confirmation flow");
        return;
    }

    const QVariantMap payload = reply.value();
    setConfirmation(false, "", "", "", "", "", "");
    setResult(
        payload.value("status", "unknown").toString(),
        payload.value("intent", m_understoodIntent).toString(),
        payload.value("tool", m_selectedTool).toString(),
        payload.value("message", "Пустой ответ AI").toString());
    m_explanationSource = payload.value("source", "system").toString();
    m_suggestedAction = payload.value("suggested_action", "").toString();
    emit resultChanged();
}

void AiClient::setResult(
    const QString &status,
    const QString &intent,
    const QString &tool,
    const QString &result)
{
    m_status = status;
    m_understoodIntent = intent;
    m_selectedTool = tool;
    m_resultText = result;
    emit resultChanged();
}

void AiClient::setConfirmation(
    bool pending,
    const QString &actionId,
    const QString &summary,
    const QString &details,
    const QString &risk,
    const QString &app,
    const QString &permission)
{
    m_confirmationPending = pending;
    m_confirmationActionId = actionId;
    m_confirmationSummary = summary;
    m_confirmationDetails = details;
    m_confirmationRisk = risk;
    m_confirmationApp = app;
    m_confirmationPermission = permission;
    emit confirmationChanged();
}
