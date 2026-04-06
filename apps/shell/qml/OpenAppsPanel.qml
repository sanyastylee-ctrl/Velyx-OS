import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Velyx.DesignSystem
import Velyx.UI

Rectangle {
    id: root

    property string title: ""
    property string subtitle: ""
    property var appsModel: []
    required property var permissionClient

    radius: Theme.radiusLg
    color: Theme.shellSurface
    border.width: 1
    border.color: Theme.shellStroke

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: Theme.space5
        spacing: Theme.space3

        SectionHeader {
            Layout.fillWidth: true
            title: root.title
            subtitle: root.subtitle
        }

        Rectangle {
            Layout.fillWidth: true
            visible: root.appsModel.length === 0
            radius: Theme.radiusMd
            color: Qt.rgba(1, 1, 1, 0.03)
            border.width: 1
            border.color: Theme.shellStroke
            implicitHeight: 74

            ColumnLayout {
                anchors.fill: parent
                anchors.margins: Theme.space4
                spacing: 4

                Label {
                    text: "Nothing active here"
                    color: Theme.textPrimary
                    font.pixelSize: 13
                    font.weight: Font.DemiBold
                }

                Label {
                    text: "This section will fill as windows align with the current context."
                    color: Theme.textMuted
                    font.pixelSize: 11
                    wrapMode: Text.WordWrap
                }
            }
        }

        ListView {
            Layout.fillWidth: true
            Layout.fillHeight: true
            visible: root.appsModel.length > 0
            clip: true
            spacing: 10
            model: root.appsModel

            delegate: AppCard {
                width: ListView.view.width
                compact: true
                app: modelData
                selected: root.permissionClient.activeAppId === modelData.app_id
                onSelectRequested: root.permissionClient.selectApp(appId)
                onLaunchRequested: {
                    root.permissionClient.selectApp(appId)
                    root.permissionClient.launchSelectedApp()
                }
                onStopRequested: root.permissionClient.closeOpenApp(appId)
                onRestartRequested: root.permissionClient.restartOpenApp(appId)
                onActivateRequested: root.permissionClient.selectActiveApp(appId)
            }
        }
    }
}
