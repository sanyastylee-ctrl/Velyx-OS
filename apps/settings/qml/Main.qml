import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Velyx.DesignSystem
import Velyx.UI

ApplicationWindow {
    width: 1320
    height: 860
    visible: true
    color: Theme.windowBg
    title: "Velyx Settings"

    Rectangle {
        anchors.fill: parent
        gradient: Gradient {
            GradientStop { position: 0.0; color: "#10131a" }
            GradientStop { position: 1.0; color: "#141a26" }
        }
    }

    RowLayout {
        anchors.fill: parent
        anchors.margins: 28
        spacing: 20

        Card {
            Layout.preferredWidth: 310
            Layout.fillHeight: true

            ColumnLayout {
                anchors.fill: parent
                spacing: Theme.space4

                SectionHeader {
                    title: "Настройки"
                    subtitle: "Единая точка системной конфигурации"
                }

                SearchField {
                    Layout.fillWidth: true
                    placeholderText: "Поиск настроек"
                }

                Repeater {
                    model: [
                        "Система",
                        "Персонализация",
                        "Дисплеи",
                        "Звук",
                        "Сеть",
                        "Bluetooth и устройства",
                        "Хранилище",
                        "Приложения",
                        "Приватность и разрешения",
                        "Аккаунты",
                        "Обновления и восстановление",
                        "Игры и производительность",
                        "Для разработчика"
                    ]

                    delegate: ListRow {
                        title: modelData
                        subtitle: index === 0 ? "Питание, запуск, поведение системы" : "Каркас раздела"
                    }
                }
            }
        }

        Card {
            Layout.fillWidth: true
            Layout.fillHeight: true

            ColumnLayout {
                anchors.fill: parent
                spacing: Theme.space5

                SectionHeader {
                    title: "Система"
                    subtitle: "Настройки должны быть плоскими, понятными и безопасными по умолчанию."
                }

                RowLayout {
                    Layout.fillWidth: true
                    spacing: Theme.space4

                    Card {
                        Layout.fillWidth: true
                        Layout.preferredHeight: 180
                        fillColor: Theme.surface2

                        ColumnLayout {
                            anchors.fill: parent
                            spacing: Theme.space3

                            Label {
                                text: "Устройство"
                                color: Theme.textSecondary
                            }

                            Label {
                                text: "VelyxBook Prototype"
                                color: Theme.textPrimary
                                font.family: Theme.fontDisplay
                                font.pixelSize: 26
                            }

                            Label {
                                text: "Здесь будут стартовые параметры, режимы энергопотребления, recovery и безопасные дефолты."
                                wrapMode: Text.WordWrap
                                color: Theme.textSecondary
                            }
                        }
                    }

                    Card {
                        Layout.fillWidth: true
                        Layout.preferredHeight: 180
                        fillColor: Theme.surface2

                        ColumnLayout {
                            anchors.fill: parent
                            spacing: Theme.space3

                            Label {
                                text: "Security-first IA"
                                color: Theme.textSecondary
                            }

                            Label {
                                text: "Опасные действия объясняются, подтверждаются и по возможности обратимы."
                                wrapMode: Text.WordWrap
                                color: Theme.textPrimary
                                font.family: Theme.fontDisplay
                                font.pixelSize: 24
                            }
                        }
                    }
                }

                ListRow { title: "Запуск и вход"; subtitle: "Блокировка, resume, поведение после обновлений"; trailingText: "План" }
                ListRow { title: "Recovery"; subtitle: "Точки отката, инструменты восстановления"; trailingText: "План" }
                ListRow { title: "Приватность и разрешения"; subtitle: "Камера, микрофон, экран, файлы, USB"; trailingText: "Критично" }
                ListRow { title: "Обновления"; subtitle: "Подписанные пакеты, staged apply, rollback"; trailingText: "Критично" }

                Card {
                    Layout.fillWidth: true
                    Layout.preferredHeight: 260
                    fillColor: Theme.surface2

                    ColumnLayout {
                        anchors.fill: parent
                        spacing: Theme.space4

                        SectionHeader {
                            title: "Settings Service"
                            subtitle: "Первый реальный backend системных настроек"
                        }

                        ListRow {
                            title: "Тема"
                            subtitle: "appearance.theme"
                            trailingText: settingsClient.theme
                        }

                        RowLayout {
                            Layout.fillWidth: true

                            Button {
                                text: "Тема: dark"
                                onClicked: settingsClient.setValue("appearance.theme", "dark")
                            }

                            Button {
                                text: "Тема: light"
                                onClicked: settingsClient.setValue("appearance.theme", "light")
                            }
                        }

                        ListRow {
                            title: "Bluetooth"
                            subtitle: "bluetooth.enabled"
                            trailingText: settingsClient.bluetoothEnabled
                        }

                        Button {
                            text: settingsClient.bluetoothEnabled === "true"
                                ? "Выключить Bluetooth"
                                : "Включить Bluetooth"
                            onClicked: settingsClient.setValue(
                                "bluetooth.enabled",
                                settingsClient.bluetoothEnabled === "true" ? "false" : "true")
                        }

                        ListRow {
                            title: "AI Layer"
                            subtitle: "ai.enabled"
                            trailingText: settingsClient.aiEnabled
                        }
                    }
                }

                Card {
                    Layout.fillWidth: true
                    Layout.fillHeight: true
                    fillColor: Theme.surface2

                    ColumnLayout {
                        anchors.fill: parent
                        spacing: Theme.space3

                        SectionHeader {
                            title: "Журнал разрешений"
                            subtitle: "UI-заглушка для будущего privacy dashboard"
                        }

                        ListRow { title: "com.velyx.files"; subtitle: "filesystem"; trailingText: "allow / deny / prompt" }
                        ListRow { title: "com.velyx.testapp"; subtitle: "filesystem"; trailingText: "allow / deny / prompt" }
                    }
                }

                RowLayout {
                    Layout.fillWidth: true
                    spacing: Theme.space4

                    AIPreferencesPage {
                        Layout.fillWidth: true
                        Layout.preferredHeight: 240
                    }

                    AIPrivacyPage {
                        Layout.fillWidth: true
                        Layout.preferredHeight: 240
                    }
                }

                AIAuditPage {
                    Layout.fillWidth: true
                    Layout.preferredHeight: 220
                }
            }
        }
    }
}
