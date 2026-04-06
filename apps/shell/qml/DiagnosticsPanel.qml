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
    implicitHeight: 242

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: Theme.space4
        spacing: Theme.space3

        SectionHeader {
            Layout.fillWidth: true
            title: "Diagnostics"
            subtitle: "Repair, export state, or verify the runtime without dropping into a manual debug workflow."
        }

        Flow {
            Layout.fillWidth: true
            spacing: 8

            StatusChip { compact: true; label: "Launcher"; value: root.permissionClient.launcherAvailability; tone: root.permissionClient.launcherAvailability === "available" ? "success" : "warning" }
            StatusChip { compact: true; label: "Permissions"; value: root.permissionClient.permissionsAvailability; tone: root.permissionClient.permissionsAvailability === "available" ? "success" : "warning" }
            StatusChip { compact: true; label: "Session"; value: root.permissionClient.sessionAvailability; tone: root.permissionClient.sessionAvailability === "available" ? "success" : "warning" }
            StatusChip { compact: true; label: "Model"; value: root.permissionClient.aiModelAvailable ? "ready" : "degraded"; tone: root.permissionClient.aiModelAvailable ? "success" : "warning" }
        }

        Label {
            Layout.fillWidth: true
            text: root.permissionClient.recoveryNeeded
                ? "Recovery is currently recommended. Export diagnostics first if you want a snapshot before repair."
                : "Velyx can export a diagnostics bundle or enter recovery without leaving the shell."
            color: Theme.textSecondary
            wrapMode: Text.WordWrap
        }

        RowLayout {
            Layout.fillWidth: true
            spacing: 8

            Button { text: "Retry check"; onClicked: root.permissionClient.rerunFirstBootChecks() }
            Button { text: "Enter recovery"; onClicked: root.permissionClient.runRecoveryFlow() }
            Button { text: "Export diagnostics"; onClicked: root.permissionClient.exportDiagnostics() }
        }
    }
}
