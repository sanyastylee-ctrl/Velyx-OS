import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Velyx.DesignSystem
import Velyx.UI

Rectangle {
    id: root

    required property var permissionClient
    anchors.fill: parent
    color: Qt.rgba(0.03, 0.05, 0.08, 0.88)
    visible: root.permissionClient.firstBootRequired
    z: 100

    Rectangle {
        anchors.centerIn: parent
        width: Math.min(parent.width - Theme.space8 * 2, 1120)
        height: Math.min(parent.height - Theme.space8 * 2, 760)
        radius: Theme.radiusXl
        color: Theme.shellSurfaceRaised
        border.width: 1
        border.color: Theme.shellStrokeStrong

        RowLayout {
            anchors.fill: parent
            anchors.margins: Theme.space6
            spacing: Theme.space6

            Rectangle {
                Layout.preferredWidth: 300
                Layout.fillHeight: true
                radius: Theme.radiusLg
                color: Theme.shellSurface
                border.width: 1
                border.color: Theme.shellStroke

                ColumnLayout {
                    anchors.fill: parent
                    anchors.margins: Theme.space5
                    spacing: Theme.space4

                    Label {
                        text: "VELYX OS PREVIEW"
                        color: Theme.accentCoolStrong
                        font.pixelSize: 12
                        font.weight: Font.DemiBold
                        letterSpacing: 2
                    }

                    Label {
                        text: "Velyx First Boot"
                        color: Theme.textPrimary
                        font.family: Theme.fontDisplay
                        font.pixelSize: 28
                        font.weight: Font.DemiBold
                    }

                    Label {
                        text: "Finish setup, confirm system readiness, and enter Velyx Shell."
                        color: Theme.textSecondary
                        wrapMode: Text.WordWrap
                    }

                    Flow {
                        Layout.fillWidth: true
                        spacing: 8

                        StatusChip { compact: true; label: "Version"; value: root.permissionClient.firstBootVersion.length > 0 ? root.permissionClient.firstBootVersion : "preview"; tone: "accent" }
                        StatusChip { compact: true; label: "Install"; value: root.permissionClient.firstBootInstallMode; tone: "neutral" }
                        StatusChip { compact: true; label: "Network"; value: root.permissionClient.firstBootNetworkState; tone: root.permissionClient.firstBootNetworkState === "available" ? "success" : "warning" }
                    }

                    Repeater {
                        model: [
                            { step: "welcome", title: "1. Welcome" },
                            { step: "system", title: "2. System Ready" },
                            { step: "ai", title: "3. AI and Model" },
                            { step: "space", title: "4. Default Space" },
                            { step: "finish", title: "5. Enter Velyx" }
                        ]

                        delegate: Rectangle {
                            required property var modelData
                            Layout.fillWidth: true
                            radius: Theme.radiusMd
                            color: root.permissionClient.firstBootStep === modelData.step
                                ? Qt.rgba(Theme.accentCool.r, Theme.accentCool.g, Theme.accentCool.b, 0.14)
                                : Qt.rgba(1, 1, 1, 0.03)
                            border.width: 1
                            border.color: Theme.shellStroke
                            implicitHeight: 52

                            Label {
                                anchors.verticalCenter: parent.verticalCenter
                                anchors.left: parent.left
                                anchors.leftMargin: Theme.space4
                                text: modelData.title
                                color: Theme.textPrimary
                                font.pixelSize: 13
                                font.weight: Font.DemiBold
                            }

                            MouseArea {
                                anchors.fill: parent
                                onClicked: root.permissionClient.setFirstBootStep(parent.modelData.step)
                            }
                        }
                    }

                    Item { Layout.fillHeight: true }

                    Button {
                        text: "Run repair"
                        onClicked: root.permissionClient.runRecoveryFlow()
                    }

                    Button {
                        text: "Export diagnostics"
                        onClicked: root.permissionClient.exportDiagnostics()
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

                ScrollView {
                    anchors.fill: parent
                    anchors.margins: Theme.space5
                    clip: true

                    ColumnLayout {
                        width: parent.width
                        spacing: Theme.space5

                        Rectangle {
                            Layout.fillWidth: true
                            radius: Theme.radiusLg
                            color: Qt.rgba(Theme.accentCool.r, Theme.accentCool.g, Theme.accentCool.b, 0.10)
                            border.width: 1
                            border.color: Theme.shellStroke
                            implicitHeight: 150

                            ColumnLayout {
                                anchors.fill: parent
                                anchors.margins: Theme.space5
                                spacing: 8

                                Label {
                                    text: "Welcome to Velyx"
                                    color: Theme.textPrimary
                                    font.family: Theme.fontDisplay
                                    font.pixelSize: 26
                                    font.weight: Font.DemiBold
                                }

                                Label {
                                    Layout.fillWidth: true
                                    text: root.permissionClient.firstBootSystemReady
                                        ? "The preview environment is ready enough to enter Velyx Shell. You can still tune AI, model routing, and context defaults here."
                                        : "Velyx needs a quick readiness pass before normal use. You can re-run checks, export diagnostics, or enter recovery."
                                    color: Theme.textSecondary
                                    wrapMode: Text.WordWrap
                                }
                            }
                        }

                        SectionHeader {
                            Layout.fillWidth: true
                            title: "System Ready Check"
                            subtitle: "Confirm session health, update state, and connectivity before entering Velyx."
                        }

                        Flow {
                            Layout.fillWidth: true
                            spacing: 8

                            StatusChip { compact: true; label: "Session"; value: root.permissionClient.sessionState; tone: root.permissionClient.firstBootSystemReady ? "success" : "warning" }
                            StatusChip { compact: true; label: "Confidence"; value: root.permissionClient.sessionHealth; tone: root.permissionClient.sessionHealth === "ready" ? "success" : "warning" }
                            StatusChip { compact: true; label: "Update"; value: root.permissionClient.updateState; tone: root.permissionClient.recoveryNeeded ? "danger" : "neutral" }
                            StatusChip { compact: true; label: "Recovery"; value: root.permissionClient.recoveryNeeded ? "needed" : "available"; tone: root.permissionClient.recoveryNeeded ? "danger" : "success" }
                        }

                        RowLayout {
                            Layout.fillWidth: true
                            spacing: 8

                            Button {
                                text: "Retry check"
                                onClicked: {
                                    root.permissionClient.setFirstBootStep("system")
                                    root.permissionClient.rerunFirstBootChecks()
                                }
                            }

                            Button {
                                text: "Open diagnostics"
                                onClicked: root.permissionClient.exportDiagnostics()
                            }
                        }

                        SectionHeader {
                            Layout.fillWidth: true
                            title: "AI and Model Setup"
                            subtitle: "Choose how assistive and predictive Velyx should be in this preview install."
                        }

                        Flow {
                            Layout.fillWidth: true
                            spacing: 8

                            StatusChip { compact: true; label: "AI"; value: root.permissionClient.firstBootAiMode; tone: root.permissionClient.firstBootAiMode === "auto" ? "warning" : "accent" }
                            StatusChip { compact: true; label: "Selection"; value: root.permissionClient.firstBootModelSelectionMode; tone: "accent" }
                            StatusChip { compact: true; label: "Backend"; value: root.permissionClient.aiRuntimeBackend; tone: root.permissionClient.aiModelAvailable ? "success" : "warning" }
                            StatusChip { compact: true; label: "Model"; value: root.permissionClient.aiModelName.length > 0 ? root.permissionClient.aiModelName : "not ready"; tone: root.permissionClient.aiModelAvailable ? "success" : "warning" }
                        }

                        RowLayout {
                            Layout.fillWidth: true
                            spacing: 8

                            Button { text: "AI Off"; onClicked: { root.permissionClient.setFirstBootStep("ai"); root.permissionClient.setFirstBootAiMode("off") } }
                            Button { text: "AI Basic"; onClicked: { root.permissionClient.setFirstBootStep("ai"); root.permissionClient.setFirstBootAiMode("suggest") } }
                            Button { text: "AI Full"; onClicked: { root.permissionClient.setFirstBootStep("ai"); root.permissionClient.setFirstBootAiMode("auto") } }
                            Item { Layout.fillWidth: true }
                            Button { text: "Detect hardware"; onClicked: { root.permissionClient.setFirstBootStep("ai"); root.permissionClient.detectModelHardware(); root.permissionClient.rerunFirstBootChecks() } }
                        }

                        RowLayout {
                            Layout.fillWidth: true
                            spacing: 8

                            Button { text: "Manual"; onClicked: { root.permissionClient.setFirstBootStep("ai"); root.permissionClient.setFirstBootModelSelectionMode("manual") } }
                            Button { text: "Auto by hardware"; onClicked: { root.permissionClient.setFirstBootStep("ai"); root.permissionClient.setFirstBootModelSelectionMode("auto_hardware") } }
                            Button { text: "Auto by task"; onClicked: { root.permissionClient.setFirstBootStep("ai"); root.permissionClient.setFirstBootModelSelectionMode("auto_task") } }
                        }

                        SectionHeader {
                            Layout.fillWidth: true
                            title: "Default Space and Predictive Features"
                            subtitle: "Set the first context you want to land in and decide how proactive Velyx should be."
                        }

                        RowLayout {
                            Layout.fillWidth: true
                            spacing: 8

                            Button { text: "General"; onClicked: { root.permissionClient.setFirstBootStep("space"); root.permissionClient.setFirstBootDefaultSpace("general") } }
                            Button { text: "Development"; onClicked: { root.permissionClient.setFirstBootStep("space"); root.permissionClient.setFirstBootDefaultSpace("development") } }
                            Button { text: "Safe Web"; onClicked: { root.permissionClient.setFirstBootStep("space"); root.permissionClient.setFirstBootDefaultSpace("safe-web") } }
                            Button { text: "Recovery"; onClicked: { root.permissionClient.setFirstBootStep("space"); root.permissionClient.setFirstBootDefaultSpace("recovery") } }
                        }

                        RowLayout {
                            Layout.fillWidth: true
                            spacing: 8

                            Button { text: "Predictive Off"; onClicked: { root.permissionClient.setFirstBootStep("finish"); root.permissionClient.setFirstBootPredictiveMode("off") } }
                            Button { text: "Predictive Suggest"; onClicked: { root.permissionClient.setFirstBootStep("finish"); root.permissionClient.setFirstBootPredictiveMode("suggest") } }
                            Button { text: "Predictive Auto"; onClicked: { root.permissionClient.setFirstBootStep("finish"); root.permissionClient.setFirstBootPredictiveMode("auto") } }
                        }

                        Rectangle {
                            Layout.fillWidth: true
                            radius: Theme.radiusLg
                            color: Qt.rgba(1, 1, 1, 0.03)
                            border.width: 1
                            border.color: Theme.shellStroke
                            implicitHeight: 116

                            ColumnLayout {
                                anchors.fill: parent
                                anchors.margins: Theme.space5
                                spacing: 8

                                Label {
                                    text: "Enter Velyx"
                                    color: Theme.textPrimary
                                    font.pixelSize: 18
                                    font.weight: Font.DemiBold
                                }

                                Label {
                                    Layout.fillWidth: true
                                    text: "When you're ready, Velyx will treat this preview setup as complete and keep the shell as your primary session surface."
                                    color: Theme.textSecondary
                                    wrapMode: Text.WordWrap
                                }

                                RowLayout {
                                    Layout.fillWidth: true
                                    spacing: 8

                                    Button {
                                        text: "Enter Velyx"
                                        enabled: root.permissionClient.firstBootSystemReady || !root.permissionClient.recoveryNeeded
                                        onClicked: {
                                            root.permissionClient.setFirstBootStep("finish")
                                            root.permissionClient.completeFirstBoot()
                                        }
                                    }

                                    Button {
                                        text: "Later"
                                        onClicked: {
                                            root.permissionClient.setFirstBootStep("finish")
                                            root.permissionClient.completeFirstBoot()
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
