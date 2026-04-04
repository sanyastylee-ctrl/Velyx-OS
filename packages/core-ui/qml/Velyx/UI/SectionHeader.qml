import QtQuick
import QtQuick.Controls
import Velyx.DesignSystem

Column {
    id: root

    property string title: ""
    property string subtitle: ""

    spacing: 4

    Label {
        text: root.title
        color: Theme.textPrimary
        font.family: Theme.fontDisplay
        font.pixelSize: 24
        font.weight: Font.DemiBold
    }

    Label {
        visible: text.length > 0
        text: root.subtitle
        color: Theme.textSecondary
        font.family: Theme.fontSans
        font.pixelSize: 13
    }
}
