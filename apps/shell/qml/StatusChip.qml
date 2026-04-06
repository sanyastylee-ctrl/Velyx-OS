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
            return "#2f9e6f"
        if (tone === "warning")
            return "#d6a44a"
        if (tone === "danger")
            return "#cf5c61"
        if (tone === "accent")
            return "#5b8cff"
        return "#6b7280"
    }

    radius: compact ? 14 : 18
    color: "#161c28"
    border.width: 1
    border.color: Qt.rgba(1, 1, 1, 0.08)
    implicitWidth: compact ? 170 : 210
    implicitHeight: compact ? 54 : 68

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: compact ? 10 : 14
        spacing: compact ? 2 : 4

        Label {
            text: root.label
            color: "#8f99ad"
            font.pixelSize: compact ? 11 : 12
        }

        RowLayout {
            Layout.fillWidth: true
            spacing: 8

            Rectangle {
                width: 8
                height: 8
                radius: 4
                color: root.toneColor()
            }

            Label {
                Layout.fillWidth: true
                text: root.value
                color: "#f3f6fb"
                font.pixelSize: compact ? 13 : 14
                font.weight: Font.DemiBold
                elide: Text.ElideRight
            }
        }
    }
}
