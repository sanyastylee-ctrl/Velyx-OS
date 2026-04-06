import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Velyx.DesignSystem

Rectangle {
    id: root

    property string title: ""
    property string subtitle: ""
    property var appsModel: []
    required property var permissionClient

    radius: 22
    color: "#111722"
    border.width: 1
    border.color: Qt.rgba(1, 1, 1, 0.08)

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: 18
        spacing: 12

        ColumnLayout {
            Layout.fillWidth: true
            spacing: 4

            Label {
                text: root.title
                color: "#f3f6fb"
                font.pixelSize: 18
                font.weight: Font.DemiBold
            }

            Label {
                text: root.subtitle
                color: "#8f99ad"
                font.pixelSize: 12
            }
        }

        Label {
            visible: root.appsModel.length === 0
            text: "No running apps in this section"
            color: "#7f8aa0"
            font.pixelSize: 13
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
