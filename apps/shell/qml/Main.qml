import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Velyx.DesignSystem
import Velyx.UI

ApplicationWindow {
    id: window

    width: 1520
    height: 940
    visible: true
    color: Theme.shellBg
    title: "Velyx Shell"

    property var appsInActiveSpace: permissionClient.apps.filter(function(app) { return app.in_active_space === true })
    property var runningInActiveSpace: permissionClient.openApps.filter(function(app) { return app.in_active_space === true })
    property var runningOutsideSpace: permissionClient.openApps.filter(function(app) { return app.in_active_space !== true })
    property var suggestedIntents: {
        var matching = []
        var others = []
        for (var i = 0; i < permissionClient.intents.length; ++i) {
            var intent = permissionClient.intents[i]
            if (intent.status === "disabled")
                continue
            if (permissionClient.activeSpaceId.length > 0 && intent.target_space === permissionClient.activeSpaceId)
                matching.push(intent)
            else
                others.push(intent)
        }
        return matching.concat(others).slice(0, 4)
    }
    property string systemConfidence: {
        if (permissionClient.recoveryNeeded)
            return "Recovery required"
        if (permissionClient.sessionState === "ready" && permissionClient.sessionHealth === "ready")
            return "Ready"
        if (permissionClient.sessionState === "failed" || permissionClient.sessionHealth === "failed" || permissionClient.updateState === "failed")
            return "Needs attention"
        return "Degraded"
    }

    Component.onCompleted: {
        permissionClient.refreshRuntimeStatus()
        permissionClient.refreshSpaces()
        permissionClient.refreshIntents()
        permissionClient.refreshRules()
        permissionClient.refreshAgentState()
        permissionClient.refreshAiState()
        permissionClient.refreshAssistantState()
        permissionClient.refreshFirstBootState()
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
            permissionClient.refreshAiState()
            permissionClient.refreshAssistantState()
            permissionClient.refreshFirstBootState()
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
            GradientStop { position: 0.0; color: Theme.shellBg }
            GradientStop { position: 0.44; color: Theme.shellBgAlt }
            GradientStop { position: 1.0; color: Theme.windowBg }
        }
    }

    Rectangle {
        anchors.fill: parent
        color: "transparent"

        ColumnLayout {
            anchors.fill: parent
            anchors.margins: Theme.space6
            spacing: Theme.space4

            Rectangle {
                Layout.fillWidth: true
                Layout.preferredHeight: 108
                radius: Theme.radiusXl
                color: Theme.shellSurfaceGlass
                border.width: 1
                border.color: Theme.shellStrokeStrong

                RowLayout {
                    anchors.fill: parent
                    anchors.margins: Theme.space5
                    spacing: Theme.space5

                    ColumnLayout {
                        Layout.fillWidth: true
                        spacing: 4

                        Label {
                            text: "VELYX"
                            color: Theme.accentCoolStrong
                            font.family: Theme.fontSans
                            font.pixelSize: 11
                            font.weight: Font.DemiBold
                            letterSpacing: 2
                        }

                        Label {
                            text: permissionClient.activeSpaceName.length > 0
                                ? permissionClient.activeSpaceName
                                : "Context-driven operating environment"
                            color: Theme.textPrimary
                            font.family: Theme.fontDisplay
                            font.pixelSize: 30
                            font.weight: Font.DemiBold
                        }

                        Label {
                            text: window.systemConfidence + "  •  " +
                                (permissionClient.activeAppTitle.length > 0
                                    ? "Active app: " + permissionClient.activeAppTitle
                                    : "Choose a space and move into work")
                            color: Theme.textSecondary
                            font.pixelSize: 13
                        }
                    }

                    StatusChip {
                        compact: true
                        label: "Confidence"
                        value: window.systemConfidence
                        tone: window.systemConfidence === "Ready" ? "success"
                            : (window.systemConfidence === "Recovery required" ? "danger" : "warning")
                    }

                    StatusChip {
                        compact: true
                        label: "Mode"
                        value: permissionClient.activeSpaceSecurityMode.length > 0 ? permissionClient.activeSpaceSecurityMode : "standard"
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
                        label: "Runtime"
                        value: permissionClient.currentVersion.length > 0 ? permissionClient.currentVersion : "unknown"
                        tone: "accent"
                    }

                    StatusChip {
                        compact: true
                        label: "AI"
                        value: permissionClient.aiMode
                        tone: permissionClient.aiMode === "auto" ? "warning"
                            : (permissionClient.aiMode === "suggest" ? "accent" : "neutral")
                    }
                }
            }

            RowLayout {
                Layout.fillWidth: true
                Layout.fillHeight: true
                spacing: Theme.space4

                Rectangle {
                    Layout.preferredWidth: 300
                    Layout.fillHeight: true
                    radius: Theme.radiusXl
                    color: Theme.shellSurface
                    border.width: 1
                    border.color: Theme.shellStroke

                    ColumnLayout {
                        anchors.fill: parent
                        anchors.margins: Theme.space5
                        spacing: Theme.space4

                        SectionHeader {
                            Layout.fillWidth: true
                            title: "Contexts"
                            subtitle: "Choose the space first. The rest of Velyx follows that context."
                        }

                        Button {
                            text: "Refresh workspace"
                            onClicked: {
                                permissionClient.refreshRuntimeStatus()
                                permissionClient.refreshSpaces()
                                permissionClient.refreshIntents()
                                permissionClient.refreshRules()
                                permissionClient.refreshAgentState()
                                permissionClient.refreshAiState()
                                permissionClient.refreshAssistantState()
                                permissionClient.refreshOpenApps()
                                permissionClient.refreshApps()
                            }
                        }

                        ScrollView {
                            Layout.fillWidth: true
                            Layout.fillHeight: true
                            clip: true

                            Column {
                                width: parent.width
                                spacing: Theme.space3

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
                    spacing: Theme.space4

                    SpaceOverviewPanel {
                        Layout.fillWidth: true
                        Layout.preferredHeight: 252
                        permissionClient: permissionClient
                        inSpaceCount: window.appsInActiveSpace.length
                        runningInSpaceCount: window.runningInActiveSpace.length
                        outsideCount: window.runningOutsideSpace.length
                    }

                    RowLayout {
                        Layout.fillWidth: true
                        Layout.preferredHeight: 282
                        spacing: Theme.space4

                        SuggestedActionsPanel {
                            Layout.fillWidth: true
                            Layout.fillHeight: true
                            permissionClient: permissionClient
                            intentsModel: window.suggestedIntents
                        }

                        SystemStatusPanel {
                            Layout.preferredWidth: 390
                            Layout.fillHeight: true
                            permissionClient: permissionClient
                        }
                    }

                    Rectangle {
                        Layout.fillWidth: true
                        Layout.fillHeight: true
                        radius: Theme.radiusXl
                        color: Theme.shellSurface
                        border.width: 1
                        border.color: Theme.shellStroke

                        ColumnLayout {
                            anchors.fill: parent
                            anchors.margins: Theme.space5
                            spacing: Theme.space4

                            SectionHeader {
                                Layout.fillWidth: true
                                title: "Apps serving this space"
                                subtitle: "Primary tools for the active context. Operational details stay secondary."
                            }

                            Rectangle {
                                Layout.fillWidth: true
                                visible: window.appsInActiveSpace.length === 0
                                radius: Theme.radiusMd
                                color: Qt.rgba(1, 1, 1, 0.03)
                                border.width: 1
                                border.color: Theme.shellStroke
                                implicitHeight: 96

                                ColumnLayout {
                                    anchors.fill: parent
                                    anchors.margins: Theme.space4
                                    spacing: 6

                                    Label {
                                        text: "No apps in this space"
                                        color: Theme.textPrimary
                                        font.pixelSize: 14
                                        font.weight: Font.DemiBold
                                    }

                                    Label {
                                        text: permissionClient.recoveryNeeded
                                            ? "Recovery is currently the highest-priority action."
                                            : "Run a suggested action or install apps that belong to this context."
                                        color: Theme.textMuted
                                        font.pixelSize: 12
                                        wrapMode: Text.WordWrap
                                    }
                                }
                            }

                            ScrollView {
                                Layout.fillWidth: true
                                Layout.fillHeight: true
                                clip: true
                                visible: window.appsInActiveSpace.length > 0

                                GridLayout {
                                    width: parent.width
                                    columns: width > 920 ? 2 : 1
                                    columnSpacing: Theme.space3
                                    rowSpacing: Theme.space3

                                    Repeater {
                                        model: window.appsInActiveSpace

                                        delegate: AppCard {
                                            Layout.fillWidth: true
                                            Layout.minimumWidth: 320
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
                        Layout.preferredHeight: 270
                        spacing: Theme.space4

                        OpenAppsPanel {
                            Layout.fillWidth: true
                            Layout.fillHeight: true
                            title: "Running in this space"
                            subtitle: "Live windows aligned with the current context."
                            appsModel: window.runningInActiveSpace
                            permissionClient: permissionClient
                        }

                        OpenAppsPanel {
                            Layout.fillWidth: true
                            Layout.fillHeight: true
                            title: "Outside current space"
                            subtitle: "Still visible, but not central to the active mode."
                            appsModel: window.runningOutsideSpace
                            permissionClient: permissionClient
                        }
                    }
                }

                ColumnLayout {
                    Layout.preferredWidth: 390
                    Layout.fillHeight: true
                    spacing: Theme.space4

                    AssistantPanel {
                        Layout.fillWidth: true
                        permissionClient: permissionClient
                    }

                    AiSuggestionPanel {
                        Layout.fillWidth: true
                        permissionClient: permissionClient
                    }

                    DetailsPanel {
                        Layout.fillWidth: true
                        Layout.fillHeight: true
                        permissionClient: permissionClient
                    }

                    AutomationPanel {
                        Layout.fillWidth: true
                        permissionClient: permissionClient
                    }

                    DiagnosticsPanel {
                        Layout.fillWidth: true
                        permissionClient: permissionClient
                    }
                }
            }

            Rectangle {
                Layout.fillWidth: true
                Layout.preferredHeight: 58
                radius: Theme.radiusLg
                color: Theme.shellSurfaceGlass
                border.width: 1
                border.color: Theme.shellStroke

                RowLayout {
                    anchors.fill: parent
                    anchors.margins: Theme.space4
                    spacing: Theme.space4

                    Label {
                        text: "Last action"
                        color: Theme.textMuted
                        font.pixelSize: 12
                    }

                    Label {
                        Layout.fillWidth: true
                        text: permissionClient.lastAction.length > 0
                            ? permissionClient.lastAction + "  •  " + permissionClient.lastResult + "  •  " + permissionClient.lastReason
                            : "The system is waiting for the next action."
                        color: Theme.textPrimary
                        font.pixelSize: 12
                        elide: Text.ElideRight
                    }

                    Label {
                        text: permissionClient.shortcutFeedback.length > 0
                            ? permissionClient.shortcutFeedback
                            : permissionClient.inputControlMode
                        color: Theme.textMuted
                        font.pixelSize: 12
                    }
                }
            }
        }
    }

    FirstBootOverlay {
        permissionClient: permissionClient
    }
}
