import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import QtQuick.Window
import Velyx.DesignSystem
import Velyx.UI

ApplicationWindow {
    id: window

    readonly property var runtimeInfo: shellRuntime || ({})
    readonly property string environmentMode: runtimeInfo.environmentMode || "vm"
    readonly property bool vmMode: environmentMode === "vm"
    readonly property bool bareMetalMode: environmentMode === "bare-metal"
    readonly property real uiScale: runtimeInfo.uiScale || 1.0
    readonly property real screenDpr: runtimeInfo.devicePixelRatio || Screen.devicePixelRatio || 1.0
    readonly property real screenDpi: runtimeInfo.pixelDensity || 96.0

    function scaled(value) {
        return Math.round(value * window.uiScale)
    }

    width: Screen.width > 0 ? Screen.width : 1366
    height: Screen.height > 0 ? Screen.height : 768
    visible: true
    visibility: Window.FullScreen
    flags: Qt.FramelessWindowHint
    color: Theme.shellBg
    title: "Velyx Shell"
    readonly property var permissionBridge: permissionClient
    readonly property bool firstBootActive: permissionClient.firstBootRequired
    readonly property bool compactMode: width < 1366 || height < 820
    readonly property bool vmPerformanceMode: vmMode && (compactMode || firstBootActive)
    readonly property int shellMargin: scaled(vmPerformanceMode ? Theme.space4 : Theme.space6)
    readonly property int shellSpacing: scaled(vmPerformanceMode ? Theme.space3 : Theme.space4)
    readonly property int headerHeight: scaled(vmPerformanceMode ? 88 : 108)
    readonly property int leftRailWidth: scaled(vmPerformanceMode ? 252 : 300)
    readonly property int rightRailWidth: scaled(vmPerformanceMode ? 332 : 390)
    readonly property int overviewHeight: scaled(vmPerformanceMode ? 208 : 252)
    readonly property int statusRowHeight: scaled(vmPerformanceMode ? 240 : 282)
    readonly property int runningAppsHeight: scaled(vmPerformanceMode ? 224 : 270)
    readonly property int footerHeight: scaled(vmPerformanceMode ? 52 : 58)

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

    function refreshWorkspaceState() {
        permissionClient.refreshRuntimeStatus()
        permissionClient.refreshSpaces()
        permissionClient.refreshIntents()
        permissionClient.refreshRules()
        permissionClient.refreshAgentState()
        permissionClient.refreshAiState()
        permissionClient.refreshAssistantState()
        permissionClient.refreshDevModeState()
        permissionClient.refreshFirstBootState()
        permissionClient.refreshOpenApps()
        permissionClient.refreshSelectedAppRuntime()
        permissionClient.refreshApps()
    }

    function refreshVmState() {
        permissionClient.refreshRuntimeStatus()
        permissionClient.refreshFirstBootState()
        if (!window.firstBootActive) {
            permissionClient.refreshSpaces()
            permissionClient.refreshOpenApps()
            permissionClient.refreshSelectedAppRuntime()
            permissionClient.refreshApps()
        }
    }

    Component.onCompleted: {
        permissionClient.refreshFirstBootState()
        if (window.vmPerformanceMode)
            window.refreshVmState()
        else
            window.refreshWorkspaceState()
        window.contentItem.forceActiveFocus()
        console.info("shell startup mode environmentMode=" + window.environmentMode
            + " vmPerformanceMode=" + window.vmPerformanceMode
            + " compactMode=" + window.compactMode
            + " uiScale=" + window.uiScale.toFixed(2)
            + " dpi=" + window.screenDpi.toFixed(1)
            + " dpr=" + window.screenDpr.toFixed(2)
            + " size=" + window.width + "x" + window.height)
    }

    HoverHandler {
        acceptedDevices: PointerDevice.Mouse | PointerDevice.TouchPad
        cursorShape: Qt.ArrowCursor
    }

    Timer {
        interval: 1500
        running: true
        repeat: false
        onTriggered: console.info("GUI fully initialized")
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
        interval: window.vmPerformanceMode ? 6000 : 3000
        running: true
        repeat: true
        onTriggered: {
            if (window.vmPerformanceMode)
                window.refreshVmState()
            else
                window.refreshWorkspaceState()
        }
    }

    Timer {
        interval: 15000
        running: window.vmPerformanceMode && !window.firstBootActive
        repeat: true
        onTriggered: {
            permissionClient.refreshIntents()
            permissionClient.refreshRules()
            permissionClient.refreshAgentState()
            permissionClient.refreshAiState()
            permissionClient.refreshAssistantState()
            permissionClient.refreshDevModeState()
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
            anchors.margins: window.shellMargin
            spacing: window.shellSpacing

            Rectangle {
                Layout.fillWidth: true
                Layout.preferredHeight: window.headerHeight
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
                            font.letterSpacing: 2
                        }

                        Label {
                            text: permissionClient.activeSpaceName.length > 0
                                ? permissionClient.activeSpaceName
                                : "Context-driven operating environment"
                            color: Theme.textPrimary
                            font.family: Theme.fontDisplay
                            font.pixelSize: window.scaled(30)
                            font.weight: Font.DemiBold
                        }

                        Label {
                            text: window.systemConfidence + "  •  " +
                                (permissionClient.activeAppTitle.length > 0
                                    ? "Active app: " + permissionClient.activeAppTitle
                                    : "Choose a space and move into work")
                            color: Theme.textSecondary
                            font.pixelSize: window.scaled(13)
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
                        visible: !window.vmPerformanceMode
                        compact: true
                        label: "AI"
                        value: permissionClient.aiMode
                        tone: permissionClient.aiMode === "auto" ? "warning"
                            : (permissionClient.aiMode === "suggest" ? "accent" : "neutral")
                    }

                    StatusChip {
                        visible: permissionClient.devModeEnabled
                        compact: true
                        label: "Dev Mode"
                        value: "Active"
                        tone: "warning"
                    }
                }
            }

            Loader {
                Layout.fillWidth: true
                Layout.fillHeight: true
                asynchronous: false
                sourceComponent: window.firstBootActive ? vmBackdropComponent : workspaceSceneComponent
            }

            Rectangle {
                Layout.fillWidth: true
                Layout.preferredHeight: window.footerHeight
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
                        font.pixelSize: window.scaled(12)
                    }

                    Label {
                        Layout.fillWidth: true
                        text: permissionClient.lastAction.length > 0
                            ? permissionClient.lastAction + "  •  " + permissionClient.lastResult + "  •  " + permissionClient.lastReason
                            : "The system is waiting for the next action."
                        color: Theme.textPrimary
                        font.pixelSize: window.scaled(12)
                        elide: Text.ElideRight
                    }

                    Label {
                        text: permissionClient.shortcutFeedback.length > 0
                            ? permissionClient.shortcutFeedback
                            : permissionClient.inputControlMode
                        color: Theme.textMuted
                        font.pixelSize: window.scaled(12)
                    }
                }
            }
        }
    }

    FirstBootOverlay {
        permissionClient: window.permissionBridge
    }

    Component {
        id: vmBackdropComponent

        RowLayout {
            spacing: window.shellSpacing

            Rectangle {
                Layout.preferredWidth: window.leftRailWidth
                Layout.fillHeight: true
                radius: Theme.radiusXl
                color: Theme.shellSurface
                border.width: 1
                border.color: Theme.shellStroke

                ColumnLayout {
                    anchors.fill: parent
                    anchors.margins: Theme.space4
                    spacing: Theme.space3

                    SectionHeader {
                        Layout.fillWidth: true
                        title: "Contexts"
                        subtitle: "Preview mode keeps the background light while setup is active."
                    }

                    Rectangle {
                        Layout.fillWidth: true
                        Layout.fillHeight: true
                        radius: Theme.radiusLg
                        color: Qt.rgba(1, 1, 1, 0.025)
                        border.width: 1
                        border.color: Theme.shellStroke
                    }
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
                    anchors.margins: Theme.space4
                    spacing: Theme.space3

                    SectionHeader {
                        Layout.fillWidth: true
                        title: "Shell Preview"
                        subtitle: "Heavy panels load after First Boot completes so setup stays responsive in VM."
                    }

                    Rectangle {
                        Layout.fillWidth: true
                        Layout.preferredHeight: window.overviewHeight
                        radius: Theme.radiusLg
                        color: Qt.rgba(1, 1, 1, 0.025)
                        border.width: 1
                        border.color: Theme.shellStroke
                    }

                    RowLayout {
                        Layout.fillWidth: true
                        Layout.fillHeight: true
                        spacing: Theme.space3

                        Rectangle {
                            Layout.fillWidth: true
                            Layout.fillHeight: true
                            radius: Theme.radiusLg
                            color: Qt.rgba(1, 1, 1, 0.025)
                            border.width: 1
                            border.color: Theme.shellStroke
                        }

                        Rectangle {
                            Layout.preferredWidth: window.rightRailWidth
                            Layout.fillHeight: true
                            radius: Theme.radiusLg
                            color: Qt.rgba(1, 1, 1, 0.025)
                            border.width: 1
                            border.color: Theme.shellStroke
                        }
                    }
                }
            }
        }
    }

    Component {
        id: workspaceSceneComponent

        RowLayout {
            spacing: window.shellSpacing

            Rectangle {
                Layout.preferredWidth: window.leftRailWidth
                Layout.fillHeight: true
                radius: Theme.radiusXl
                color: Theme.shellSurface
                border.width: 1
                border.color: Theme.shellStroke

                ColumnLayout {
                    anchors.fill: parent
                    anchors.margins: Theme.space5
                    spacing: window.shellSpacing

                    SectionHeader {
                        Layout.fillWidth: true
                        title: "Contexts"
                        subtitle: "Choose the space first. The rest of Velyx follows that context."
                    }

                    Button {
                        text: "Refresh workspace"
                        onClicked: window.refreshWorkspaceState()
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
                spacing: window.shellSpacing

                SpaceOverviewPanel {
                    Layout.fillWidth: true
                    Layout.preferredHeight: window.overviewHeight
                    permissionClient: window.permissionBridge
                    inSpaceCount: window.appsInActiveSpace.length
                    runningInSpaceCount: window.runningInActiveSpace.length
                    outsideCount: window.runningOutsideSpace.length
                }

                RowLayout {
                    Layout.fillWidth: true
                    Layout.preferredHeight: window.statusRowHeight
                    spacing: window.shellSpacing

                    SuggestedActionsPanel {
                        Layout.fillWidth: true
                        Layout.fillHeight: true
                        permissionClient: window.permissionBridge
                        intentsModel: window.suggestedIntents
                    }

                    SystemStatusPanel {
                        Layout.preferredWidth: window.rightRailWidth
                        Layout.fillHeight: true
                        permissionClient: window.permissionBridge
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
                        spacing: window.shellSpacing

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
                                columns: width > (window.vmPerformanceMode ? 980 : 920) ? 2 : 1
                                columnSpacing: window.scaled(Theme.space3)
                                rowSpacing: window.scaled(Theme.space3)

                                Repeater {
                                    model: window.appsInActiveSpace

                                    delegate: AppCard {
                                        Layout.fillWidth: true
                                        Layout.minimumWidth: window.vmPerformanceMode ? 280 : 320
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
                    Layout.preferredHeight: window.runningAppsHeight
                    spacing: window.shellSpacing

                    OpenAppsPanel {
                        Layout.fillWidth: true
                        Layout.fillHeight: true
                        title: "Running in this space"
                        subtitle: "Live windows aligned with the current context."
                        appsModel: window.runningInActiveSpace
                        permissionClient: window.permissionBridge
                    }

                    OpenAppsPanel {
                        Layout.fillWidth: true
                        Layout.fillHeight: true
                        title: "Outside current space"
                        subtitle: "Still visible, but not central to the active mode."
                        appsModel: window.runningOutsideSpace
                        permissionClient: window.permissionBridge
                    }
                }
            }

            ScrollView {
                Layout.preferredWidth: window.rightRailWidth
                Layout.fillHeight: true
                clip: true

                ColumnLayout {
                    width: parent.width
                    spacing: window.shellSpacing

                    ControlCenterPanel {
                        Layout.fillWidth: true
                        permissionClient: window.permissionBridge
                    }

                    AssistantPanel {
                        Layout.fillWidth: true
                        permissionClient: window.permissionBridge
                    }

                    Loader {
                        Layout.fillWidth: true
                        active: !window.vmPerformanceMode && permissionClient.devModeEnabled
                        sourceComponent: devModePanelComponent
                    }

                    Loader {
                        Layout.fillWidth: true
                        active: !window.vmPerformanceMode
                        sourceComponent: aiSuggestionPanelComponent
                    }

                    DetailsPanel {
                        Layout.fillWidth: true
                        permissionClient: window.permissionBridge
                    }

                    Loader {
                        Layout.fillWidth: true
                        active: !window.vmPerformanceMode
                        sourceComponent: automationPanelComponent
                    }

                    Loader {
                        Layout.fillWidth: true
                        active: !window.vmPerformanceMode
                        sourceComponent: diagnosticsPanelComponent
                    }
                }
            }
        }
    }

    Component {
        id: devModePanelComponent

        DevModePanel {
            width: parent ? parent.width : 0
            permissionClient: window.permissionBridge
        }
    }

    Component {
        id: aiSuggestionPanelComponent

        AiSuggestionPanel {
            width: parent ? parent.width : 0
            permissionClient: window.permissionBridge
        }
    }

    Component {
        id: automationPanelComponent

        AutomationPanel {
            width: parent ? parent.width : 0
            permissionClient: window.permissionBridge
        }
    }

    Component {
        id: diagnosticsPanelComponent

        DiagnosticsPanel {
            width: parent ? parent.width : 0
            permissionClient: window.permissionBridge
        }
    }
}
