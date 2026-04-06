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
    implicitHeight: root.permissionClient.devModeEnabled ? 470 : 108

    function localImageSource(path) {
        if (!path || path.length === 0)
            return ""
        var normalized = path.replace(/\\/g, "/")
        if (/^[A-Za-z]:/.test(normalized))
            return "file:///" + normalized
        return "file://" + normalized
    }

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: Theme.space4
        spacing: Theme.space3

        SectionHeader {
            Layout.fillWidth: true
            title: "Dev Mode"
            subtitle: root.permissionClient.devModeEnabled
                ? "Live UI editing and visual feedback are active."
                : "Hidden by default. Enable it only for controlled shell UI iteration."
        }

        Flow {
            Layout.fillWidth: true
            spacing: 8

            StatusChip {
                compact: true
                label: "State"
                value: root.permissionClient.devModeEnabled ? "Active" : "Off"
                tone: root.permissionClient.devModeEnabled ? "warning" : "neutral"
            }

            StatusChip {
                compact: true
                label: "Overlay"
                value: root.permissionClient.devModeEnabled ? "Live" : "Disabled"
                tone: root.permissionClient.devModeEnabled ? "accent" : "neutral"
            }

            StatusChip {
                compact: true
                label: "Visual"
                value: root.permissionClient.devVisualFeedbackActive ? "Active" : "Fallback"
                tone: root.permissionClient.devVisualFeedbackActive ? "accent" : "neutral"
            }

            StatusChip {
                compact: true
                label: "Auto refine"
                value: root.permissionClient.devAutoRefine ? "On" : "Off"
                tone: root.permissionClient.devAutoRefine ? "warning" : "neutral"
            }
        }

        Label {
            Layout.fillWidth: true
            visible: root.permissionClient.devModeEnabled
            text: root.permissionClient.devLastChange.length > 0
                ? "Last UI change: " + root.permissionClient.devLastChange
                : "No live UI edits yet."
            color: Theme.textSecondary
            wrapMode: Text.WordWrap
            font.pixelSize: 12
        }

        Label {
            Layout.fillWidth: true
            visible: root.permissionClient.devModeEnabled
            text: root.permissionClient.devOverlayPath.length > 0
                ? "Overlay: " + root.permissionClient.devOverlayPath
                : ""
            color: Theme.textMuted
            wrapMode: Text.WordWrap
            font.pixelSize: 11
        }

        RowLayout {
            Layout.fillWidth: true
            spacing: 8

            Button {
                text: root.permissionClient.devModeEnabled ? "Disable" : "Enable"
                onClicked: root.permissionClient.devModeEnabled
                    ? root.permissionClient.disableDevMode()
                    : root.permissionClient.enableDevMode()
            }

            Button {
                text: root.permissionClient.devAutoRefine ? "Stop" : "Auto refine"
                enabled: root.permissionClient.devModeEnabled
                onClicked: root.permissionClient.setDevAutoRefine(!root.permissionClient.devAutoRefine)
            }

            Button {
                text: "Apply next refinement"
                enabled: root.permissionClient.devModeEnabled && root.permissionClient.devPendingRefinement.length > 0
                onClicked: root.permissionClient.applyNextDevRefinement()
            }

            Button {
                text: "Rollback"
                enabled: root.permissionClient.devModeEnabled
                onClicked: root.permissionClient.rollbackDevMode()
            }

            Button {
                text: "Restart shell"
                enabled: root.permissionClient.devModeEnabled
                onClicked: root.permissionClient.restartShellDev()
            }
        }

        Rectangle {
            Layout.fillWidth: true
            visible: root.permissionClient.devModeEnabled
            radius: Theme.radiusMd
            color: Qt.rgba(1, 1, 1, 0.03)
            border.width: 1
            border.color: Theme.shellStroke
            implicitHeight: 160

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
            implicitHeight: 104

            ColumnLayout {
                anchors.fill: parent
                anchors.margins: Theme.space3
                spacing: 6

                Label {
                    text: root.permissionClient.devVisualSummary.length > 0
                        ? root.permissionClient.devVisualSummary
                        : "After each live patch Velyx can capture a screenshot and review the result."
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
                    text: "Next refinement: " + root.permissionClient.devPendingRefinement
                    color: Theme.textMuted
                    font.pixelSize: 11
                    wrapMode: Text.WordWrap
                }
            }
        }
    }
}
