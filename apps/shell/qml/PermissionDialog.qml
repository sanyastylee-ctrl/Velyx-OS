import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Velyx.DesignSystem
import Velyx.UI

Popup {
    id: root

    property string appId: ""
    property string appName: ""
    property string permission: ""
    property string permissionDisplayName: ""
    property string explanation: ""

    signal allowSelected(string appId, string appName, string permission)
    signal denySelected(string appId, string appName, string permission)

    modal: true
    focus: true
    dim: true
    closePolicy: Popup.NoAutoClose
    anchors.centerIn: Overlay.overlay
    width: 460
    height: 340
    padding: 0

    background: Card {
        fillColor: "#171d2b"
        borderColor: Theme.strokeSubtle
    }

    contentItem: ColumnLayout {
        anchors.fill: parent
        anchors.margins: Theme.space6
        spacing: Theme.space4

        Label {
            text: "Системный запрос разрешения"
            color: Theme.warning
            font.pixelSize: 12
            font.weight: Font.DemiBold
        }

        Label {
            text: root.appName
            color: Theme.textPrimary
            font.family: Theme.fontDisplay
            font.pixelSize: 28
            font.weight: Font.DemiBold
        }

        Label {
            text: "запрашивает: " + root.permissionDisplayName
            color: Theme.textSecondary
            font.pixelSize: 15
        }

        Label {
            text: root.explanation
            wrapMode: Text.WordWrap
            color: Theme.textSecondary
            font.pixelSize: 14
        }

        Card {
            Layout.fillWidth: true
            Layout.preferredHeight: 88
            fillColor: Theme.surface2

            ColumnLayout {
                anchors.fill: parent
                spacing: 4

                Label {
                    text: "App ID"
                    color: Theme.textMuted
                    font.pixelSize: 12
                }

                Label {
                    text: root.appId
                    color: Theme.textPrimary
                    font.pixelSize: 14
                }
            }
        }

        Item {
            Layout.fillHeight: true
        }

        RowLayout {
            Layout.fillWidth: true
            spacing: Theme.space3

            Button {
                Layout.fillWidth: true
                text: "Запретить"
                onClicked: root.denySelected(root.appId, root.appName, root.permission)
            }

            Button {
                Layout.fillWidth: true
                text: "Разрешить"
                onClicked: root.allowSelected(root.appId, root.appName, root.permission)
            }
        }
    }
}
