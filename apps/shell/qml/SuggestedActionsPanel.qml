import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Velyx.DesignSystem
import Velyx.UI

Rectangle {
    id: root

    required property var permissionClient
    property var intentsModel: []

    radius: Theme.radiusLg
    color: Theme.shellSurface
    border.width: 1
    border.color: Theme.shellStroke

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: Theme.space5
        spacing: Theme.space4

        SectionHeader {
            Layout.fillWidth: true
            title: "Suggested actions"
            subtitle: root.permissionClient.activeSpaceName.length > 0
                ? "Actions aligned with " + root.permissionClient.activeSpaceName
                : "Choose a context and Velyx will surface the next best actions."
        }

        Rectangle {
            Layout.fillWidth: true
            visible: root.intentsModel.length === 0
            radius: Theme.radiusMd
            color: Qt.rgba(1, 1, 1, 0.03)
            border.width: 1
            border.color: Theme.shellStroke
            implicitHeight: 86

            ColumnLayout {
                anchors.fill: parent
                anchors.margins: Theme.space4
                spacing: 4

                Label {
                    text: "No suggested actions"
                    color: Theme.textPrimary
                    font.pixelSize: 13
                    font.weight: Font.DemiBold
                }

                Label {
                    text: "Intents will appear here as spaces and system state become available."
                    color: Theme.textMuted
                    font.pixelSize: 11
                    wrapMode: Text.WordWrap
                }
            }
        }

        GridLayout {
            Layout.fillWidth: true
            visible: root.intentsModel.length > 0
            columns: width > 760 ? 2 : 1
            columnSpacing: Theme.space3
            rowSpacing: Theme.space3

            Repeater {
                model: root.intentsModel

                delegate: IntentCard {
                    Layout.fillWidth: true
                    intent: modelData
                    lastIntentId: root.permissionClient.lastIntentId
                    lastIntentResult: root.permissionClient.lastIntentResult
                    onRunRequested: root.permissionClient.runIntent(intentId)
                }
            }
        }
    }
}
