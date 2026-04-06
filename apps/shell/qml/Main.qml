import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Velyx.DesignSystem
import Velyx.UI

ApplicationWindow {
    id: window

    width: 1480
    height: 920
    visible: true
    color: "#0c1017"
    title: "Velyx Shell"

    property var appsInActiveSpace: permissionClient.apps.filter(function(app) { return app.in_active_space === true })
    property var runningInActiveSpace: permissionClient.openApps.filter(function(app) { return app.in_active_space === true })
    property var runningOutsideSpace: permissionClient.openApps.filter(function(app) { return app.in_active_space !== true })

    Component.onCompleted: {
        permissionClient.refreshRuntimeStatus()
        permissionClient.refreshSpaces()
        permissionClient.refreshIntents()
        permissionClient.refreshRules()
        permissionClient.refreshAgentState()
        permissionClient.refreshOpenApps()
        permissionClient.refreshApps()
    }

    Shortcut {
        sequence: "Alt+Tab"
        enabled: permissionClient.inputControlMode !== "x11-global"
        onActivated: permissionClient.activateNextApp()
    }

    Shortcut {
        sequence: "Alt+Q"
        enabled: permissionClient.inputControlMode !== "x11-global"
        onActivated: permissionClient.closeActiveApp()
    }

    Shortcut {
        sequence: "Alt+R"
        enabled: permissionClient.inputControlMode !== "x11-global"
        onActivated: permissionClient.restartActiveInstance()
    }

    Shortcut {
        sequence: "Alt+1"
        enabled: permissionClient.inputControlMode !== "x11-global"
        onActivated: permissionClient.activateAppByIndex(0)
    }

    Shortcut {
        sequence: "Alt+2"
        enabled: permissionClient.inputControlMode !== "x11-global"
        onActivated: permissionClient.activateAppByIndex(1)
    }

    Shortcut {
        sequence: "Alt+3"
        enabled: permissionClient.inputControlMode !== "x11-global"
        onActivated: permissionClient.activateAppByIndex(2)
    }

    Shortcut {
        sequence: "Alt+4"
        enabled: permissionClient.inputControlMode !== "x11-global"
        onActivated: permissionClient.activateAppByIndex(3)
    }

    Shortcut {
        sequence: "Alt+5"
        enabled: permissionClient.inputControlMode !== "x11-global"
        onActivated: permissionClient.activateAppByIndex(4)
    }

    Shortcut {
        sequence: "Alt+6"
        enabled: permissionClient.inputControlMode !== "x11-global"
        onActivated: permissionClient.activateAppByIndex(5)
    }

    Shortcut {
        sequence: "Alt+7"
        enabled: permissionClient.inputControlMode !== "x11-global"
        onActivated: permissionClient.activateAppByIndex(6)
    }

    Shortcut {
        sequence: "Alt+8"
        enabled: permissionClient.inputControlMode !== "x11-global"
        onActivated: permissionClient.activateAppByIndex(7)
    }

    Shortcut {
        sequence: "Alt+9"
        enabled: permissionClient.inputControlMode !== "x11-global"
        onActivated: permissionClient.activateAppByIndex(8)
    }

    Timer {
        interval: 3000
        running: true
        repeat: true
        onTriggered: {
            permissionClient.refreshRuntimeStatus()
            permissionClient.refreshSpaces()
            permissionClient.refreshIntents()
            permissionClient.refreshRules()
            permissionClient.refreshAgentState()
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
            GradientStop { position: 0.0; color: "#0b1016" }
            GradientStop { position: 0.45; color: "#0f1520" }
            GradientStop { position: 1.0; color: "#121926" }
        }
    }

    Rectangle {
        anchors.fill: parent
        color: "transparent"
        border.width: 0

        ColumnLayout {
            anchors.fill: parent
            anchors.margins: 22
            spacing: 16

            Rectangle {
                Layout.fillWidth: true
                Layout.preferredHeight: 92
                radius: 26
                color: "#101722"
                border.width: 1
                border.color: Qt.rgba(1, 1, 1, 0.08)

                RowLayout {
                    anchors.fill: parent
                    anchors.margins: 20
                    spacing: 16

                    ColumnLayout {
                        Layout.fillWidth: true
                        spacing: 4

                        Label {
                            text: permissionClient.activeSpaceName.length > 0
                                ? permissionClient.activeSpaceName
                                : "Velyx OS"
                            color: "#f6f8fc"
                            font.pixelSize: 28
                            font.weight: Font.DemiBold
                        }

                        Label {
                            text: permissionClient.recoveryNeeded
                                ? "Recovery needed"
                                : (permissionClient.sessionState === "ready"
                                    ? "System ready"
                                    : "System requires attention")
                            color: permissionClient.recoveryNeeded ? "#ffb4b9" : "#9ba7bc"
                            font.pixelSize: 13
                        }
                    }

                    StatusChip {
                        compact: true
                        label: "Session"
                        value: permissionClient.sessionState
                        tone: permissionClient.sessionState === "ready" ? "success"
                            : (permissionClient.sessionState === "failed" ? "danger" : "warning")
                    }

                    StatusChip {
                        compact: true
                        label: "Security"
                        value: permissionClient.activeSpaceSecurityMode.length > 0 ? permissionClient.activeSpaceSecurityMode : "-"
                        tone: "accent"
                    }

                    StatusChip {
                        compact: true
                        label: "Update"
                        value: permissionClient.updateState
                        tone: permissionClient.recoveryNeeded ? "danger" : "neutral"
                    }

                    StatusChip {
                        compact: true
                        label: "Active app"
                        value: permissionClient.activeAppTitle.length > 0 ? permissionClient.activeAppTitle : "none"
                        tone: permissionClient.activeAppId.length > 0 ? "accent" : "neutral"
                    }
                }
            }

            RowLayout {
                Layout.fillWidth: true
                Layout.fillHeight: true
                spacing: 16

                Rectangle {
                    Layout.preferredWidth: 320
                    Layout.fillHeight: true
                    radius: 24
                    color: "#101722"
                    border.width: 1
                    border.color: Qt.rgba(1, 1, 1, 0.08)

                    ColumnLayout {
                        anchors.fill: parent
                        anchors.margins: 18
                        spacing: 14

                        ColumnLayout {
                            Layout.fillWidth: true
                            spacing: 4

                            Label {
                                text: "Spaces"
                                color: "#f3f6fb"
                                font.pixelSize: 22
                                font.weight: Font.DemiBold
                            }

                            Label {
                                text: "Contexts drive the session. Choose the space, then work inside it."
                                color: "#8f99ad"
                                font.pixelSize: 12
                                wrapMode: Text.WordWrap
                            }
                        }

                        Button {
                            text: "Refresh runtime"
                            onClicked: {
                                permissionClient.refreshRuntimeStatus()
                                permissionClient.refreshSpaces()
                                permissionClient.refreshIntents()
                                permissionClient.refreshRules()
                                permissionClient.refreshAgentState()
                                permissionClient.refreshOpenApps()
                                permissionClient.refreshApps()
                            }
                        }

                        Rectangle {
                            Layout.fillWidth: true
                            radius: 18
                            color: "#141c29"
                            border.width: 1
                            border.color: Qt.rgba(1, 1, 1, 0.08)
                            implicitHeight: 150

                            ColumnLayout {
                                anchors.fill: parent
                                anchors.margins: 14
                                spacing: 10

                                Label {
                                    text: "Intents"
                                    color: "#f3f6fb"
                                    font.pixelSize: 16
                                    font.weight: Font.DemiBold
                                }

                                Label {
                                    text: permissionClient.lastIntentId.length > 0
                                        ? "Last: " + permissionClient.lastIntentId + " • " + permissionClient.lastIntentResult
                                        : "Run a higher-level action to switch context."
                                    color: "#8f99ad"
                                    font.pixelSize: 11
                                    wrapMode: Text.WordWrap
                                }

                                ScrollView {
                                    Layout.fillWidth: true
                                    Layout.fillHeight: true
                                    clip: true

                                    Column {
                                        width: parent.width
                                        spacing: 10

                                        Repeater {
                                            model: permissionClient.intents

                                            delegate: IntentCard {
                                                width: parent.width
                                                intent: modelData
                                                lastIntentId: permissionClient.lastIntentId
                                                lastIntentResult: permissionClient.lastIntentResult
                                                onRunRequested: permissionClient.runIntent(intentId)
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        Rectangle {
                            Layout.fillWidth: true
                            radius: 18
                            color: "#141c29"
                            border.width: 1
                            border.color: Qt.rgba(1, 1, 1, 0.08)
                            implicitHeight: 170

                            ColumnLayout {
                                anchors.fill: parent
                                anchors.margins: 14
                                spacing: 10

                                Label {
                                    text: "Automation"
                                    color: "#f3f6fb"
                                    font.pixelSize: 16
                                    font.weight: Font.DemiBold
                                }

                                Label {
                                    text: permissionClient.lastRuleId.length > 0
                                        ? "Last: " + permissionClient.lastRuleId + " • " + permissionClient.lastRuleResult
                                        : "Rules react to system transitions and keep the runtime aligned."
                                    color: "#8f99ad"
                                    font.pixelSize: 11
                                    wrapMode: Text.WordWrap
                                }

                                Label {
                                    text: "Enabled rules: " + permissionClient.rules.filter(function(rule) { return rule.enabled === true }).length
                                    color: "#a4afc3"
                                    font.pixelSize: 11
                                }

                                ScrollView {
                                    Layout.fillWidth: true
                                    Layout.fillHeight: true
                                    clip: true

                                    Column {
                                        width: parent.width
                                        spacing: 8

                                        Repeater {
                                            model: permissionClient.rules

                                            delegate: Rectangle {
                                                width: parent.width
                                                radius: 14
                                                color: modelData.rule_id === permissionClient.lastRuleId ? "#1b2433" : "#151d2a"
                                                border.width: 1
                                                border.color: modelData.rule_id === permissionClient.lastRuleId ? "#5b8cff" : Qt.rgba(1, 1, 1, 0.08)
                                                implicitHeight: 74

                                                RowLayout {
                                                    anchors.fill: parent
                                                    anchors.margins: 12
                                                    spacing: 10

                                                    ColumnLayout {
                                                        Layout.fillWidth: true
                                                        spacing: 2

                                                        Label {
                                                            text: modelData.display_name || modelData.rule_id
                                                            color: "#f3f6fb"
                                                            font.pixelSize: 13
                                                            font.weight: Font.DemiBold
                                                            elide: Text.ElideRight
                                                        }

                                                        Label {
                                                            text: (modelData.trigger_type || "-") + " -> " + (modelData.action_type || "-")
                                                            color: "#8f99ad"
                                                            font.pixelSize: 11
                                                            elide: Text.ElideRight
                                                        }
                                                    }

                                                    Button {
                                                        text: "Run"
                                                        enabled: modelData.enabled === true
                                                        onClicked: permissionClient.runRule(modelData.rule_id)
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        AgentPanel {
                            Layout.fillWidth: true
                            permissionClient: permissionClient
                        }

                        ScrollView {
                            Layout.fillWidth: true
                            Layout.fillHeight: true
                            clip: true

                            Column {
                                width: parent.width
                                spacing: 10

                                Repeater {
                                    model: permissionClient.spaces

                                    delegate: SpaceCard {
                                        width: parent.width
                                        space: modelData
                                        onActivateRequested: permissionClient.activateSpace(spaceId)
                                    }
                                }
                            }
                        }
                    }
                }

                ColumnLayout {
                    Layout.fillWidth: true
                    Layout.fillHeight: true
                    spacing: 16

                    SpaceOverviewPanel {
                        Layout.fillWidth: true
                        Layout.preferredHeight: 190
                        permissionClient: permissionClient
                        inSpaceCount: window.appsInActiveSpace.length
                        runningInSpaceCount: window.runningInActiveSpace.length
                        outsideCount: window.runningOutsideSpace.length
                    }

                    SystemStatusPanel {
                        Layout.fillWidth: true
                        Layout.preferredHeight: 154
                        permissionClient: permissionClient
                    }

                    Rectangle {
                        Layout.fillWidth: true
                        Layout.fillHeight: true
                        radius: 24
                        color: "#101722"
                        border.width: 1
                        border.color: Qt.rgba(1, 1, 1, 0.08)

                        ColumnLayout {
                            anchors.fill: parent
                            anchors.margins: 18
                            spacing: 12

                            RowLayout {
                                Layout.fillWidth: true

                                ColumnLayout {
                                    Layout.fillWidth: true
                                    spacing: 4

                                    Label {
                                        text: "Apps in current space"
                                        color: "#f3f6fb"
                                        font.pixelSize: 20
                                        font.weight: Font.DemiBold
                                    }

                                    Label {
                                        text: "Primary app set for the active context."
                                        color: "#8f99ad"
                                        font.pixelSize: 12
                                    }
                                }
                            }

                            Label {
                                visible: window.appsInActiveSpace.length === 0
                                text: "No apps in this space"
                                color: "#7f8aa0"
                                font.pixelSize: 13
                            }

                            ScrollView {
                                Layout.fillWidth: true
                                Layout.fillHeight: true
                                clip: true
                                visible: window.appsInActiveSpace.length > 0

                                GridLayout {
                                    width: parent.width
                                    columns: width > 900 ? 2 : 1
                                    columnSpacing: 12
                                    rowSpacing: 12

                                    Repeater {
                                        model: window.appsInActiveSpace

                                        delegate: AppCard {
                                            Layout.fillWidth: true
                                            Layout.minimumWidth: 300
                                            app: modelData
                                            selected: permissionClient.selectedAppId === modelData.app_id
                                            onSelectRequested: permissionClient.selectApp(appId)
                                            onLaunchRequested: {
                                                permissionClient.selectApp(appId)
                                                permissionClient.launchSelectedApp()
                                            }
                                            onStopRequested: permissionClient.closeOpenApp(appId)
                                            onRestartRequested: permissionClient.restartOpenApp(appId)
                                            onActivateRequested: permissionClient.selectActiveApp(appId)
                                        }
                                    }
                                }
                            }
                        }
                    }

                    RowLayout {
                        Layout.fillWidth: true
                        Layout.preferredHeight: 250
                        spacing: 16

                        OpenAppsPanel {
                            Layout.fillWidth: true
                            Layout.fillHeight: true
                            title: "Open in this space"
                            subtitle: "Running windows aligned with the current context"
                            appsModel: window.runningInActiveSpace
                            permissionClient: permissionClient
                        }

                        OpenAppsPanel {
                            Layout.fillWidth: true
                            Layout.fillHeight: true
                            title: "Running outside current space"
                            subtitle: "Visible, but secondary to the active context"
                            appsModel: window.runningOutsideSpace
                            permissionClient: permissionClient
                        }
                    }
                }

                DetailsPanel {
                    Layout.preferredWidth: 360
                    Layout.fillHeight: true
                    permissionClient: permissionClient
                }
            }

            Rectangle {
                Layout.fillWidth: true
                Layout.preferredHeight: 54
                radius: 18
                color: "#101722"
                border.width: 1
                border.color: Qt.rgba(1, 1, 1, 0.08)

                RowLayout {
                    anchors.fill: parent
                    anchors.margins: 14
                    spacing: 16

                    Label {
                        text: "Last action"
                        color: "#8f99ad"
                        font.pixelSize: 12
                    }

                    Label {
                        Layout.fillWidth: true
                        text: permissionClient.lastAction.length > 0
                            ? permissionClient.lastAction + " • " + permissionClient.lastResult + " • " + permissionClient.lastReason
                            : "No recent action"
                        color: "#e8edf7"
                        font.pixelSize: 12
                        elide: Text.ElideRight
                    }

                    Label {
                        text: permissionClient.shortcutFeedback.length > 0
                            ? permissionClient.shortcutFeedback
                            : permissionClient.inputControlMode
                        color: "#8f99ad"
                        font.pixelSize: 12
                    }
                }
            }
        }
    }
}
