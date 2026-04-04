import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Velyx.DesignSystem

Rectangle {
    id: root

    property alias text: input.text
    property string placeholderText: "Поиск"

    radius: Theme.radiusLg
    color: Theme.surface1
    border.width: 1
    border.color: Theme.strokeSubtle
    implicitHeight: 48

    RowLayout {
        anchors.fill: parent
        anchors.leftMargin: Theme.space4
        anchors.rightMargin: Theme.space4
        spacing: Theme.space3

        Label {
            text: "Поиск"
            color: Theme.textMuted
            font.pixelSize: 12
        }

        TextField {
            id: input
            Layout.fillWidth: true
            placeholderText: root.placeholderText
            color: Theme.textPrimary
            placeholderTextColor: Theme.textMuted
            background: null
            font.family: Theme.fontSans
            font.pixelSize: 15
            selectByMouse: true
        }
    }
}
