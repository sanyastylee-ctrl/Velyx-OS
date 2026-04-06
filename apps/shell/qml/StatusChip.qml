import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Velyx.DesignSystem

Rectangle {
    id: root

    property string label: ""
    property string value: ""
    property string tone: "neutral"
    property bool compact: false

    function toneColor() {
        if (tone === "success")
            return Theme.success
        if (tone === "warning")
            return Theme.warning
        if (tone === "danger")
            return Theme.danger
        if (tone === "accent")
            return Theme.accentCool
        return Theme.info
    }

    function fillColor() {
        const base = toneColor()
        return Qt.rgba(base.r, base.g, base.b, compact ? 0.12 : 0.14)
    }

    radius: compact ? Theme.radiusMd : Theme.radiusLg
    color: fillColor()
    border.width: 1
    border.color: Qt.rgba(toneColor().r, toneColor().g, toneColor().b, 0.25)
    implicitWidth: compact ? 168 : 210
    implicitHeight: compact ? 58 : 72

    Behavior on color {
        ColorAnimation { duration: Theme.motionBase }
    }

    Behavior on border.color {
        ColorAnimation { duration: Theme.motionBase }
    }

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: compact ? Theme.space3 : Theme.space4
        spacing: compact ? 4 : 6

        Label {
            text: root.label
            color: Theme.textMuted
            font.family: Theme.fontSans
            font.pixelSize: compact ? 11 : 12
            font.weight: Font.Medium
        }

        RowLayout {
            Layout.fillWidth: true
            spacing: 8

            Rectangle {
                width: 9
                height: 9
                radius: 4.5
                color: root.toneColor()
            }

            Label {
                Layout.fillWidth: true
                text: root.value
                color: Theme.textPrimary
                font.family: Theme.fontSans
                font.pixelSize: compact ? 13 : 14
                font.weight: Font.DemiBold
                elide: Text.ElideRight
            }
        }
    }
}
