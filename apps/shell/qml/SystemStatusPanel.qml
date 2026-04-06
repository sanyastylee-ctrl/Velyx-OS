import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Velyx.DesignSystem
import Velyx.UI

Rectangle {
    id: root

    required property var permissionClient

    function toneForState(value) {
        if (value === "ready" || value === "available" || value === "installed" || value === "running")
            return "success"
        if (value === "failed" || value === "unavailable" || value === "error")
            return "danger"
        if (value === "degraded" || value === "recovery" || value === "recovery_needed")
            return "warning"
        return "accent"
    }

    function confidenceTitle() {
        if (permissionClient.recoveryNeeded)
            return "Recovery required"
        if (permissionClient.sessionState === "ready" && permissionClient.sessionHealth === "ready")
            return "Ready"
        if (permissionClient.sessionState === "failed" || permissionClient.sessionHealth === "failed")
            return "Needs attention"
        return "Degraded"
    }

    function confidenceMessage() {
        if (permissionClient.recoveryNeeded)
            return "The runtime can still respond, but recovery should take priority before continuing work."
        if (permissionClient.sessionState === "ready" && permissionClient.launcherAvailability === "available" && permissionClient.permissionsAvailability === "available")
            return "Core services are up and the current context can continue safely."
        if (permissionClient.updateState === "failed")
            return "The last update needs attention before the system returns to full confidence."
        return "Velyx is available, but one or more subsystems need a closer look."
    }

    radius: Theme.radiusLg
    color: Theme.shellSurface
    border.width: 1
    border.color: Theme.shellStroke

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: Theme.space5
        spacing: Theme.space4

        RowLayout {
            Layout.fillWidth: true
            spacing: Theme.space4

            Rectangle {
                Layout.fillWidth: true
                Layout.preferredHeight: 102
                radius: Theme.radiusLg
                color: Qt.rgba(
                    (root.permissionClient.recoveryNeeded ? Theme.danger : (root.confidenceTitle() === "Ready" ? Theme.accentCool : Theme.warning)).r,
                    (root.permissionClient.recoveryNeeded ? Theme.danger : (root.confidenceTitle() === "Ready" ? Theme.accentCool : Theme.warning)).g,
                    (root.permissionClient.recoveryNeeded ? Theme.danger : (root.confidenceTitle() === "Ready" ? Theme.accentCool : Theme.warning)).b,
                    0.12
                )
                border.width: 1
                border.color: Qt.rgba(
                    (root.permissionClient.recoveryNeeded ? Theme.danger : (root.confidenceTitle() === "Ready" ? Theme.accentCool : Theme.warning)).r,
                    (root.permissionClient.recoveryNeeded ? Theme.danger : (root.confidenceTitle() === "Ready" ? Theme.accentCool : Theme.warning)).g,
                    (root.permissionClient.recoveryNeeded ? Theme.danger : (root.confidenceTitle() === "Ready" ? Theme.accentCool : Theme.warning)).b,
                    0.22
                )

                ColumnLayout {
                    anchors.fill: parent
                    anchors.margins: Theme.space4
                    spacing: 4

                    Label {
                        text: "System confidence"
                        color: Theme.textMuted
                        font.pixelSize: 11
                        font.weight: Font.Medium
                    }

                    Label {
                        text: root.confidenceTitle()
                        color: Theme.textPrimary
                        font.family: Theme.fontDisplay
                        font.pixelSize: 24
                        font.weight: Font.DemiBold
                    }

                    Label {
                        Layout.fillWidth: true
                        text: root.confidenceMessage()
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
                label: "Session"
                value: root.permissionClient.sessionState
                tone: root.toneForState(root.permissionClient.sessionState)
            }

            StatusChip {
                label: "Launcher"
                value: root.permissionClient.launcherAvailability
                tone: root.toneForState(root.permissionClient.launcherAvailability)
            }

            StatusChip {
                label: "Permissions"
                value: root.permissionClient.permissionsAvailability
                tone: root.toneForState(root.permissionClient.permissionsAvailability)
            }

            StatusChip {
                label: "Update"
                value: root.permissionClient.updateState
                tone: root.permissionClient.recoveryNeeded ? "danger" : root.toneForState(root.permissionClient.updateState)
            }

            StatusChip {
                label: "Runtime"
                value: root.permissionClient.currentVersion.length > 0 ? root.permissionClient.currentVersion : "unknown"
                tone: "accent"
            }
        }
    }
}
