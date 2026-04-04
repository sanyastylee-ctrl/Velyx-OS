import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Velyx.DesignSystem
import Velyx.UI

ApplicationWindow {
    width: 1380
    height: 900
    visible: true
    color: Theme.windowBg
    title: "Velyx Files"

    Rectangle {
        anchors.fill: parent
        color: "#0f131a"
    }

    RowLayout {
        anchors.fill: parent
        anchors.margins: 24
        spacing: 20

        Card {
            Layout.preferredWidth: 280
            Layout.fillHeight: true

            ColumnLayout {
                anchors.fill: parent
                spacing: Theme.space4

                SectionHeader {
                    title: "Файлы"
                    subtitle: "Безопасная и понятная работа с данными пользователя"
                }

                SearchField {
                    Layout.fillWidth: true
                    placeholderText: "Поиск файлов"
                }

                ListRow { title: "Домашняя"; subtitle: "Закрепленное расположение" }
                ListRow { title: "Рабочий стол"; subtitle: "Закрепленное расположение" }
                ListRow { title: "Документы"; subtitle: "Закрепленное расположение" }
                ListRow { title: "Загрузки"; subtitle: "Закрепленное расположение" }
                ListRow { title: "Изображения"; subtitle: "Закрепленное расположение" }
                ListRow { title: "Внешний диск"; subtitle: "USB-C SSD подключен"; trailingText: "1.8 ТБ" }
                ListRow { title: "Корзина"; subtitle: "Восстановление доступно" }
            }
        }

        ColumnLayout {
            Layout.fillWidth: true
            Layout.fillHeight: true
            spacing: 20

            Card {
                Layout.fillWidth: true
                Layout.preferredHeight: 110

                RowLayout {
                    anchors.fill: parent
                    spacing: Theme.space4

                    ColumnLayout {
                        Layout.fillWidth: true
                        spacing: 4

                        Label {
                            text: "Документы"
                            color: Theme.textPrimary
                            font.family: Theme.fontDisplay
                            font.pixelSize: 28
                        }

                        Label {
                            text: "Файловый менеджер должен отделять пользовательские операции от системного контура и давать безопасный UX для внешних носителей."
                            color: Theme.textSecondary
                            wrapMode: Text.WordWrap
                        }
                    }

                    Button { text: "Новая папка" }
                    Button { text: "Импорт" }
                }
            }

            Card {
                Layout.fillWidth: true
                Layout.fillHeight: true

                ColumnLayout {
                    anchors.fill: parent
                    spacing: Theme.space3

                    RowLayout {
                        Layout.fillWidth: true
                        spacing: Theme.space4

                        Label {
                            text: "Имя"
                            color: Theme.textMuted
                            Layout.fillWidth: true
                        }

                        Label {
                            text: "Изменено"
                            color: Theme.textMuted
                            Layout.preferredWidth: 180
                        }

                        Label {
                            text: "Размер"
                            color: Theme.textMuted
                            Layout.preferredWidth: 100
                        }
                    }

                    ListRow { title: "Бренд-материалы"; subtitle: "Папка"; trailingText: "2.4 ГБ" }
                    ListRow { title: "Концепты"; subtitle: "Папка"; trailingText: "412 МБ" }
                    ListRow { title: "Счета"; subtitle: "Папка"; trailingText: "28 МБ" }
                    ListRow { title: "Steam Captures"; subtitle: "Папка"; trailingText: "8.2 ГБ" }
                    ListRow { title: "Путешествия"; subtitle: "Папка"; trailingText: "1.3 ГБ" }
                }
            }
        }
    }
}
