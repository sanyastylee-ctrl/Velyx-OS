import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Velyx.DesignSystem
import Velyx.UI

ApplicationWindow {
    id: window

    width: 1440
    height: 900
    visible: true
    color: settingsClient.theme === "light" ? "#eef2f7" : Theme.windowBg
    title: "Velyx Shell"

    property bool launcherOpen: true
    property bool quickSettingsOpen: false
    property bool aiOverlayOpen: true

    PermissionDialog {
        id: permissionDialog

        onAllowSelected: function(appId, appName, permission) {
            close()
            permissionClient.submitDecision(appId, appName, permission, true)
        }

        onDenySelected: function(appId, appName, permission) {
            close()
            permissionClient.submitDecision(appId, appName, permission, false)
        }
    }

    AIConfirmationDialog {
        id: aiConfirmationDialog

        onConfirmAccepted: function() {
            close()
            aiClient.confirmPendingAction(true)
        }

        onConfirmRejected: function() {
            close()
            aiClient.confirmPendingAction(false)
        }
    }

    Connections {
        target: permissionClient

        function onPermissionPromptRequired(appId, appName, permission, permissionDisplayName, explanation) {
            permissionDialog.appId = appId
            permissionDialog.appName = appName
            permissionDialog.permission = permission
            permissionDialog.permissionDisplayName = permissionDisplayName
            permissionDialog.explanation = explanation
            permissionDialog.open()
        }
    }

    Connections {
        target: aiClient

        function onConfirmationChanged() {
            if (aiClient.confirmationPending) {
                aiConfirmationDialog.summary = aiClient.confirmationSummary
                aiConfirmationDialog.detailedReason = aiClient.confirmationDetails
                aiConfirmationDialog.riskLevel = aiClient.confirmationRisk
                aiConfirmationDialog.affectedApp = aiClient.confirmationApp
                aiConfirmationDialog.affectedPermission = aiClient.confirmationPermission
                aiConfirmationDialog.open()
            } else {
                aiConfirmationDialog.close()
            }
        }
    }

    Rectangle {
        anchors.fill: parent
        gradient: Gradient {
            GradientStop { position: 0.0; color: settingsClient.theme === "light" ? "#f6f8fb" : "#0d1016" }
            GradientStop { position: 0.45; color: settingsClient.theme === "light" ? "#edf2f7" : "#111623" }
            GradientStop { position: 1.0; color: settingsClient.theme === "light" ? "#dde7f2" : "#151b2a" }
        }
    }

    Rectangle {
        anchors.top: parent.top
        anchors.left: parent.left
        anchors.right: parent.right
        height: 52
        color: "#101520"
        border.color: Theme.strokeSubtle

        RowLayout {
            anchors.fill: parent
            anchors.leftMargin: Theme.space5
            anchors.rightMargin: Theme.space5

            Label {
                text: "Velyx OS"
                color: Theme.textPrimary
                font.family: Theme.fontDisplay
                font.pixelSize: 20
                font.weight: Font.DemiBold
            }

            Item { Layout.fillWidth: true }

            Label {
                text: "Пятница 10:42"
                color: Theme.textSecondary
                font.pixelSize: 13
            }

            Button {
                text: "Центр управления"
                onClicked: window.quickSettingsOpen = !window.quickSettingsOpen
            }
        }
    }

    SearchField {
        anchors.top: parent.top
        anchors.topMargin: 72
        anchors.horizontalCenter: parent.horizontalCenter
        width: 520
        placeholderText: "Поиск приложений, файлов, настроек и действий"
    }

    SectionHeader {
        anchors.top: parent.top
        anchors.topMargin: 140
        anchors.left: parent.left
        anchors.leftMargin: 72
        title: "Прототип сессии Velyx OS"
        subtitle: "Shell остается тонким слоем, а критичная логика уходит в сервисы."
    }

    Card {
        anchors.top: parent.top
        anchors.topMargin: 240
        anchors.left: parent.left
        anchors.leftMargin: 72
        width: 500
        height: 300

        ColumnLayout {
            anchors.fill: parent
            spacing: Theme.space4

            Label {
                text: "Приоритеты"
                color: Theme.textSecondary
                font.pixelSize: 12
            }

            Label {
                text: "Безопасность, ясность, восстановление."
                wrapMode: Text.WordWrap
                color: Theme.textPrimary
                font.family: Theme.fontDisplay
                font.pixelSize: 28
                font.weight: Font.DemiBold
            }

            Label {
                text: "Shell должен быстро запускать сценарии пользователя, но не тащить в себя update, permissions или compatibility-логику."
                wrapMode: Text.WordWrap
                color: Theme.textSecondary
                font.pixelSize: 14
            }

            Item { Layout.fillHeight: true }

            RowLayout {
                spacing: Theme.space3

                Button {
                    text: "Лаунчер"
                    onClicked: window.launcherOpen = !window.launcherOpen
                }

                Button {
                    text: "Быстрые настройки"
                    onClicked: window.quickSettingsOpen = !window.quickSettingsOpen
                }

                Button {
                    text: "Velyx AI"
                    onClicked: window.aiOverlayOpen = !window.aiOverlayOpen
                }
            }

            Button {
                text: "Открыть тестовое приложение"
                onClicked: permissionClient.startLaunch(
                    "com.velyx.testapp",
                    "Тестовое приложение",
                    "filesystem")
            }

            Label {
                Layout.fillWidth: true
                wrapMode: Text.WordWrap
                text: permissionClient.launchResultMessage.length > 0
                    ? permissionClient.launchResultMessage
                    : "Здесь появится результат security flow."
                color: permissionClient.launchStatus === "denied"
                    ? Theme.danger
                    : (permissionClient.launchStatus === "allowed" ? Theme.accentStrong : Theme.textSecondary)
                font.pixelSize: 13
            }
        }
    }

    Card {
        anchors.top: parent.top
        anchors.topMargin: 240
        anchors.right: parent.right
        anchors.rightMargin: 72
        width: 520
        height: 360

        ColumnLayout {
            anchors.fill: parent
            spacing: Theme.space3

            SectionHeader {
                title: "Опорные подсистемы"
                subtitle: "Что shell будет вызывать через сервисные границы"
            }

            ListRow { title: "Notifications"; subtitle: "Отдельный сервис и журнал событий"; trailingText: "M2" }
            ListRow { title: "Permissions"; subtitle: "Privacy dashboard и consent flows"; trailingText: "M2" }
            ListRow { title: "Updates"; subtitle: "Проверка, staged apply, rollback"; trailingText: "M2" }
            ListRow { title: "Compatibility"; subtitle: "Управляемый запуск Windows-приложений"; trailingText: "M3" }
        }
    }

    AIOverlay {
        visible: window.aiOverlayOpen
        anchors.top: parent.top
        anchors.topMargin: 620
        anchors.left: parent.left
        anchors.leftMargin: 72
        width: 620
        height: 220
        understoodIntent: aiClient.understoodIntent
        selectedTool: aiClient.selectedTool
        resultText: aiClient.resultText
        explanationSource: aiClient.explanationSource
        suggestedAction: aiClient.suggestedAction
        onSubmitCommand: function(text) {
            aiClient.submitCommand(text)
        }
    }

    Rectangle {
        anchors.bottom: parent.bottom
        anchors.horizontalCenter: parent.horizontalCenter
        anchors.bottomMargin: 20
        width: 620
        height: 82
        radius: 28
        color: "#182031"
        border.color: Theme.strokeSubtle

        RowLayout {
            anchors.fill: parent
            anchors.margins: Theme.space4
            spacing: Theme.space4

            Repeater {
                model: ["Пуск", "Браузер", "Файлы", "Настройки", "Store", "Steam"]

                delegate: Rectangle {
                    Layout.preferredWidth: 88
                    Layout.fillHeight: true
                    radius: 20
                    color: index === 0 ? Theme.surface3 : "transparent"
                    border.width: index === 0 ? 1 : 0
                    border.color: Theme.strokeSubtle

                    Label {
                        anchors.centerIn: parent
                        text: modelData
                        color: Theme.textPrimary
                        font.pixelSize: 12
                    }
                }
            }
        }
    }

    Card {
        visible: window.launcherOpen
        anchors.left: parent.left
        anchors.leftMargin: 72
        anchors.bottom: parent.bottom
        anchors.bottomMargin: 120
        width: 420
        height: 360
        fillColor: "#171d2b"

        ColumnLayout {
            anchors.fill: parent
            spacing: Theme.space4

            SectionHeader {
                title: "Лаунчер"
                subtitle: "Поиск и входные точки системы"
            }

            SearchField {
                Layout.fillWidth: true
                placeholderText: "Поиск приложений, файлов, команд"
            }

            ListRow { title: "Браузер"; subtitle: "Закреплено" }
            ListRow { title: "Файлы"; subtitle: "Недавнее" }
            ListRow { title: "Настройки"; subtitle: "Результат по запросу Дисплеи" }
            ListRow { title: "Steam"; subtitle: "Установлено" }

            RowLayout {
                Layout.fillWidth: true
                spacing: Theme.space3

                Button {
                    text: "Запустить com.velyx.files"
                    onClicked: permissionClient.startLaunch(
                        "com.velyx.files",
                        "Файлы",
                        "filesystem")
                }

                Button {
                    text: "Сбросить решения"
                    onClicked: permissionClient.resetPermissions("com.velyx.testapp")
                }
            }
        }
    }

    Card {
        visible: window.quickSettingsOpen
        anchors.top: parent.top
        anchors.topMargin: 72
        anchors.right: parent.right
        anchors.rightMargin: 28
        width: 340
        height: 420
        fillColor: "#171d2b"

        ColumnLayout {
            anchors.fill: parent
            spacing: Theme.space4

            SectionHeader {
                title: "Быстрые настройки"
                subtitle: "Точка входа в централизованные системные действия"
            }

            ListRow { title: "Wi-Fi"; subtitle: "Office-5G"; trailingText: "Подключено" }
            ListRow { title: "Bluetooth"; subtitle: "Клавиатура, наушники"; trailingText: "2 устройства" }
            ListRow { title: "Звук"; subtitle: "Динамики"; trailingText: "68%" }
            ListRow { title: "Режим фокуса"; subtitle: "Уведомления ограничены"; trailingText: "Выкл." }

            Item { Layout.fillHeight: true }

            Button {
                text: "Закрыть"
                onClicked: window.quickSettingsOpen = false
            }
        }
    }
}
