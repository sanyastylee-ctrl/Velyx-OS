import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Velyx.DesignSystem

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

    radius: 24
    color: "#111722"
    border.width: 1
    border.color: Qt.rgba(1, 1, 1, 0.08)

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: 18
        spacing: 12

        Label {
            text: "System"
            color: "#f3f6fb"
            font.pixelSize: 18
            font.weight: Font.DemiBold
        }

        Flow {
            Layout.fillWidth: true
            spacing: 10

            StatusChip {
                label: "Session"
                value: root.permissionClient.sessionState
                tone: root.toneForState(root.permissionClient.sessionState)
            }

            StatusChip {
                label: "Health"
                value: root.permissionClient.sessionHealth
                tone: root.toneForState(root.permissionClient.sessionHealth)
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
