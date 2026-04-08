import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Velyx.DesignSystem

Rectangle {
    id: root

    required property var permissionClient

    color: "transparent"

    property string draftUsername: ""
    property string draftAiMode: permissionClient.firstBootAiMode === "hybrid" ? "hybrid" : "local"
    property string draftModelSelection: permissionClient.firstBootModelSelectionMode.length > 0 ? permissionClient.firstBootModelSelectionMode : "auto_hardware"
    property string draftBackend: permissionClient.aiRuntimeBackend === "ollama-compatible"
        ? "ollama"
        : (permissionClient.aiRuntimeBackend.length > 0 ? permissionClient.aiRuntimeBackend : "stub")

    function goToStep(stepName) {
        permissionClient.setFirstBootStep(stepName)
    }

    function completeSetup() {
        permissionClient.completeFirstBootSetup(draftUsername, draftAiMode)
    }

    Rectangle {
        anchors.centerIn: parent
        width: Math.min(parent.width - Theme.space8 * 2, 860)
        height: Math.min(parent.height - Theme.space8 * 2, 620)
        radius: Theme.radiusXl
        color: Theme.shellSurfaceRaised
        border.width: 1
        border.color: Theme.shellStrokeStrong

        ColumnLayout {
            anchors.fill: parent
            anchors.margins: Theme.space7
            spacing: Theme.space5

            RowLayout {
                Layout.fillWidth: true

                ColumnLayout {
                    Layout.fillWidth: true
                    spacing: 4

                    Label {
                        text: "VELYX OS"
                        color: Theme.accentCoolStrong
                        font.pixelSize: 12
                        font.weight: Font.DemiBold
                        font.letterSpacing: 2
                    }

                    Label {
                        text: "Velyx First Boot"
                        color: Theme.textPrimary
                        font.family: Theme.fontDisplay
                        font.pixelSize: 30
                        font.weight: Font.DemiBold
                    }
                }

                RowLayout {
                    spacing: Theme.space2

                    Repeater {
                        model: [
                            { id: "welcome", label: "1" },
                            { id: "setup", label: "2" },
                            { id: "ready", label: "3" }
                        ]

                        delegate: Rectangle {
                            required property var modelData
                            width: 34
                            height: 34
                            radius: 17
                            color: permissionClient.firstBootStep === modelData.id
                                ? Theme.accentCool
                                : Qt.rgba(1, 1, 1, 0.06)
                            border.width: 1
                            border.color: permissionClient.firstBootStep === modelData.id
                                ? Theme.accentCoolStrong
                                : Theme.shellStroke

                            Label {
                                anchors.centerIn: parent
                                text: parent.modelData.label
                                color: permissionClient.firstBootStep === parent.modelData.id
                                    ? Theme.windowBg
                                    : Theme.textSecondary
                                font.pixelSize: 13
                                font.weight: Font.DemiBold
                            }
                        }
                    }
                }
            }

            Rectangle {
                Layout.fillWidth: true
                Layout.fillHeight: true
                radius: Theme.radiusLg
                color: Theme.shellSurface
                border.width: 1
                border.color: Theme.shellStroke

                Item {
                    anchors.fill: parent
                    anchors.margins: Theme.space7

                    ColumnLayout {
                        anchors.fill: parent
                        spacing: Theme.space5
                        visible: permissionClient.firstBootStep === "welcome"

                        Item { Layout.fillHeight: true }

                        Label {
                            Layout.alignment: Qt.AlignHCenter
                            text: "Welcome to Velyx OS"
                            color: Theme.textPrimary
                            font.family: Theme.fontDisplay
                            font.pixelSize: 42
                            font.weight: Font.DemiBold
                        }

                        Label {
                            Layout.alignment: Qt.AlignHCenter
                            text: "A system that evolves with you"
                            color: Theme.textSecondary
                            font.pixelSize: 17
                        }

                        Button {
                            Layout.alignment: Qt.AlignHCenter
                            text: "Start Setup →"
                            onClicked: root.goToStep("setup")
                        }

                        Item { Layout.fillHeight: true }
                    }

                    ColumnLayout {
                        anchors.fill: parent
                        spacing: Theme.space5
                        visible: permissionClient.firstBootStep === "setup"

                        Label {
                            text: "Setup"
                            color: Theme.textPrimary
                            font.family: Theme.fontDisplay
                            font.pixelSize: 34
                            font.weight: Font.DemiBold
                        }

                        Label {
                            Layout.fillWidth: true
                            text: "Choose a name for this Velyx environment and decide how AI should work on first launch."
                            color: Theme.textSecondary
                            wrapMode: Text.WordWrap
                        }

                        ColumnLayout {
                            Layout.fillWidth: true
                            spacing: Theme.space3

                            Label {
                                text: "Username (optional)"
                                color: Theme.textPrimary
                                font.pixelSize: 13
                                font.weight: Font.DemiBold
                            }

                            TextField {
                                id: usernameField
                                Layout.fillWidth: true
                                placeholderText: "Your name"
                                text: root.draftUsername
                                onTextChanged: root.draftUsername = text
                            }
                        }

                        ColumnLayout {
                            Layout.fillWidth: true
                            spacing: Theme.space3

                            Label {
                                text: "AI mode"
                                color: Theme.textPrimary
                                font.pixelSize: 13
                                font.weight: Font.DemiBold
                            }

                            RowLayout {
                                Layout.fillWidth: true
                                spacing: Theme.space3

                                Button {
                                    Layout.fillWidth: true
                                    text: "Local"
                                    highlighted: root.draftAiMode === "local"
                                    onClicked: root.draftAiMode = "local"
                                }

                                Button {
                                    Layout.fillWidth: true
                                    text: "Hybrid"
                                    highlighted: root.draftAiMode === "hybrid"
                                    onClicked: root.draftAiMode = "hybrid"
                                }
                            }

                            Label {
                                Layout.fillWidth: true
                                text: root.draftAiMode === "hybrid"
                                    ? "Hybrid uses local AI with optional internet-assisted features when the network is available."
                                    : "Local keeps Velyx focused on local model behavior and offline-safe assistant flows."
                                color: Theme.textMuted
                                wrapMode: Text.WordWrap
                            }
                        }

                        ColumnLayout {
                            Layout.fillWidth: true
                            spacing: Theme.space3

                            Label {
                                text: "Model selection"
                                color: Theme.textPrimary
                                font.pixelSize: 13
                                font.weight: Font.DemiBold
                            }

                            RowLayout {
                                Layout.fillWidth: true
                                spacing: Theme.space3

                                Button {
                                    Layout.fillWidth: true
                                    text: "Manual"
                                    highlighted: root.draftModelSelection === "manual"
                                    onClicked: {
                                        root.draftModelSelection = "manual"
                                        root.permissionClient.setFirstBootModelSelectionMode("manual")
                                    }
                                }

                                Button {
                                    Layout.fillWidth: true
                                    text: "Auto HW"
                                    highlighted: root.draftModelSelection === "auto_hardware"
                                    onClicked: {
                                        root.draftModelSelection = "auto_hardware"
                                        root.permissionClient.setFirstBootModelSelectionMode("auto_hardware")
                                    }
                                }

                                Button {
                                    Layout.fillWidth: true
                                    text: "Auto Task"
                                    highlighted: root.draftModelSelection === "auto_task"
                                    onClicked: {
                                        root.draftModelSelection = "auto_task"
                                        root.permissionClient.setFirstBootModelSelectionMode("auto_task")
                                    }
                                }
                            }
                        }

                        ColumnLayout {
                            Layout.fillWidth: true
                            spacing: Theme.space3

                            Label {
                                text: "Model backend"
                                color: Theme.textPrimary
                                font.pixelSize: 13
                                font.weight: Font.DemiBold
                            }

                            RowLayout {
                                Layout.fillWidth: true
                                spacing: Theme.space3

                                Button {
                                    Layout.fillWidth: true
                                    text: "Stub"
                                    highlighted: root.draftBackend === "stub"
                                    onClicked: {
                                        root.draftBackend = "stub"
                                        root.permissionClient.setFirstBootBackend("stub")
                                    }
                                }

                                Button {
                                    Layout.fillWidth: true
                                    text: "Ollama"
                                    highlighted: root.draftBackend === "ollama"
                                    onClicked: {
                                        root.draftBackend = "ollama"
                                        root.permissionClient.setFirstBootBackend("ollama")
                                    }
                                }

                                Button {
                                    Layout.fillWidth: true
                                    text: "OpenAI local"
                                    highlighted: root.draftBackend === "openai-compatible"
                                    onClicked: {
                                        root.draftBackend = "openai-compatible"
                                        root.permissionClient.setFirstBootBackend("openai-compatible")
                                    }
                                }
                            }
                        }

                        Item { Layout.fillHeight: true }

                        RowLayout {
                            Layout.fillWidth: true

                            Button {
                                text: "Back"
                                onClicked: root.goToStep("welcome")
                            }

                            Item { Layout.fillWidth: true }

                            Button {
                                text: "Continue"
                                onClicked: root.goToStep("ready")
                            }
                        }
                    }

                    ColumnLayout {
                        anchors.fill: parent
                        spacing: Theme.space5
                        visible: permissionClient.firstBootStep === "ready"

                        Label {
                            text: "Velyx is ready"
                            color: Theme.textPrimary
                            font.family: Theme.fontDisplay
                            font.pixelSize: 34
                            font.weight: Font.DemiBold
                        }

                        Label {
                            Layout.fillWidth: true
                            text: "Your system is set up and ready to open into the Velyx Shell experience."
                            color: Theme.textSecondary
                            wrapMode: Text.WordWrap
                        }

                        Rectangle {
                            Layout.fillWidth: true
                            radius: Theme.radiusLg
                            color: Qt.rgba(Theme.accentCool.r, Theme.accentCool.g, Theme.accentCool.b, 0.08)
                            border.width: 1
                            border.color: Theme.shellStroke
                            implicitHeight: 168

                            ColumnLayout {
                                anchors.fill: parent
                                anchors.margins: Theme.space5
                                spacing: Theme.space3

                                Label {
                                    text: "What you can do next"
                                    color: Theme.textPrimary
                                    font.pixelSize: 15
                                    font.weight: Font.DemiBold
                                }

                                Label {
                                    text: "• Ask your system to do things"
                                    color: Theme.textSecondary
                                    font.pixelSize: 14
                                }

                                Label {
                                    text: "• Customize UI in real time"
                                    color: Theme.textSecondary
                                    font.pixelSize: 14
                                }

                                Label {
                                    text: "• AI assistant is available"
                                    color: Theme.textSecondary
                                    font.pixelSize: 14
                                }

                                Label {
                                    text: "AI mode: " + (root.draftAiMode === "hybrid" ? "Hybrid" : "Local")
                                    color: Theme.textMuted
                                    font.pixelSize: 13
                                }
                            }
                        }

                        Item { Layout.fillHeight: true }

                        RowLayout {
                            Layout.fillWidth: true

                            Button {
                                text: "Back"
                                onClicked: root.goToStep("setup")
                            }

                            Item { Layout.fillWidth: true }

                            Button {
                                text: "Enter Velyx"
                                onClicked: root.completeSetup()
                            }
                        }
                    }
                }
            }
        }
    }
}
