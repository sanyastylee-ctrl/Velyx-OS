import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Velyx.DesignSystem
import Velyx.UI

Rectangle {
    id: root

    required property var permissionClient

    radius: Theme.radiusLg
    color: Qt.rgba(Theme.accentCool.r, Theme.accentCool.g, Theme.accentCool.b, 0.08)
    border.width: 1
    border.color: root.permissionClient.devModeEnabled ? Theme.accentCool : Theme.shellStroke
    implicitHeight: root.permissionClient.devModeEnabled ? 980 : 132

    function localImageSource(path) {
        if (!path || path.length === 0)
            return ""
        var normalized = path.replace(/\\/g, "/")
        if (/^[A-Za-z]:/.test(normalized))
            return "file:///" + normalized
        return "file://" + normalized
    }

    function strategyTone(strategy) {
        if (strategy === "live_apply")
            return "success"
        if (strategy === "staged_update")
            return "warning"
        if (strategy === "reboot_required")
            return "danger"
        if (strategy === "deny")
            return "danger"
        return "accent"
    }

    function strategyTitle(strategy) {
        if (strategy === "live_apply")
            return "This is a live change."
        if (strategy === "staged_update")
            return "This change requires staged update."
        if (strategy === "reboot_required")
            return "This change needs a restart or reboot path."
        if (strategy === "deny")
            return "This request is blocked in the current Dev Mode."
        return "Dev Agent is ready for the next iteration."
    }

    function strategyMessage(strategy) {
        if (strategy === "live_apply")
            return "Velyx can preview, apply, reload the shell, capture screenshots, and iterate without reinstall or update."
        if (strategy === "staged_update")
            return "The assistant will stage the patch and keep the change honest. It will not pretend that a runtime update can be applied live."
        if (strategy === "reboot_required")
            return "This touches deeper runtime paths. Keep the change staged and plan for a restart or reboot cycle."
        if (strategy === "deny")
            return "The requested change is outside the allowed development scope or current policy."
        return "Use Dev Mode for compact UI edits, visual polish, and bounded shell development."
    }

    function joinList(value) {
        if (!value)
            return ""
        if (Array.isArray(value))
            return value.join(", ")
        if (typeof value === "string")
            return value
        var items = []
        for (var i = 0; i < value.length; ++i)
            items.push(value[i])
        return items.join(", ")
    }

    function stopDevFlow() {
        if (root.permissionClient.devAutoRefine)
            root.permissionClient.setDevAutoRefine(false)
        else
            root.permissionClient.disableDevMode()
    }

    ScrollView {
        anchors.fill: parent
        clip: true

        ColumnLayout {
            width: parent.width
            spacing: Theme.space3

            SectionHeader {
                Layout.fillWidth: true
                title: root.permissionClient.devModeEnabled ? "Dev Mode Active" : "Dev Mode"
                subtitle: root.permissionClient.devModeEnabled
                    ? "Preview, apply, reload, analyze, and roll back shell changes directly inside Velyx."
                    : "Hidden by default. Enable it only when you want controlled live UI editing and system development planning."
            }

            Flow {
                Layout.fillWidth: true
                spacing: 8

                StatusChip {
                    compact: true
                    label: "State"
                    value: root.permissionClient.devModeEnabled ? "Dev Mode Active" : "Off"
                    tone: root.permissionClient.devModeEnabled ? "warning" : "neutral"
                }

                StatusChip {
                    compact: true
                    label: "Mode"
                    value: root.permissionClient.devAgentMode.length > 0 ? root.permissionClient.devAgentMode : "disabled"
                    tone: root.permissionClient.devAgentMode === "full_dev" ? "warning"
                        : (root.permissionClient.devModeEnabled ? "accent" : "neutral")
                }

                StatusChip {
                    compact: true
                    label: "Apply"
                    value: root.permissionClient.devApplyStrategy.length > 0 ? root.permissionClient.devApplyStrategy : "idle"
                    tone: root.strategyTone(root.permissionClient.devApplyStrategy)
                }

                StatusChip {
                    compact: true
                    label: "Validation"
                    value: root.permissionClient.devValidationStatus.length > 0 ? root.permissionClient.devValidationStatus : "pending"
                    tone: root.permissionClient.devValidationStatus === "failed" ? "danger"
                        : (root.permissionClient.devValidationStatus.length > 0 ? "accent" : "neutral")
                }

                StatusChip {
                    compact: true
                    label: "Visual"
                    value: root.permissionClient.devVisualFeedbackActive ? "Visual feedback active" : "Fallback"
                    tone: root.permissionClient.devVisualFeedbackActive ? "accent" : "neutral"
                }

                StatusChip {
                    compact: true
                    label: "Auto refine"
                    value: root.permissionClient.devAutoRefine ? "On" : "Off"
                    tone: root.permissionClient.devAutoRefine ? "warning" : "neutral"
                }
            }

            Rectangle {
                Layout.fillWidth: true
                visible: root.permissionClient.devModeEnabled
                radius: Theme.radiusMd
                color: Qt.rgba(Theme.accentCool.r, Theme.accentCool.g, Theme.accentCool.b, 0.08)
                border.width: 1
                border.color: Theme.shellStroke
                implicitHeight: 120

                ColumnLayout {
                    anchors.fill: parent
                    anchors.margins: Theme.space4
                    spacing: 6

                    Label {
                        text: root.strategyTitle(root.permissionClient.devApplyStrategy)
                        color: Theme.textPrimary
                        font.pixelSize: 13
                        font.weight: Font.DemiBold
                    }

                    Label {
                        Layout.fillWidth: true
                        text: root.permissionClient.devPlanSummary.length > 0
                            ? root.permissionClient.devPlanSummary
                            : root.strategyMessage(root.permissionClient.devApplyStrategy)
                        color: Theme.textSecondary
                        wrapMode: Text.WordWrap
                        font.pixelSize: 12
                    }

                    Label {
                        Layout.fillWidth: true
                        visible: root.permissionClient.devLastRequest.length > 0
                        text: "Request: " + root.permissionClient.devLastRequest
                        color: Theme.textMuted
                        wrapMode: Text.WordWrap
                        font.pixelSize: 11
                    }
                }
            }

            RowLayout {
                Layout.fillWidth: true
                spacing: 8

                Button {
                    text: root.permissionClient.devModeEnabled ? "Disable Dev" : "Enable Dev"
                    onClicked: root.permissionClient.devModeEnabled
                        ? root.permissionClient.disableDevMode()
                        : root.permissionClient.enableDevMode()
                }

                Button {
                    text: "UI Live"
                    enabled: root.permissionClient.devModeEnabled
                    onClicked: root.permissionClient.setDevAgentMode("ui_live_only")
                }

                Button {
                    text: "Full Dev"
                    enabled: root.permissionClient.devModeEnabled
                    onClicked: root.permissionClient.setDevAgentMode("full_dev")
                }

                Item { Layout.fillWidth: true }

                Button {
                    text: "Apply"
                    enabled: root.permissionClient.devModeEnabled && root.permissionClient.devPendingRefinement.length > 0
                    onClicked: root.permissionClient.applyNextDevRefinement()
                }

                Button {
                    text: "Rollback"
                    enabled: root.permissionClient.devModeEnabled
                    onClicked: root.permissionClient.rollbackDevMode()
                }

                Button {
                    text: "Stop"
                    enabled: root.permissionClient.devModeEnabled
                    onClicked: root.stopDevFlow()
                }

                Button {
                    text: "Restart Shell"
                    enabled: root.permissionClient.devModeEnabled
                    onClicked: root.permissionClient.restartShellDev()
                }

                Button {
                    text: root.permissionClient.devAutoRefine ? "Auto Refine On" : "Toggle Auto Refine"
                    enabled: root.permissionClient.devModeEnabled
                    onClicked: root.permissionClient.setDevAutoRefine(!root.permissionClient.devAutoRefine)
                }
            }

            Rectangle {
                Layout.fillWidth: true
                visible: root.permissionClient.devModeEnabled
                radius: Theme.radiusMd
                color: Theme.shellSurface
                border.width: 1
                border.color: Theme.shellStroke
                implicitHeight: 118

                ColumnLayout {
                    anchors.fill: parent
                    anchors.margins: Theme.space4
                    spacing: 6

                    Label {
                        text: root.permissionClient.devLastChange.length > 0
                            ? "Last change: " + root.permissionClient.devLastChange
                            : "No live edits yet. Ask Velyx to tighten spacing, move a panel, or compact the layout."
                        color: Theme.textPrimary
                        wrapMode: Text.WordWrap
                        font.pixelSize: 12
                    }

                    Label {
                        Layout.fillWidth: true
                        text: "Scope: "
                            + (root.permissionClient.devScope.length > 0 ? root.permissionClient.devScope : "unspecified")
                            + "  |  Approval: "
                            + (root.permissionClient.devApprovalLevel.length > 0 ? root.permissionClient.devApprovalLevel : "pending")
                        color: Theme.textSecondary
                        wrapMode: Text.WordWrap
                        font.pixelSize: 11
                    }

                    Label {
                        Layout.fillWidth: true
                        visible: root.permissionClient.devOverlayPath.length > 0
                        text: "Overlay: " + root.permissionClient.devOverlayPath
                        color: Theme.textMuted
                        wrapMode: Text.WordWrap
                        font.pixelSize: 11
                    }
                }
            }

            Rectangle {
                Layout.fillWidth: true
                visible: root.permissionClient.devModeEnabled
                radius: Theme.radiusMd
                color: Theme.shellSurface
                border.width: 1
                border.color: Theme.shellStroke
                implicitHeight: 122

                ColumnLayout {
                    anchors.fill: parent
                    anchors.margins: Theme.space4
                    spacing: 6

                    Label {
                        text: "Affected files"
                        color: Theme.textPrimary
                        font.pixelSize: 13
                        font.weight: Font.DemiBold
                    }

                    Label {
                        Layout.fillWidth: true
                        text: root.permissionClient.devAffectedFiles.length > 0
                            ? root.joinList(root.permissionClient.devAffectedFiles)
                            : "The next dev plan will list the files it wants to touch before apply."
                        color: Theme.textSecondary
                        wrapMode: Text.WordWrap
                        font.pixelSize: 12
                    }
                }
            }

            Rectangle {
                Layout.fillWidth: true
                visible: root.permissionClient.devModeEnabled
                radius: Theme.radiusMd
                color: Qt.rgba(1, 1, 1, 0.03)
                border.width: 1
                border.color: Theme.shellStroke
                implicitHeight: 188

                ColumnLayout {
                    anchors.fill: parent
                    anchors.margins: Theme.space3
                    spacing: Theme.space3

                    Label {
                        text: "Visual feedback"
                        color: Theme.textPrimary
                        font.pixelSize: 13
                        font.weight: Font.DemiBold
                    }

                    RowLayout {
                        Layout.fillWidth: true
                        Layout.fillHeight: true
                        spacing: Theme.space3

                        Rectangle {
                            Layout.fillWidth: true
                            Layout.fillHeight: true
                            radius: Theme.radiusMd
                            color: Theme.shellSurface
                            border.width: 1
                            border.color: Theme.shellStroke

                            ColumnLayout {
                                anchors.fill: parent
                                anchors.margins: Theme.space2
                                spacing: 4

                                Label {
                                    text: "Before"
                                    color: Theme.textMuted
                                    font.pixelSize: 11
                                }

                                Image {
                                    Layout.fillWidth: true
                                    Layout.fillHeight: true
                                    fillMode: Image.PreserveAspectFit
                                    cache: false
                                    source: root.localImageSource(root.permissionClient.devPreviousScreenshotPath)
                                }
                            }
                        }

                        Rectangle {
                            Layout.fillWidth: true
                            Layout.fillHeight: true
                            radius: Theme.radiusMd
                            color: Theme.shellSurface
                            border.width: 1
                            border.color: Theme.shellStroke

                            ColumnLayout {
                                anchors.fill: parent
                                anchors.margins: Theme.space2
                                spacing: 4

                                Label {
                                    text: "After"
                                    color: Theme.textMuted
                                    font.pixelSize: 11
                                }

                                Image {
                                    Layout.fillWidth: true
                                    Layout.fillHeight: true
                                    fillMode: Image.PreserveAspectFit
                                    cache: false
                                    source: root.localImageSource(root.permissionClient.devLastScreenshotPath)
                                }
                            }
                        }
                    }
                }
            }

            Rectangle {
                Layout.fillWidth: true
                visible: root.permissionClient.devModeEnabled
                radius: Theme.radiusMd
                color: Qt.rgba(Theme.warning.r, Theme.warning.g, Theme.warning.b, 0.08)
                border.width: 1
                border.color: Theme.shellStroke
                implicitHeight: 118

                ColumnLayout {
                    anchors.fill: parent
                    anchors.margins: Theme.space3
                    spacing: 6

                    Label {
                        text: root.permissionClient.devVisualSummary.length > 0
                            ? root.permissionClient.devVisualSummary
                            : "After each live patch Velyx can capture a screenshot, describe what changed, and suggest one more refinement."
                        color: Theme.textPrimary
                        font.pixelSize: 12
                        wrapMode: Text.WordWrap
                    }

                    Label {
                        visible: root.permissionClient.devVisualRecommendation.length > 0
                        text: root.permissionClient.devVisualRecommendation
                        color: Theme.textSecondary
                        font.pixelSize: 11
                        wrapMode: Text.WordWrap
                    }

                    Label {
                        visible: root.permissionClient.devPendingRefinement.length > 0
                        text: "Next refinement: " + root.permissionClient.devPendingRefinement + "  |  Iterations are limited for safety."
                        color: Theme.textMuted
                        font.pixelSize: 11
                        wrapMode: Text.WordWrap
                    }
                }
            }

            Rectangle {
                Layout.fillWidth: true
                visible: root.permissionClient.devModeEnabled
                radius: Theme.radiusMd
                color: Theme.shellSurface
                border.width: 1
                border.color: Theme.shellStroke
                implicitHeight: 260

                ColumnLayout {
                    anchors.fill: parent
                    anchors.margins: Theme.space4
                    spacing: Theme.space3

                    RowLayout {
                        Layout.fillWidth: true

                        Label {
                            text: "Dev history"
                            color: Theme.textPrimary
                            font.pixelSize: 13
                            font.weight: Font.DemiBold
                        }

                        Item { Layout.fillWidth: true }

                        Label {
                            text: root.permissionClient.devHistory.length > 0
                                ? root.permissionClient.devHistory.length + " recent changes"
                                : "No changes yet"
                            color: Theme.textMuted
                            font.pixelSize: 11
                        }
                    }

                    ScrollView {
                        Layout.fillWidth: true
                        Layout.fillHeight: true
                        clip: true

                        Column {
                            width: parent.width
                            spacing: Theme.space2

                            Repeater {
                                model: root.permissionClient.devHistory

                                delegate: Rectangle {
                                    required property var modelData
                                    width: parent.width
                                    radius: Theme.radiusMd
                                    color: Qt.rgba(1, 1, 1, 0.03)
                                    border.width: 1
                                    border.color: Theme.shellStroke
                                    implicitHeight: changeColumn.implicitHeight + Theme.space4 * 2

                                    Column {
                                        id: changeColumn
                                        anchors.fill: parent
                                        anchors.margins: Theme.space4
                                        spacing: 4

                                        Label {
                                            width: parent.width
                                            text: modelData.summary || "Dev change"
                                            color: Theme.textPrimary
                                            wrapMode: Text.WordWrap
                                            font.pixelSize: 12
                                            font.weight: Font.DemiBold
                                        }

                                        Label {
                                            width: parent.width
                                            text: "Type: "
                                                + (modelData.classified_type || "unknown")
                                                + "  |  Apply: "
                                                + (modelData.apply_mode || "unknown")
                                            color: Theme.textSecondary
                                            wrapMode: Text.WordWrap
                                            font.pixelSize: 11
                                        }

                                        Label {
                                            width: parent.width
                                            visible: (modelData.affected_files || []).length > 0
                                            text: "Files: " + root.joinList(modelData.affected_files || [])
                                            color: Theme.textMuted
                                            wrapMode: Text.WordWrap
                                            font.pixelSize: 11
                                        }

                                        RowLayout {
                                            width: parent.width
                                            spacing: 8

                                            Label {
                                                Layout.fillWidth: true
                                                text: modelData.change_id || ""
                                                color: Theme.textMuted
                                                font.pixelSize: 10
                                                elide: Text.ElideRight
                                            }

                                            Button {
                                                text: "Rollback selected"
                                                onClicked: root.permissionClient.restoreDevChange(modelData.change_id || "")
                                            }
                                        }
                                    }
                                }
                            }

                            Label {
                                width: parent.width
                                visible: root.permissionClient.devHistory.length === 0
                                text: "Your dev changes will appear here with type, affected files, and rollback controls."
                                color: Theme.textMuted
                                wrapMode: Text.WordWrap
                                font.pixelSize: 11
                            }
                        }
                    }
                }
            }
        }
    }
}
