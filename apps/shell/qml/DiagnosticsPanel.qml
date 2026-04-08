import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Velyx.DesignSystem
import Velyx.UI

Rectangle {
    id: root

    required property var permissionClient
    readonly property bool hasPermissionClient: !!root.permissionClient
    readonly property string launcherAvailability: root.hasPermissionClient ? root.permissionClient.launcherAvailability : "checking"
    readonly property string permissionsAvailability: root.hasPermissionClient ? root.permissionClient.permissionsAvailability : "checking"
    readonly property string sessionAvailability: root.hasPermissionClient ? root.permissionClient.sessionAvailability : "checking"
    readonly property bool aiModelAvailable: root.hasPermissionClient && root.permissionClient.aiModelAvailable
    readonly property bool recoveryNeeded: root.hasPermissionClient && root.permissionClient.recoveryNeeded
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

            StatusChip { compact: true; label: "Launcher"; value: root.launcherAvailability; tone: root.launcherAvailability === "available" ? "success" : "warning" }
            StatusChip { compact: true; label: "Permissions"; value: root.permissionsAvailability; tone: root.permissionsAvailability === "available" ? "success" : "warning" }
            StatusChip { compact: true; label: "Session"; value: root.sessionAvailability; tone: root.sessionAvailability === "available" ? "success" : "warning" }
            StatusChip { compact: true; label: "Model"; value: root.aiModelAvailable ? "ready" : "degraded"; tone: root.aiModelAvailable ? "success" : "warning" }
        }

        Label {
            Layout.fillWidth: true
            text: root.recoveryNeeded
                ? "Recovery is currently recommended. Export diagnostics first if you want a snapshot before repair."
                : "Velyx can export a diagnostics bundle or enter recovery without leaving the shell."
            color: Theme.textSecondary
            wrapMode: Text.WordWrap
        }

        RowLayout {
            Layout.fillWidth: true
            spacing: 8

            Button { text: "Retry check"; enabled: root.hasPermissionClient; onClicked: root.permissionClient.rerunFirstBootChecks() }
            Button { text: "Enter recovery"; enabled: root.hasPermissionClient; onClicked: root.permissionClient.runRecoveryFlow() }
            Button { text: "Export diagnostics"; enabled: root.hasPermissionClient; onClicked: root.permissionClient.exportDiagnostics() }
        }
    }
}
