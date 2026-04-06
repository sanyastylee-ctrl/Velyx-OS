import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

Rectangle {
    id: root

    required property var intent
    required property string lastIntentId
    required property string lastIntentResult
    signal runRequested(string intentId)

    radius: 16
    color: root.intent.intent_id === root.lastIntentId ? "#1b2433" : "#141a25"
    border.width: 1
    border.color: root.intent.intent_id === root.lastIntentId ? "#5b8cff" : Qt.rgba(1, 1, 1, 0.08)
    implicitHeight: 92

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: 14
        spacing: 8

        RowLayout {
            Layout.fillWidth: true

            ColumnLayout {
                Layout.fillWidth: true
                spacing: 2

                Label {
                    text: root.intent.display_name || root.intent.intent_id
                    color: "#f3f6fb"
                    font.pixelSize: 14
                    font.weight: Font.DemiBold
                }

                Label {
                    text: (root.intent.target_space || "-") + " • " + (root.intent.status || "enabled")
                    color: "#8f99ad"
                    font.pixelSize: 11
                }
            }

            Button {
                text: "Run"
                enabled: root.intent.status !== "disabled"
                onClicked: root.runRequested(root.intent.intent_id)
            }
        }

        Label {
            Layout.fillWidth: true
            text: root.intent.intent_id === root.lastIntentId && root.lastIntentResult.length > 0
                ? "Last result: " + root.lastIntentResult
                : (root.intent.description || "")
            color: "#a4afc3"
            font.pixelSize: 11
            wrapMode: Text.WordWrap
        }
    }
}
