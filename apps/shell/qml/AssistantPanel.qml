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
    implicitHeight: 520

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: Theme.space4
        spacing: Theme.space3

        SectionHeader {
            Layout.fillWidth: true
            title: "Assistant"
            subtitle: root.permissionClient.assistantMode === "off"
                ? "Assistant automation is off. You can still ask for summaries and safe actions."
                : "Say what you want. Velyx plans the steps, asks when needed, and returns the result."
        }

        Flow {
            Layout.fillWidth: true
            spacing: 8

            StatusChip {
                compact: true
                label: "Mode"
                value: root.permissionClient.assistantMode
                tone: root.permissionClient.assistantMode === "auto" ? "warning"
                    : (root.permissionClient.assistantMode === "suggest" ? "accent" : "neutral")
            }

            StatusChip {
                compact: true
                label: "State"
                value: root.permissionClient.assistantExecutionStatus
                tone: root.permissionClient.assistantPendingApproval ? "warning" : "neutral"
            }

            StatusChip {
                compact: true
                label: "AI"
                value: root.permissionClient.aiMode
                tone: root.permissionClient.aiModelAvailable ? "success" : "neutral"
            }
        }

        RowLayout {
            Layout.fillWidth: true
            spacing: 8

            Button { text: "Off"; onClicked: root.permissionClient.setAssistantMode("off") }
            Button { text: "Suggest"; onClicked: root.permissionClient.setAssistantMode("suggest") }
            Button { text: "Auto"; onClicked: root.permissionClient.setAssistantMode("auto") }
            Item { Layout.fillWidth: true }
            Button { text: "Explain"; onClicked: root.permissionClient.askAssistant("Explain the current system state") }
            Button { text: "Summary"; onClicked: root.permissionClient.askAssistant("Summarize the current system state") }
        }

        TextField {
            id: assistantInput
            Layout.fillWidth: true
            placeholderText: "Open the browser • Find the best Qt IDEs • Create a note in Documents"
            color: Theme.textPrimary
            placeholderTextColor: Theme.textMuted
            selectByMouse: true
            onAccepted: {
                if (text.trim().length > 0) {
                    root.permissionClient.askAssistant(text)
                    text = ""
                }
            }
        }

        Flow {
            Layout.fillWidth: true
            spacing: 8

            Button { text: "Open Browser"; onClicked: root.permissionClient.askAssistant("Open the browser") }
            Button { text: "Qt IDE Search"; onClicked: root.permissionClient.askAssistant("Find the best Qt IDEs on the internet") }
            Button { text: "Development"; onClicked: root.permissionClient.askAssistant("Switch me to development and open the browser") }
            Button { text: "Create Note"; onClicked: root.permissionClient.askAssistant("Create a note in Documents as markdown") }
        }

        Rectangle {
            Layout.fillWidth: true
            visible: root.permissionClient.assistantPendingApproval
            radius: Theme.radiusMd
            color: Qt.rgba(Theme.warning.r, Theme.warning.g, Theme.warning.b, 0.10)
            border.width: 1
            border.color: Theme.shellStroke
            implicitHeight: 116

            ColumnLayout {
                anchors.fill: parent
                anchors.margins: Theme.space4
                spacing: 8

                Label {
                    text: "Approval needed"
                    color: Theme.textPrimary
                    font.pixelSize: 13
                    font.weight: Font.DemiBold
                }

                Label {
                    Layout.fillWidth: true
                    text: root.permissionClient.assistantPendingSummary.length > 0
                        ? root.permissionClient.assistantPendingSummary
                        : "Velyx needs approval to continue."
                    color: Theme.textSecondary
                    wrapMode: Text.WordWrap
                }

                RowLayout {
                    Layout.fillWidth: true
                    spacing: 8

                    Button {
                        text: "Allow once"
                        onClicked: root.permissionClient.approveAssistant(root.permissionClient.assistantPendingRequestId)
                    }

                    Button {
                        text: "Deny"
                        onClicked: root.permissionClient.denyAssistant(root.permissionClient.assistantPendingRequestId)
                    }

                    Label {
                        Layout.fillWidth: true
                        text: root.permissionClient.assistantPendingDetails
                        color: Theme.textMuted
                        font.pixelSize: 11
                        horizontalAlignment: Text.AlignRight
                        wrapMode: Text.WordWrap
                    }
                }
            }
        }

        Rectangle {
            Layout.fillWidth: true
            radius: Theme.radiusMd
            color: Qt.rgba(Theme.accentCool.r, Theme.accentCool.g, Theme.accentCool.b, 0.08)
            border.width: 1
            border.color: Theme.shellStroke
            implicitHeight: 90

            ColumnLayout {
                anchors.fill: parent
                anchors.margins: Theme.space4
                spacing: 6

                Label {
                    text: root.permissionClient.assistantLastRequest.length > 0
                        ? "Last request: " + root.permissionClient.assistantLastRequest
                        : "Ask Velyx to search, explain, switch context, or save a note."
                    color: Theme.textSecondary
                    font.pixelSize: 11
                    wrapMode: Text.WordWrap
                }

                Label {
                    text: root.permissionClient.assistantLastResponse.length > 0
                        ? root.permissionClient.assistantLastResponse
                        : "Results appear here after the assistant responds."
                    color: Theme.textPrimary
                    font.pixelSize: 12
                    wrapMode: Text.WordWrap
                }
            }
        }

        ScrollView {
            Layout.fillWidth: true
            Layout.fillHeight: true
            clip: true

            Column {
                width: parent.width
                spacing: Theme.space2

                Repeater {
                    model: root.permissionClient.assistantHistory

                    Rectangle {
                        required property var modelData
                        width: parent.width
                        radius: Theme.radiusMd
                        color: modelData.role === "user"
                            ? Qt.rgba(1, 1, 1, 0.04)
                            : Qt.rgba(Theme.accentCool.r, Theme.accentCool.g, Theme.accentCool.b, 0.07)
                        border.width: 1
                        border.color: Theme.shellStroke
                        implicitHeight: historyColumn.implicitHeight + Theme.space4 * 2

                        Column {
                            id: historyColumn
                            anchors.fill: parent
                            anchors.margins: Theme.space4
                            spacing: 4

                            Label {
                                text: modelData.role === "user" ? "You" : "Velyx"
                                color: modelData.role === "user" ? Theme.textMuted : Theme.accentCoolStrong
                                font.pixelSize: 11
                                font.weight: Font.DemiBold
                            }

                            Label {
                                width: parent.width
                                text: modelData.text || ""
                                color: Theme.textPrimary
                                wrapMode: Text.WordWrap
                                font.pixelSize: 12
                            }

                            Label {
                                visible: (modelData.status || "").length > 0
                                text: modelData.status || ""
                                color: Theme.textMuted
                                font.pixelSize: 10
                            }
                        }
                    }
                }
            }
        }
    }
}
