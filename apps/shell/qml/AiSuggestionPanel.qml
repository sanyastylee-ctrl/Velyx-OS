import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Velyx.DesignSystem
import Velyx.UI

Rectangle {
    id: root

    required property var permissionClient
    radius: Theme.radiusLg
    color: Theme.shellSurfaceRaised
    border.width: 1
    border.color: Theme.shellStroke
    implicitHeight: 270

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: Theme.space4
        spacing: Theme.space3

        SectionHeader {
            Layout.fillWidth: true
            title: "Velyx AI"
            subtitle: root.permissionClient.aiMode === "off"
                ? "AI suggestions are experimental and currently off."
                : "AI stays inside safe system boundaries and never bypasses the operator layer."
        }

        Flow {
            Layout.fillWidth: true
            spacing: 8

            StatusChip {
                compact: true
                label: "Mode"
                value: root.permissionClient.aiMode
                tone: root.permissionClient.aiMode === "auto" ? "warning"
                    : (root.permissionClient.aiMode === "suggest" ? "accent" : "neutral")
            }

            StatusChip {
                compact: true
                label: "Model"
                value: root.permissionClient.aiModelName.length > 0 ? root.permissionClient.aiModelName : "unconfigured"
                tone: root.permissionClient.aiModelAvailable ? "success" : "warning"
            }
        }

        RowLayout {
            Layout.fillWidth: true
            spacing: 8

            Button { text: "Off"; onClicked: root.permissionClient.setAiMode("off") }
            Button { text: "Suggest"; onClicked: root.permissionClient.setAiMode("suggest") }
            Button { text: "Auto"; onClicked: root.permissionClient.setAiMode("auto") }
            Item { Layout.fillWidth: true }
            Button { text: "Explain"; onClicked: root.permissionClient.runAiExplain() }
            Button { text: "Suggest"; onClicked: root.permissionClient.runAiSuggest() }
        }

        Rectangle {
            Layout.fillWidth: true
            radius: Theme.radiusMd
            color: Qt.rgba(Theme.accentCool.r, Theme.accentCool.g, Theme.accentCool.b, 0.08)
            border.width: 1
            border.color: Theme.shellStroke
            implicitHeight: root.permissionClient.aiSuggestionAvailable ? 126 : 96

            ColumnLayout {
                anchors.fill: parent
                anchors.margins: Theme.space4
                spacing: 6

                Label {
                    text: root.permissionClient.aiSuggestionAvailable ? "Suggested next action" : "AI summary"
                    color: Theme.textMuted
                    font.pixelSize: 11
                }

                Label {
                    Layout.fillWidth: true
                    text: root.permissionClient.aiSuggestionAvailable
                        ? root.permissionClient.aiSuggestionMessage
                        : (root.permissionClient.aiLastSummary.length > 0 ? root.permissionClient.aiLastSummary : "No AI output yet.")
                    color: Theme.textPrimary
                    font.pixelSize: 12
                    wrapMode: Text.WordWrap
                }

                Label {
                    visible: root.permissionClient.aiSuggestionAvailable
                    text: "Action: " + root.permissionClient.aiSuggestionActionType
                        + "  •  Confidence: " + Number(root.permissionClient.aiSuggestionConfidence).toFixed(2)
                    color: Theme.textSecondary
                    font.pixelSize: 11
                }
            }
        }

        RowLayout {
            Layout.fillWidth: true
            spacing: 8
            visible: root.permissionClient.aiSuggestionAvailable

            Button {
                text: "Apply"
                onClicked: root.permissionClient.applyAiSuggestion()
            }

            Button {
                text: "Dismiss"
                onClicked: root.permissionClient.dismissAiSuggestion()
            }

            Button {
                text: "Never"
                onClicked: root.permissionClient.blockAiSuggestion()
            }

            Label {
                Layout.fillWidth: true
                text: root.permissionClient.aiSuggestionReason
                color: Theme.textMuted
                font.pixelSize: 11
                wrapMode: Text.WordWrap
            }
        }

        Label {
            visible: root.permissionClient.aiLastError.length > 0
            text: root.permissionClient.aiLastError
            color: Theme.warning
            font.pixelSize: 11
            wrapMode: Text.WordWrap
        }
    }
}
