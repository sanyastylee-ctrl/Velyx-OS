import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Velyx.DesignSystem
import Velyx.UI

Card {
    id: root

    property alias commandText: commandField.text
    signal submitRequested(string text)

    ColumnLayout {
        anchors.fill: parent
        spacing: Theme.space4

        SectionHeader {
            title: "AI Command Palette"
            subtitle: "AI интерпретирует запрос, но не действует мимо системных сервисов."
        }

        SearchField {
            id: commandField
            Layout.fillWidth: true
            placeholderText: "Например: Открой браузер"
        }

        RowLayout {
            Layout.fillWidth: true

            Label {
                Layout.fillWidth: true
                text: "Режим по умолчанию: ask-before-act"
                color: Theme.textSecondary
                font.pixelSize: 12
            }

            Button {
                text: "Отправить"
                onClicked: root.submitRequested(commandField.text)
            }
        }
    }
}
