import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Velyx.DesignSystem

Rectangle {
    id: root

    required property var permissionClient
    property int inSpaceCount: 0
    property int runningInSpaceCount: 0
    property int outsideCount: 0

    function stateTone() {
        if (root.permissionClient.activeSpaceState === "ready")
            return Theme.success
        if (root.permissionClient.activeSpaceState === "failed")
            return Theme.danger
        return Theme.warning
    }

    function confidenceTitle() {
        if (root.permissionClient.recoveryNeeded)
            return "Recovery required"
        if (root.permissionClient.activeSpaceState === "ready")
            return "Ready to work"
        if (root.permissionClient.activeSpaceState === "failed")
            return "Needs attention"
        return "Degraded"
    }

    function narrative() {
        if (root.permissionClient.recoveryNeeded)
            return "Velyx detected a recovery path for this environment. Enter recovery before continuing normal work."
        if (root.permissionClient.activeSpaceName.length === 0)
            return "Choose a space to give the system a clear working context."
        if (root.permissionClient.activeSpaceState === "ready")
            return "This context is aligned. Suggested actions and apps below are tuned for the current mode."
        if (root.permissionClient.activeSpaceState === "failed")
            return "Required apps for this space are not fully available. Use the actions below to recover the workspace."
        return "The context is available, but some optional pieces are still settling into place."
    }

    function primaryActionLabel() {
        if (root.permissionClient.recoveryNeeded)
            return "Open recovery"
        if (root.permissionClient.activeSpaceId === "development")
            return "Start development"
        if (root.permissionClient.activeSpaceId === "safe-web")
            return "Safe browse"
        if (root.permissionClient.activeSpacePreferredApp.length > 0)
            return "Continue work"
        return "Activate context"
    }

    function primaryAction() {
        if (root.permissionClient.recoveryNeeded) {
            root.permissionClient.runIntent("recovery_mode")
            return
        }
        if (root.permissionClient.activeSpaceId === "development") {
            root.permissionClient.runIntent("dev_start")
            return
        }
        if (root.permissionClient.activeSpaceId === "safe-web") {
            root.permissionClient.runIntent("safe_browse")
            return
        }
        if (root.permissionClient.activeSpacePreferredApp.length > 0) {
            root.permissionClient.selectActiveApp(root.permissionClient.activeSpacePreferredApp)
            return
        }
        if (root.permissionClient.activeSpaceId.length > 0)
            root.permissionClient.activateSpace(root.permissionClient.activeSpaceId)
    }

    radius: Theme.radiusXl
    color: Theme.shellSurface
    border.width: 1
    border.color: Qt.rgba(Theme.accentCool.r, Theme.accentCool.g, Theme.accentCool.b, 0.18)

    Rectangle {
        anchors.fill: parent
        radius: parent.radius
        gradient: Gradient {
            GradientStop { position: 0.0; color: Qt.rgba(Theme.accentCool.r, Theme.accentCool.g, Theme.accentCool.b, 0.13) }
            GradientStop { position: 0.48; color: Qt.rgba(Theme.accentBlue.r, Theme.accentBlue.g, Theme.accentBlue.b, 0.08) }
            GradientStop { position: 1.0; color: Qt.rgba(Theme.accentViolet.r, Theme.accentViolet.g, Theme.accentViolet.b, 0.05) }
        }
    }

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: Theme.space5
        spacing: Theme.space4

        RowLayout {
            Layout.fillWidth: true
            spacing: Theme.space4

            ColumnLayout {
                Layout.fillWidth: true
                spacing: 6

                Label {
                    text: root.permissionClient.activeSpaceName.length > 0
                        ? root.permissionClient.activeSpaceName
                        : "No active space"
                    color: Theme.textPrimary
                    font.family: Theme.fontDisplay
                    font.pixelSize: 32
                    font.weight: Font.DemiBold
                }

                Label {
                    text: root.permissionClient.activeSpaceId.length > 0
                        ? root.permissionClient.activeSpaceId
                        : "Select a context to begin"
                    color: Theme.accentCoolStrong
                    font.pixelSize: 13
                    font.weight: Font.Medium
                }

                Label {
                    Layout.fillWidth: true
                    text: root.narrative()
                    color: Theme.textSecondary
                    font.pixelSize: 13
                    wrapMode: Text.WordWrap
                }
            }

            Rectangle {
                Layout.preferredWidth: 210
                Layout.preferredHeight: 132
                radius: Theme.radiusLg
                color: Qt.rgba(root.stateTone().r, root.stateTone().g, root.stateTone().b, 0.12)
                border.width: 1
                border.color: Qt.rgba(root.stateTone().r, root.stateTone().g, root.stateTone().b, 0.24)

                ColumnLayout {
                    anchors.fill: parent
                    anchors.margins: Theme.space4
                    spacing: 6

                    Label {
                        text: "Context confidence"
                        color: Theme.textMuted
                        font.pixelSize: 11
                    }

                    Label {
                        text: root.confidenceTitle()
                        color: Theme.textPrimary
                        font.family: Theme.fontDisplay
                        font.pixelSize: 24
                        font.weight: Font.DemiBold
                    }

                    Label {
                        text: root.permissionClient.activeSpaceSecurityMode.length > 0
                            ? "Mode: " + root.permissionClient.activeSpaceSecurityMode
                            : "Mode not set"
                        color: Theme.textSecondary
                        font.pixelSize: 12
                    }

                    Label {
                        text: root.permissionClient.activeSpacePreferredApp.length > 0
                            ? "Preferred app: " + root.permissionClient.activeSpacePreferredApp
                            : "Preferred app unavailable"
                        color: Theme.textSecondary
                        font.pixelSize: 12
                        wrapMode: Text.WordWrap
                    }
                }
            }
        }

        Flow {
            Layout.fillWidth: true
            spacing: Theme.space3

            StatusChip {
                compact: true
                label: "Apps in space"
                value: root.inSpaceCount.toString()
                tone: "accent"
            }

            StatusChip {
                compact: true
                label: "Running here"
                value: root.runningInSpaceCount.toString()
                tone: root.runningInSpaceCount > 0 ? "success" : "warning"
            }

            StatusChip {
                compact: true
                label: "Outside current"
                value: root.outsideCount.toString()
                tone: root.outsideCount > 0 ? "warning" : "neutral"
            }
        }

        RowLayout {
            Layout.fillWidth: true
            spacing: Theme.space3

            Button {
                text: root.primaryActionLabel()
                onClicked: root.primaryAction()
            }

            Button {
                text: "Refresh context"
                enabled: root.permissionClient.activeSpaceId.length > 0
                onClicked: root.permissionClient.activateSpace(root.permissionClient.activeSpaceId)
            }

            Button {
                text: "Update system"
                onClicked: root.permissionClient.runAgentCommand("update")
            }

            Item { Layout.fillWidth: true }

            Label {
                text: root.permissionClient.lastIntentId.length > 0
                    ? "Last action: " + root.permissionClient.lastIntentId + " • " + root.permissionClient.lastIntentResult
                    : "Suggested actions below align the system to this context."
                color: Theme.textMuted
                font.pixelSize: 11
                horizontalAlignment: Text.AlignRight
                Layout.preferredWidth: 320
                wrapMode: Text.WordWrap
            }
        }
    }
}
