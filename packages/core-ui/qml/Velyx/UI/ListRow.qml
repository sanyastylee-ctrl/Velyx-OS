import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Velyx.DesignSystem

Rectangle {
    id: root

    property string title: ""
    property string subtitle: ""
    property string trailingText: ""

    radius: Theme.radiusMd
    color: hover.hovered ? Theme.surface2 : Theme.surface1
    border.color: Theme.strokeSubtle
    border.width: 1
    implicitHeight: 68

    RowLayout {
        anchors.fill: parent
        anchors.margins: Theme.space4
        spacing: Theme.space4

        Rectangle {
            Layout.preferredWidth: 36
            Layout.preferredHeight: 36
            radius: 12
            color: Theme.surface3
        }

        ColumnLayout {
            Layout.fillWidth: true
            spacing: 2

            Label {
                text: root.title
                color: Theme.textPrimary
                font.pixelSize: 15
                font.weight: Font.Medium
            }

            Label {
                visible: text.length > 0
                text: root.subtitle
                color: Theme.textSecondary
                font.pixelSize: 12
            }
        }

        Label {
            visible: text.length > 0
            text: root.trailingText
            color: Theme.textMuted
            font.pixelSize: 12
        }
    }

    HoverHandler {
        id: hover
    }
}
