import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Velyx.DesignSystem
import Velyx.UI

Rectangle {
    id: root

    required property var permissionClient
    readonly property bool hasPermissionClient: !!root.permissionClient

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
            subtitle: root.hasPermissionClient && root.permissionClient.lastRuleId.length > 0
                ? "Last rule: " + root.permissionClient.lastRuleId + " • " + root.permissionClient.lastRuleResult
                : "Reactive workflows keep the runtime aligned."
        }

        Flow {
            Layout.fillWidth: true
            spacing: 8

            StatusChip {
                compact: true
                label: "Enabled"
                value: root.hasPermissionClient
                    ? root.permissionClient.rules.filter(function(rule) { return rule.enabled === true }).length.toString()
                    : "0"
                tone: "accent"
            }

            StatusChip {
                compact: true
                label: "Last result"
                value: root.hasPermissionClient && root.permissionClient.lastRuleResult.length > 0 ? root.permissionClient.lastRuleResult : "idle"
                tone: !root.hasPermissionClient ? "neutral"
                    : (root.permissionClient.lastRuleResult === "failed" ? "danger"
                        : (root.permissionClient.lastRuleResult === "cooldown" ? "warning" : "success"))
            }
        }

        ListView {
            id: rulesList
            Layout.fillWidth: true
            Layout.fillHeight: true
            clip: true
            spacing: 8
            model: root.hasPermissionClient ? root.permissionClient.rules : []

            property bool viewReady: width > 0 && height > 0

            delegate: Loader {
                required property var modelData
                readonly property bool ruleReady: !!modelData && typeof modelData === "object" && modelData.rule_id !== undefined

                width: rulesList.width
                active: rulesList.viewReady && ruleReady
                asynchronous: false

                sourceComponent: Rectangle {
                    property var ruleData: null

                    width: rulesList.width
                    radius: Theme.radiusMd
                    color: ruleData.rule_id === root.permissionClient.lastRuleId ? Theme.shellSurfaceOverlay : Theme.shellSurface
                    border.width: 1
                    border.color: ruleData.rule_id === root.permissionClient.lastRuleId
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
                                text: ruleData.display_name || ruleData.rule_id
                                color: Theme.textPrimary
                                font.pixelSize: 13
                                font.weight: Font.DemiBold
                                elide: Text.ElideRight
                            }

                            Label {
                                text: (ruleData.trigger_type || "-") + " -> " + (ruleData.action_type || "-")
                                color: Theme.textMuted
                                font.pixelSize: 11
                                elide: Text.ElideRight
                            }
                        }

                        Button {
                            text: "Run"
                            enabled: ruleData.enabled === true
                            onClicked: root.permissionClient.runRule(ruleData.rule_id)
                        }
                    }
                }

                onLoaded: {
                    if (item)
                        item.ruleData = modelData
                }
            }
        }
    }
}
