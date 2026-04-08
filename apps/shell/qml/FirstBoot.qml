import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Velyx.DesignSystem

Rectangle {
    id: root

    required property var permissionClient
    readonly property bool hasPermissionClient: !!root.permissionClient
    readonly property var runtimeInfo: shellRuntime || ({})
    readonly property real uiScale: runtimeInfo.uiScale || 1.0
    readonly property bool compactMode: width < 980 || height < 760
    readonly property bool vmMode: runtimeInfo.environmentMode === "vm"
    readonly property int dialogMargin: Math.round((compactMode ? Theme.space4 : Theme.space8) * uiScale)
    readonly property int contentMargin: Math.round((compactMode ? Theme.space5 : Theme.space7) * uiScale)
    readonly property int sectionSpacing: Math.round((compactMode ? Theme.space4 : Theme.space5) * uiScale)
    readonly property int titlePixelSize: Math.round((compactMode ? 24 : 30) * uiScale)
    readonly property int heroPixelSize: Math.round((compactMode ? 34 : 42) * uiScale)
    readonly property int bodyPixelSize: Math.round((compactMode ? 15 : 17) * uiScale)
    readonly property int fieldPixelSize: Math.round(14 * uiScale)
    readonly property int buttonRowSpacing: Math.max(8, Math.round(Theme.space3 * uiScale))

    color: "transparent"
    focus: true

    function scaled(value) {
        return Math.round(value * root.uiScale)
    }

    property string draftUsername: ""
    property string draftAiMode: root.hasPermissionClient && permissionClient.firstBootAiMode === "hybrid" ? "hybrid" : "local"
    property string draftModelSelection: root.hasPermissionClient && permissionClient.firstBootModelSelectionMode.length > 0 ? permissionClient.firstBootModelSelectionMode : "auto_hardware"
    property string draftBackend: root.hasPermissionClient && permissionClient.aiRuntimeBackend === "ollama-compatible"
        ? "ollama"
        : (root.hasPermissionClient && permissionClient.aiRuntimeBackend.length > 0 ? permissionClient.aiRuntimeBackend : "stub")
    property double interactionStartedAtMs: 0
    property string pendingStep: ""
    readonly property string currentStep: pendingStep.length > 0 ? pendingStep
        : (root.hasPermissionClient ? permissionClient.firstBootStep : "welcome")

    function beginInteraction(tag) {
        root.interactionStartedAtMs = Date.now()
        console.info("firstboot interaction-start tag=" + tag + " at_ms=" + root.interactionStartedAtMs)
    }

    function finishInteraction(tag, callback) {
        const finishedAt = Date.now()
        const latency = root.interactionStartedAtMs > 0 ? finishedAt - root.interactionStartedAtMs : 0
        console.info("firstboot interaction-finish tag=" + tag + " latency_ms=" + latency)
        callback()
    }

    function goToStep(stepName) {
        if (root.hasPermissionClient) {
            root.pendingStep = stepName
            permissionClient.setFirstBootStep(stepName)
        }
    }

    function completeSetup() {
        if (root.hasPermissionClient)
            permissionClient.completeFirstBootSetup(draftUsername, draftAiMode)
    }

    Connections {
        target: root.permissionClient

        function onFirstBootStateChanged() {
            if (root.pendingStep.length > 0 && root.permissionClient.firstBootStep === root.pendingStep)
                root.pendingStep = ""
        }
    }

    HoverHandler {
        acceptedDevices: PointerDevice.Mouse | PointerDevice.TouchPad
        cursorShape: Qt.ArrowCursor
    }

    Rectangle {
        anchors.centerIn: parent
        width: Math.min(parent.width - root.dialogMargin * 2, root.compactMode ? 760 : 860)
        height: Math.min(parent.height - root.dialogMargin * 2, root.compactMode || root.vmMode ? 700 : 680)
        radius: Theme.radiusXl
        color: Theme.shellSurfaceRaised
        border.width: 1
        border.color: Theme.shellStrokeStrong

        ColumnLayout {
            anchors.fill: parent
            anchors.margins: root.contentMargin
            spacing: root.sectionSpacing

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
                        font.pixelSize: root.titlePixelSize
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
                            color: root.currentStep === modelData.id
                                ? Theme.accentCool
                                : Qt.rgba(1, 1, 1, 0.06)
                            border.width: 1
                            border.color: root.currentStep === modelData.id
                                ? Theme.accentCoolStrong
                                : Theme.shellStroke

                            Label {
                                anchors.centerIn: parent
                                text: parent.modelData.label
                                color: root.currentStep === parent.modelData.id
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
                    anchors.margins: root.contentMargin

                    ColumnLayout {
                        anchors.fill: parent
                        spacing: root.sectionSpacing
                        visible: root.currentStep === "welcome"

                        Item { Layout.fillHeight: true }

                        Label {
                            Layout.alignment: Qt.AlignHCenter
                            text: "Welcome to Velyx OS"
                            color: Theme.textPrimary
                            font.family: Theme.fontDisplay
                            font.pixelSize: root.heroPixelSize
                            font.weight: Font.DemiBold
                        }

                        Label {
                            Layout.alignment: Qt.AlignHCenter
                            text: "A system that evolves with you"
                            color: Theme.textSecondary
                            font.pixelSize: root.bodyPixelSize
                        }

                        Button {
                            focusPolicy: Qt.StrongFocus
                            Layout.alignment: Qt.AlignHCenter
                            text: "Start Setup →"
                            onPressed: root.beginInteraction("start_setup")
                            onClicked: root.finishInteraction("start_setup", function() { root.goToStep("setup") })
                        }

                        Item { Layout.fillHeight: true }
                    }

                    ScrollView {
                        anchors.fill: parent
                        clip: true
                        visible: root.currentStep === "setup"

                        ScrollBar.vertical.policy: ScrollBar.AsNeeded

                        ColumnLayout {
                            width: parent.width
                            spacing: root.sectionSpacing

                            Label {
                                text: "Setup"
                                color: Theme.textPrimary
                                font.family: Theme.fontDisplay
                                font.pixelSize: root.scaled(root.compactMode ? 28 : 34)
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
                                spacing: root.buttonRowSpacing

                                Label {
                                    text: "Username (optional)"
                                    color: Theme.textPrimary
                                    font.pixelSize: root.fieldPixelSize
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
                                spacing: root.buttonRowSpacing

                                Label {
                                    text: "AI mode"
                                    color: Theme.textPrimary
                                    font.pixelSize: root.fieldPixelSize
                                    font.weight: Font.DemiBold
                                }

                                RowLayout {
                                    Layout.fillWidth: true
                                    spacing: root.buttonRowSpacing

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
                                spacing: root.buttonRowSpacing

                                Label {
                                    text: "Model selection"
                                    color: Theme.textPrimary
                                    font.pixelSize: root.fieldPixelSize
                                    font.weight: Font.DemiBold
                                }

                                RowLayout {
                                    Layout.fillWidth: true
                                    spacing: root.buttonRowSpacing

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
                                spacing: root.buttonRowSpacing

                                Label {
                                    text: "Model backend"
                                    color: Theme.textPrimary
                                    font.pixelSize: root.fieldPixelSize
                                    font.weight: Font.DemiBold
                                }

                                RowLayout {
                                    Layout.fillWidth: true
                                    spacing: root.buttonRowSpacing

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

                            RowLayout {
                                Layout.fillWidth: true
                                spacing: root.buttonRowSpacing
                                Layout.topMargin: root.scaled(4)

                                Button {
                                    text: "Back"
                                    onPressed: root.beginInteraction("back_to_welcome")
                                    onClicked: root.finishInteraction("back_to_welcome", function() { root.goToStep("welcome") })
                                }

                                Item { Layout.fillWidth: true }

                                Button {
                                    text: "Continue"
                                    onPressed: root.beginInteraction("continue_to_ready")
                                    onClicked: root.finishInteraction("continue_to_ready", function() { root.goToStep("ready") })
                                }
                            }
                        }
                    }

                    ColumnLayout {
                        anchors.fill: parent
                        spacing: root.sectionSpacing
                        visible: root.currentStep === "ready"

                        Label {
                            text: "Velyx is ready"
                            color: Theme.textPrimary
                            font.family: Theme.fontDisplay
                            font.pixelSize: root.scaled(root.compactMode ? 28 : 34)
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
