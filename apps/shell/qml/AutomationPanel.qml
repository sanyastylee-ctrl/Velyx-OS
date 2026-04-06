import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Velyx.DesignSystem
import Velyx.UI

Rectangle {
    id: root

    required property var permissionClient

    radius: Theme.radiusLg
    color: Theme.shellSurfaceRaised
    border.width: 1
    border.color: Theme.shellStroke
    implicitHeight: 240

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: Theme.space4
        spacing: Theme.space3

        SectionHeader {
            Layout.fillWidth: true
            title: "Automation"
            subtitle: root.permissionClient.lastRuleId.length > 0
                ? "Last rule: " + root.permissionClient.lastRuleId + " • " + root.permissionClient.lastRuleResult
                : "Reactive workflows keep the runtime aligned."
        }

        Flow {
            Layout.fillWidth: true
            spacing: 8

            StatusChip {
                compact: true
                label: "Enabled"
                value: root.permissionClient.rules.filter(function(rule) { return rule.enabled === true }).length.toString()
                tone: "accent"
            }

            StatusChip {
                compact: true
                label: "Last result"
                value: root.permissionClient.lastRuleResult.length > 0 ? root.permissionClient.lastRuleResult : "idle"
                tone: root.permissionClient.lastRuleResult === "failed" ? "danger"
                    : (root.permissionClient.lastRuleResult === "cooldown" ? "warning" : "success")
            }
        }

        ListView {
            Layout.fillWidth: true
            Layout.fillHeight: true
            clip: true
            spacing: 8
            model: root.permissionClient.rules

            delegate: Rectangle {
                width: ListView.view.width
                radius: Theme.radiusMd
                color: modelData.rule_id === root.permissionClient.lastRuleId ? Theme.shellSurfaceOverlay : Theme.shellSurface
                border.width: 1
                border.color: modelData.rule_id === root.permissionClient.lastRuleId
                    ? Qt.rgba(Theme.accentCool.r, Theme.accentCool.g, Theme.accentCool.b, 0.28)
                    : Theme.shellStroke
                implicitHeight: 82

                RowLayout {
                    anchors.fill: parent
                    anchors.margins: Theme.space3
                    spacing: Theme.space3

                    ColumnLayout {
                        Layout.fillWidth: true
                        spacing: 2

                        Label {
                            text: modelData.display_name || modelData.rule_id
                            color: Theme.textPrimary
                            font.pixelSize: 13
                            font.weight: Font.DemiBold
                            elide: Text.ElideRight
                        }

                        Label {
                            text: (modelData.trigger_type || "-") + " -> " + (modelData.action_type || "-")
                            color: Theme.textMuted
                            font.pixelSize: 11
                            elide: Text.ElideRight
                        }
                    }

                    Button {
                        text: "Run"
                        enabled: modelData.enabled === true
                        onClicked: root.permissionClient.runRule(modelData.rule_id)
                    }
                }
            }
        }
    }
}
