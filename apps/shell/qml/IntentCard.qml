import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Velyx.DesignSystem

Rectangle {
    id: root

    required property var intent
    required property string lastIntentId
    required property string lastIntentResult
    signal runRequested(string intentId)

    radius: Theme.radiusLg
    color: root.intent.intent_id === root.lastIntentId ? Theme.shellSurfaceOverlay : Theme.shellSurfaceRaised
    border.width: 1
    border.color: root.intent.intent_id === root.lastIntentId
        ? Qt.rgba(Theme.accentCool.r, Theme.accentCool.g, Theme.accentCool.b, 0.34)
        : Theme.shellStroke
    implicitHeight: 110

    Behavior on color {
        ColorAnimation { duration: Theme.motionBase }
    }

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: Theme.space4
        spacing: Theme.space3

        RowLayout {
            Layout.fillWidth: true
            spacing: Theme.space3

            ColumnLayout {
                Layout.fillWidth: true
                spacing: 2

                Label {
                    text: root.intent.display_name || root.intent.intent_id
                    color: Theme.textPrimary
                    font.family: Theme.fontSans
                    font.pixelSize: 15
                    font.weight: Font.DemiBold
                    elide: Text.ElideRight
                }

                Label {
                    text: (root.intent.target_space || "-") + "  •  " + (root.intent.status || "enabled")
                    color: Theme.textMuted
                    font.pixelSize: 11
                    elide: Text.ElideRight
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
                : (root.intent.description || "Move the system into the target context.")
            color: root.intent.intent_id === root.lastIntentId ? Theme.accentCoolStrong : Theme.textSecondary
            font.pixelSize: 12
            wrapMode: Text.WordWrap
        }
    }
}
