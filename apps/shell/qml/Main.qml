import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Velyx.DesignSystem
import Velyx.UI

ApplicationWindow {
    id: window

    width: 1360
    height: 880
    visible: true
    color: settingsClient.theme === "light" ? "#eef2f7" : Theme.windowBg
    title: "Velyx Shell MVP"

    Component.onCompleted: {
        permissionClient.refreshRuntimeStatus()
        permissionClient.refreshOpenApps()
        permissionClient.refreshApps()
    }

    Timer {
        interval: 3000
        running: true
        repeat: true
        onTriggered: {
            permissionClient.refreshRuntimeStatus()
            permissionClient.refreshOpenApps()
            permissionClient.refreshSelectedAppRuntime()
            permissionClient.refreshApps()
        }
    }

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
        width: 560
        placeholderText: "Это MVP launcher shell: выберите приложение и проверьте permission flow"
        readOnly: true
    }

    SectionHeader {
        anchors.top: parent.top
        anchors.topMargin: 140
        anchors.left: parent.left
        anchors.leftMargin: 72
        title: "Velyx Shell MVP"
        subtitle: "Минимальный графический клиент для launcher-service и permissions-service"
    }

    Card {
        anchors.top: parent.top
        anchors.topMargin: 184
        anchors.left: parent.left
        anchors.leftMargin: 72
        anchors.right: parent.right
        anchors.rightMargin: 72
        height: 88

        RowLayout {
            anchors.fill: parent
            spacing: Theme.space4

            ListRow {
                Layout.fillWidth: true
                title: "Launcher"
                subtitle: permissionClient.launcherAvailability
            }

            ListRow {
                Layout.fillWidth: true
                title: "Permissions"
                subtitle: permissionClient.permissionsAvailability
            }

            ListRow {
                Layout.fillWidth: true
                title: "Session"
                subtitle: permissionClient.sessionAvailability
            }

            ListRow {
                Layout.fillWidth: true
                title: "Session state"
                subtitle: permissionClient.sessionState
            }

            ListRow {
                Layout.fillWidth: true
                title: "Session health"
                subtitle: permissionClient.sessionHealth
            }

            ListRow {
                Layout.fillWidth: true
                title: "Active app"
                subtitle: permissionClient.activeAppTitle.length > 0
                    ? permissionClient.activeAppTitle + " (" + permissionClient.activeAppId + ")"
                    : "Нет активного приложения"
            }
        }
    }

    Card {
        anchors.top: parent.top
        anchors.topMargin: 292
        anchors.left: parent.left
        anchors.leftMargin: 72
        width: 360
        height: 340

        ColumnLayout {
            anchors.fill: parent
            spacing: Theme.space4

            Label {
                text: "Список приложений"
                color: Theme.textSecondary
                font.pixelSize: 12
            }

            RowLayout {
                Layout.fillWidth: true
                spacing: Theme.space3

                Button {
                    text: "Обновить"
                    onClicked: permissionClient.refreshApps()
                }
            }

            ListView {
                Layout.fillWidth: true
                Layout.fillHeight: true
                clip: true
                spacing: Theme.space3
                model: permissionClient.apps

                delegate: Rectangle {
                    required property var modelData
                    width: ListView.view.width
                    height: 78
                    radius: 18
                    color: permissionClient.selectedAppId === modelData.app_id ? Theme.surface3 : Theme.surface2
                    border.width: 1
                    border.color: Theme.strokeSubtle

                    MouseArea {
                        anchors.fill: parent
                        onClicked: permissionClient.selectApp(parent.modelData.app_id)
                    }

                    Column {
                        anchors.fill: parent
                        anchors.margins: 16
                        spacing: 6

                        Label {
                            text: parent.parent.modelData.display_name.length > 0
                                ? parent.parent.modelData.display_name
                                : parent.parent.modelData.app_id
                            color: Theme.textPrimary
                            font.pixelSize: 16
                            font.weight: Font.DemiBold
                        }

                        Label {
                            text: parent.parent.modelData.app_id
                            color: Theme.textSecondary
                            font.pixelSize: 12
                        }

                        Label {
                            text: (parent.parent.modelData.session_required === "true" ? "[required] " : "[optional] ")
                                + "trust=" + parent.parent.modelData.trust_level
                            color: Theme.textMuted
                            font.pixelSize: 11
                        }

                        Label {
                            text: "state=" + (parent.parent.modelData.runtime_state ? parent.parent.modelData.runtime_state : "idle")
                                + (parent.parent.modelData.runtime_pid ? " pid=" + parent.parent.modelData.runtime_pid : "")
                            color: Theme.textMuted
                            font.pixelSize: 11
                        }
                    }
                }
            }
        }
    }

    Card {
        anchors.top: parent.top
        anchors.topMargin: 292
        anchors.left: parent.left
        anchors.leftMargin: 468
        width: 380
        height: 340

        ColumnLayout {
            anchors.fill: parent
            spacing: Theme.space3

            SectionHeader {
                title: "Информация о приложении"
                subtitle: "GetAppInfo(app_id)"
            }

            ListRow {
                title: "App ID"
                subtitle: permissionClient.selectedAppInfo.app_id ? permissionClient.selectedAppInfo.app_id : "Не выбрано"
            }
            ListRow {
                title: "Display name"
                subtitle: permissionClient.selectedAppInfo.display_name ? permissionClient.selectedAppInfo.display_name : "Не выбрано"
            }
            ListRow {
                title: "Trust level"
                subtitle: permissionClient.selectedAppInfo.trust_level ? permissionClient.selectedAppInfo.trust_level : "-"
            }
            ListRow {
                title: "Required"
                subtitle: permissionClient.selectedAppInfo.session_required ? permissionClient.selectedAppInfo.session_required : "false"
            }
            ListRow {
                title: "Autostart"
                subtitle: permissionClient.selectedAppInfo.session_autostart ? permissionClient.selectedAppInfo.session_autostart : "false"
            }
            ListRow {
                title: "Required permissions"
                subtitle: permissionClient.selectedAppInfo.requested_permissions ? permissionClient.selectedAppInfo.requested_permissions : "-"
            }
            ListRow {
                title: "Executable path"
                subtitle: permissionClient.selectedAppInfo.executable_path ? permissionClient.selectedAppInfo.executable_path : "-"
            }
            ListRow {
                title: "Runtime state"
                subtitle: permissionClient.selectedAppInfo.runtime_state ? permissionClient.selectedAppInfo.runtime_state : "idle"
            }
            ListRow {
                title: "Runtime pid"
                subtitle: permissionClient.selectedAppInfo.runtime_pid ? permissionClient.selectedAppInfo.runtime_pid : "-"
            }
            ListRow {
                title: "Last launch"
                subtitle: permissionClient.selectedAppInfo.last_launch_status ? permissionClient.selectedAppInfo.last_launch_status : "-"
            }
            ListRow {
                title: "Last pid"
                subtitle: permissionClient.selectedAppInfo.last_pid ? permissionClient.selectedAppInfo.last_pid : "-"
            }
            ListRow {
                title: "Last exit code"
                subtitle: permissionClient.selectedAppInfo.runtime_exit_code ? permissionClient.selectedAppInfo.runtime_exit_code : "-"
            }
            ListRow {
                title: "Failure reason"
                subtitle: permissionClient.selectedAppInfo.runtime_failure_reason ? permissionClient.selectedAppInfo.runtime_failure_reason : "-"
            }
            ListRow {
                title: "Restart attempts"
                subtitle: permissionClient.selectedAppInfo.session_retry_count ? permissionClient.selectedAppInfo.session_retry_count : "0"
            }

            Item { Layout.fillHeight: true }

            RowLayout {
                Layout.fillWidth: true
                spacing: Theme.space3

                Button {
                    Layout.fillWidth: true
                    text: "Launch"
                    enabled: permissionClient.selectedAppId.length > 0
                    onClicked: permissionClient.launchSelectedApp()
                }

                Button {
                    Layout.fillWidth: true
                    text: "Stop"
                    enabled: permissionClient.selectedAppId.length > 0
                    onClicked: permissionClient.stopSelectedApp()
                }

                Button {
                    Layout.fillWidth: true
                    text: "Restart"
                    enabled: permissionClient.selectedAppId.length > 0
                    onClicked: permissionClient.restartSelectedApp()
                }

                Button {
                    Layout.fillWidth: true
                    text: "Reset permissions"
                    enabled: permissionClient.selectedAppId.length > 0
                    onClicked: permissionClient.resetPermissions(permissionClient.selectedAppId)
                }
            }
        }
    }

    Card {
        anchors.top: parent.top
        anchors.topMargin: 292
        anchors.right: parent.right
        anchors.rightMargin: 72
        width: 420
        height: 340

        ColumnLayout {
            anchors.fill: parent
            spacing: Theme.space3

            SectionHeader {
                title: "Статус и результат"
                subtitle: "Последний backend action/result"
            }

            ListRow {
                title: "Last action"
                subtitle: permissionClient.lastAction.length > 0 ? permissionClient.lastAction : "-"
            }
            ListRow {
                title: "Last result"
                subtitle: permissionClient.lastResult.length > 0 ? permissionClient.lastResult : "-"
            }
            ListRow {
                title: "Reason"
                subtitle: permissionClient.lastReason.length > 0 ? permissionClient.lastReason : "-"
            }
            ListRow {
                title: "Next action"
                subtitle: permissionClient.nextAction.length > 0 ? permissionClient.nextAction : "-"
            }

            Card {
                Layout.fillWidth: true
                Layout.fillHeight: true
                fillColor: Theme.surface2

                ColumnLayout {
                    anchors.fill: parent
                    spacing: Theme.space3

                    Label {
                        text: "Last message"
                        color: Theme.textMuted
                        font.pixelSize: 12
                    }

                    Label {
                        Layout.fillWidth: true
                        wrapMode: Text.WordWrap
                        text: permissionClient.launchResultMessage.length > 0
                            ? permissionClient.launchResultMessage
                            : "Здесь появится результат launch/permission flow."
                        color: permissionClient.launchStatus === "denied"
                            ? Theme.danger
                            : ((permissionClient.launchStatus === "allowed"
                                || permissionClient.launchStatus === "already_running"
                                || permissionClient.launchStatus === "launched")
                               ? Theme.accentStrong
                               : Theme.textPrimary)
                        font.pixelSize: 14
                    }
                }
            }
        }
    }

    Card {
        anchors.left: parent.left
        anchors.leftMargin: 72
        anchors.right: parent.right
        anchors.rightMargin: 72
        anchors.bottom: parent.bottom
        anchors.bottomMargin: 48
        height: 180

        ColumnLayout {
            anchors.fill: parent
            spacing: Theme.space3

            SectionHeader {
                title: "Open Apps"
                subtitle: "Минимальный window host layer"
            }

            ListView {
                Layout.fillWidth: true
                Layout.fillHeight: true
                clip: true
                spacing: Theme.space3
                model: permissionClient.openApps

                delegate: Rectangle {
                    required property var modelData
                    width: ListView.view.width
                    height: 62
                    radius: 16
                    color: modelData.active ? Theme.surface3 : Theme.surface2
                    border.width: 1
                    border.color: modelData.active ? Theme.accentStrong : Theme.strokeSubtle

                    RowLayout {
                        anchors.fill: parent
                        anchors.margins: 14
                        spacing: Theme.space3

                        ColumnLayout {
                            Layout.fillWidth: true
                            spacing: 4

                            Label {
                                text: modelData.display_name ? modelData.display_name : modelData.app_id
                                color: Theme.textPrimary
                                font.pixelSize: 15
                                font.weight: Font.DemiBold
                            }

                            Label {
                                text: modelData.app_id + " | state=" + modelData.state
                                    + (modelData.pid ? " | pid=" + modelData.pid : "")
                                color: Theme.textSecondary
                                font.pixelSize: 12
                            }
                        }

                        Button {
                            text: modelData.active ? "Active" : "Activate"
                            onClicked: permissionClient.selectActiveApp(modelData.app_id)
                        }

                        Button {
                            text: "Restart"
                            onClicked: permissionClient.restartOpenApp(modelData.app_id)
                        }

                        Button {
                            text: "Close"
                            onClicked: permissionClient.closeOpenApp(modelData.app_id)
                        }
                    }

                }
            }
        }
    }
}
