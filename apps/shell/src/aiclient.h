#pragma once

#include <QObject>
#include <QString>

class AiClient : public QObject
{
    Q_OBJECT
    Q_PROPERTY(QString understoodIntent READ understoodIntent NOTIFY resultChanged)
    Q_PROPERTY(QString selectedTool READ selectedTool NOTIFY resultChanged)
    Q_PROPERTY(QString resultText READ resultText NOTIFY resultChanged)
    Q_PROPERTY(QString status READ status NOTIFY resultChanged)
    Q_PROPERTY(QString explanationSource READ explanationSource NOTIFY resultChanged)
    Q_PROPERTY(QString suggestedAction READ suggestedAction NOTIFY resultChanged)
    Q_PROPERTY(bool confirmationPending READ confirmationPending NOTIFY confirmationChanged)
    Q_PROPERTY(QString confirmationSummary READ confirmationSummary NOTIFY confirmationChanged)
    Q_PROPERTY(QString confirmationDetails READ confirmationDetails NOTIFY confirmationChanged)
    Q_PROPERTY(QString confirmationRisk READ confirmationRisk NOTIFY confirmationChanged)
    Q_PROPERTY(QString confirmationApp READ confirmationApp NOTIFY confirmationChanged)
    Q_PROPERTY(QString confirmationPermission READ confirmationPermission NOTIFY confirmationChanged)

public:
    explicit AiClient(QObject *parent = nullptr);

    Q_INVOKABLE void submitCommand(const QString &text);
    Q_INVOKABLE void confirmPendingAction(bool accepted);

    QString understoodIntent() const;
    QString selectedTool() const;
    QString resultText() const;
    QString status() const;
    QString explanationSource() const;
    QString suggestedAction() const;
    bool confirmationPending() const;
    QString confirmationSummary() const;
    QString confirmationDetails() const;
    QString confirmationRisk() const;
    QString confirmationApp() const;
    QString confirmationPermission() const;

signals:
    void resultChanged();
    void confirmationChanged();

private:
    void setResult(
        const QString &status,
        const QString &intent,
        const QString &tool,
        const QString &result);
    void setConfirmation(
        bool pending,
        const QString &actionId,
        const QString &summary,
        const QString &details,
        const QString &risk,
        const QString &app,
        const QString &permission);

    QString m_understoodIntent = "Unknown";
    QString m_selectedTool = "none";
    QString m_resultText = "AI пока не обрабатывал команду.";
    QString m_status = "idle";
    QString m_explanationSource = "system";
    QString m_suggestedAction;
    bool m_confirmationPending = false;
    QString m_confirmationActionId;
    QString m_confirmationSummary;
    QString m_confirmationDetails;
    QString m_confirmationRisk;
    QString m_confirmationApp;
    QString m_confirmationPermission;
};
