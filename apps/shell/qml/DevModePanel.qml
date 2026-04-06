import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Velyx.DesignSystem
import Velyx.UI

Rectangle {
    id: root

    required property var permissionClient
    radius: Theme.radiusLg
    color: Qt.rgba(Theme.accentCool.r, Theme.accentCool.g, Theme.accentCool.b, 0.08)
    border.width: 1
    border.color: root.permissionClient.devModeEnabled ? Theme.accentCool : Theme.shellStroke
    implicitHeight: root.permissionClient.devModeEnabled ? 188 : 108

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: Theme.space4
        spacing: Theme.space3

        SectionHeader {
            Layout.fillWidth: true
            title: "Dev Mode"
            subtitle: root.permissionClient.devModeEnabled
                ? "Live UI editing is active. Velyx Shell will prefer the overlay layer."
                : "Hidden by default. Enable it only for controlled shell UI iteration."
        }

        RowLayout {
            Layout.fillWidth: true
            spacing: 8

            StatusChip {
                compact: true
                label: "State"
                value: root.permissionClient.devModeEnabled ? "Active" : "Off"
                tone: root.permissionClient.devModeEnabled ? "warning" : "neutral"
            }

            StatusChip {
                compact: true
                label: "Overlay"
                value: root.permissionClient.devModeEnabled ? "live" : "disabled"
                tone: root.permissionClient.devModeEnabled ? "accent" : "neutral"
            }
        }

        Label {
            Layout.fillWidth: true
            visible: root.permissionClient.devModeEnabled
            text: root.permissionClient.devLastChange.length > 0
                ? "Last UI change: " + root.permissionClient.devLastChange
                : "No live UI edits yet."
            color: Theme.textSecondary
            wrapMode: Text.WordWrap
            font.pixelSize: 12
        }

        Label {
            Layout.fillWidth: true
            visible: root.permissionClient.devModeEnabled
            text: root.permissionClient.devOverlayPath.length > 0
                ? "Overlay: " + root.permissionClient.devOverlayPath
                : ""
            color: Theme.textMuted
            wrapMode: Text.WordWrap
            font.pixelSize: 11
        }

        RowLayout {
            Layout.fillWidth: true
            spacing: 8

            Button {
                text: root.permissionClient.devModeEnabled ? "Disable" : "Enable"
                onClicked: root.permissionClient.devModeEnabled
                    ? root.permissionClient.disableDevMode()
                    : root.permissionClient.enableDevMode()
            }

            Button {
                text: "Rollback"
                enabled: root.permissionClient.devModeEnabled
                onClicked: root.permissionClient.rollbackDevMode()
            }

            Button {
                text: "Restart shell"
                enabled: root.permissionClient.devModeEnabled
                onClicked: root.permissionClient.restartShellDev()
            }
        }
    }
}
