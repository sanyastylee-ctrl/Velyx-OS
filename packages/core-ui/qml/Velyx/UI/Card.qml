import QtQuick
import Velyx.DesignSystem

Rectangle {
    id: root

    property color fillColor: Theme.surface1
    property color borderColor: Theme.strokeSubtle
    default property alias data: contentItem.data

    radius: Theme.radiusMd
    color: fillColor
    border.color: borderColor
    border.width: 1

    Item {
        id: contentItem
        anchors.fill: parent
        anchors.margins: Theme.space5
    }
}
