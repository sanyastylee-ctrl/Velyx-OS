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
    implicitHeight: 680

    property string updateSource: ""
    property string modelIdInput: ""

    function toneForState(value) {
        if (value === "ready" || value === "available" || value === "installed" || value === "active" || value === "ok" || value === "reachable")
            return "success"
        if (value === "not_required" || value === "local_only")
            return "neutral"
        if (value === "failed" || value === "error" || value === "unavailable")
            return "danger"
        if (value === "degraded" || value === "offline")
            return "warning"
        return "accent"
    }

    ScrollView {
        anchors.fill: parent
        clip: true

        ColumnLayout {
            width: parent.width
            spacing: Theme.space4

            Rectangle {
                Layout.fillWidth: true
                radius: Theme.radiusLg
                color: Qt.rgba(Theme.accentCool.r, Theme.accentCool.g, Theme.accentCool.b, 0.10)
                border.width: 1
                border.color: Theme.shellStroke
                implicitHeight: 108

                ColumnLayout {
                    anchors.fill: parent
                    anchors.margins: Theme.space4
                    spacing: 4

                    Label {
                        text: "Control Center"
                        color: Theme.textPrimary
                        font.family: Theme.fontDisplay
                        font.pixelSize: 24
                        font.weight: Font.DemiBold
                    }

                    Label {
                        text: "System, AI, network, development and recovery controls in one operational panel."
                        color: Theme.textSecondary
                        font.pixelSize: 12
                        wrapMode: Text.WordWrap
                    }

                    Flow {
                        Layout.fillWidth: true
                        spacing: 8

                        StatusChip {
                            compact: true
                            label: "Version"
                            value: root.permissionClient.currentVersion.length > 0 ? root.permissionClient.currentVersion : "unknown"
                            tone: "accent"
                        }

                        StatusChip {
                            compact: true
                            label: "Update"
                            value: root.permissionClient.updateState
                            tone: root.toneForState(root.permissionClient.updateState)
                        }

                        StatusChip {
                            compact: true
                            label: "AI"
                            value: root.permissionClient.aiMode
                            tone: root.permissionClient.aiModelAvailable ? "success" : "warning"
                        }

                        StatusChip {
                            compact: true
                            label: "Dev"
                            value: root.permissionClient.devModeEnabled ? root.permissionClient.devAgentMode : "disabled"
                            tone: root.permissionClient.devModeEnabled ? "warning" : "neutral"
                        }
                    }
                }
            }

            Rectangle {
                Layout.fillWidth: true
                radius: Theme.radiusMd
                color: Theme.shellSurface
                border.width: 1
                border.color: Theme.shellStroke
                implicitHeight: 154

                ColumnLayout {
                    anchors.fill: parent
                    anchors.margins: Theme.space4
                    spacing: Theme.space3

                    Label {
                        text: "System"
                        color: Theme.textPrimary
                        font.pixelSize: 14
                        font.weight: Font.DemiBold
                    }

                    Flow {
                        Layout.fillWidth: true
                        spacing: 8

                        StatusChip { compact: true; label: "Version"; value: root.permissionClient.currentVersion.length > 0 ? root.permissionClient.currentVersion : "unknown"; tone: "accent" }
                        StatusChip { compact: true; label: "Uptime"; value: root.permissionClient.runtimeUptime; tone: "neutral" }
                        StatusChip { compact: true; label: "Install"; value: root.permissionClient.firstBootInstallMode.length > 0 ? root.permissionClient.firstBootInstallMode : "preview"; tone: "neutral" }
                        StatusChip { compact: true; label: "Session"; value: root.permissionClient.sessionState; tone: root.toneForState(root.permissionClient.sessionState) }
                        StatusChip { compact: true; label: "Health"; value: root.permissionClient.sessionHealth; tone: root.toneForState(root.permissionClient.sessionHealth) }
                        StatusChip { compact: true; label: "Update"; value: root.permissionClient.updateState; tone: root.toneForState(root.permissionClient.updateState) }
                    }

                    Label {
                        Layout.fillWidth: true
                        text: "Last update: " + (root.permissionClient.lastUpdateResult.length > 0 ? root.permissionClient.lastUpdateResult : "none")
                        color: Theme.textMuted
                        wrapMode: Text.WordWrap
                        font.pixelSize: 11
                    }
                }
            }

            Rectangle {
                Layout.fillWidth: true
                radius: Theme.radiusMd
                color: Theme.shellSurface
                border.width: 1
                border.color: Theme.shellStroke
                implicitHeight: 220

                ColumnLayout {
                    anchors.fill: parent
                    anchors.margins: Theme.space4
                    spacing: Theme.space3

                    Label {
                        text: "AI"
                        color: Theme.textPrimary
                        font.pixelSize: 14
                        font.weight: Font.DemiBold
                    }

                    Flow {
                        Layout.fillWidth: true
                        spacing: 8

                        StatusChip { compact: true; label: "Mode"; value: root.permissionClient.aiMode; tone: root.permissionClient.aiModelAvailable ? "success" : "warning" }
                        StatusChip { compact: true; label: "Model"; value: root.permissionClient.aiModelName.length > 0 ? root.permissionClient.aiModelName : "unconfigured"; tone: "accent" }
                        StatusChip { compact: true; label: "Backend"; value: root.permissionClient.aiRuntimeBackend.length > 0 ? root.permissionClient.aiRuntimeBackend : "stub"; tone: "neutral" }
                        StatusChip { compact: true; label: "Selection"; value: root.permissionClient.aiSelectionMode.length > 0 ? root.permissionClient.aiSelectionMode : "manual"; tone: "accent" }
                        StatusChip { compact: true; label: "Fallback"; value: root.permissionClient.aiFallbackReason.length > 0 ? root.permissionClient.aiFallbackReason : "none"; tone: root.permissionClient.aiFallbackReason.length > 0 ? "warning" : "neutral" }
                    }

                    RowLayout {
                        Layout.fillWidth: true
                        spacing: 8

                        Button { text: "Off"; onClicked: root.permissionClient.setAiMode("off") }
                        Button { text: "Suggest"; onClicked: root.permissionClient.setAiMode("suggest") }
                        Button { text: "Auto"; onClicked: root.permissionClient.setAiMode("auto") }
                    }

                    RowLayout {
                        Layout.fillWidth: true
                        spacing: 8

                        Button { text: "Stub"; onClicked: root.permissionClient.setModelBackend("stub") }
                        Button { text: "Ollama"; onClicked: root.permissionClient.setModelBackend("ollama") }
                        Button { text: "OpenAI local"; onClicked: root.permissionClient.setModelBackend("openai-compatible") }
                    }

                    RowLayout {
                        Layout.fillWidth: true
                        spacing: 8

                        Button { text: "Manual"; onClicked: root.permissionClient.setModelSelectionMode("manual") }
                        Button { text: "Auto HW"; onClicked: root.permissionClient.setModelSelectionMode("auto_hardware") }
                        Button { text: "Auto Task"; onClicked: root.permissionClient.setModelSelectionMode("auto_task") }
                        Button { text: "Re-detect"; onClicked: root.permissionClient.detectModelHardware() }
                    }

                    RowLayout {
                        Layout.fillWidth: true
                        spacing: 8

                        TextField {
                            Layout.fillWidth: true
                            placeholderText: "Model id, for example qwen-main-14b"
                            text: root.modelIdInput
                            onTextChanged: root.modelIdInput = text
                        }

                        Button {
                            text: "Use model"
                            enabled: root.modelIdInput.trim().length > 0
                            onClicked: root.permissionClient.setCurrentModel(root.modelIdInput)
                        }
                    }
                }
            }

            Rectangle {
                Layout.fillWidth: true
                radius: Theme.radiusMd
                color: Theme.shellSurface
                border.width: 1
                border.color: Theme.shellStroke
                implicitHeight: 164

                ColumnLayout {
                    anchors.fill: parent
                    anchors.margins: Theme.space4
                    spacing: Theme.space3

                    Label {
                        text: "Network"
                        color: Theme.textPrimary
                        font.pixelSize: 14
                        font.weight: Font.DemiBold
                    }

                    Flow {
                        Layout.fillWidth: true
                        spacing: 8

                        StatusChip { compact: true; label: "Network"; value: root.permissionClient.networkState; tone: root.toneForState(root.permissionClient.networkState) }
                        StatusChip { compact: true; label: "Update reachability"; value: root.permissionClient.networkUpdateReachability; tone: root.toneForState(root.permissionClient.networkUpdateReachability) }
                        StatusChip { compact: true; label: "AI backend"; value: root.permissionClient.networkAiBackendReachability; tone: root.toneForState(root.permissionClient.networkAiBackendReachability) }
                    }

                    Label {
                        Layout.fillWidth: true
                        visible: root.permissionClient.networkLastError.length > 0
                        text: "Network issue: " + root.permissionClient.networkLastError
                        color: Theme.textMuted
                        wrapMode: Text.WordWrap
                        font.pixelSize: 11
                    }

                    RowLayout {
                        Layout.fillWidth: true
                        spacing: 8

                        TextField {
                            Layout.fillWidth: true
                            placeholderText: "Update source, for example github or https://..."
                            text: root.updateSource
                            onTextChanged: root.updateSource = text
                        }

                        Button {
                            text: "Check"
                            onClicked: root.permissionClient.checkUpdateSource(root.updateSource)
                        }
                    }
                }
            }

            Rectangle {
                Layout.fillWidth: true
                radius: Theme.radiusMd
                color: Theme.shellSurface
                border.width: 1
                border.color: Theme.shellStroke
                implicitHeight: 178

                ColumnLayout {
                    anchors.fill: parent
                    anchors.margins: Theme.space4
                    spacing: Theme.space3

                    Label {
                        text: "Dev"
                        color: Theme.textPrimary
                        font.pixelSize: 14
                        font.weight: Font.DemiBold
                    }

                    Flow {
                        Layout.fillWidth: true
                        spacing: 8

                        StatusChip { compact: true; label: "Mode"; value: root.permissionClient.devModeEnabled ? root.permissionClient.devAgentMode : "disabled"; tone: root.permissionClient.devModeEnabled ? "warning" : "neutral" }
                        StatusChip { compact: true; label: "Auto refine"; value: root.permissionClient.devAutoRefine ? "on" : "off"; tone: root.permissionClient.devAutoRefine ? "warning" : "neutral" }
                        StatusChip { compact: true; label: "Status"; value: root.permissionClient.devApplyStrategy.length > 0 ? root.permissionClient.devApplyStrategy : "idle"; tone: "accent" }
                    }

                    RowLayout {
                        Layout.fillWidth: true
                        spacing: 8

                        Button {
                            text: root.permissionClient.devModeEnabled ? "Disable Dev" : "Enable Dev"
                            onClicked: root.permissionClient.devModeEnabled ? root.permissionClient.disableDevMode() : root.permissionClient.enableDevMode()
                        }

                        Button {
                            text: root.permissionClient.devAutoRefine ? "Auto refine on" : "Auto refine off"
                            enabled: root.permissionClient.devModeEnabled
                            onClicked: root.permissionClient.setDevAutoRefine(!root.permissionClient.devAutoRefine)
                        }

                        Button {
                            text: "Rollback"
                            enabled: root.permissionClient.devModeEnabled
                            onClicked: root.permissionClient.rollbackDevMode()
                        }
                    }
                }
            }

            Rectangle {
                Layout.fillWidth: true
                radius: Theme.radiusMd
                color: Theme.shellSurface
                border.width: 1
                border.color: Theme.shellStroke
                implicitHeight: 198

                ColumnLayout {
                    anchors.fill: parent
                    anchors.margins: Theme.space4
                    spacing: Theme.space3

                    Label {
                        text: "Recovery"
                        color: Theme.textPrimary
                        font.pixelSize: 14
                        font.weight: Font.DemiBold
                    }

                    Flow {
                        Layout.fillWidth: true
                        spacing: 8

                        StatusChip { compact: true; label: "Recovery"; value: root.permissionClient.recoveryNeeded ? "needed" : "ready"; tone: root.permissionClient.recoveryNeeded ? "danger" : "success" }
                        StatusChip { compact: true; label: "Diagnostics"; value: "available"; tone: "accent" }
                    }

                    RowLayout {
                        Layout.fillWidth: true
                        spacing: 8

                        Button {
                            text: "Export diagnostics"
                            onClicked: root.permissionClient.exportDiagnostics()
                        }

                        Button {
                            text: "Enter recovery"
                            onClicked: root.permissionClient.runRecoveryFlow()
                        }

                        Button {
                            text: "Apply update"
                            onClicked: root.permissionClient.runSystemUpdate(root.updateSource)
                        }
                    }
                }
            }
        }
    }
}
