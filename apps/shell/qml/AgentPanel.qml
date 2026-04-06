import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Velyx.DesignSystem

Rectangle {
    id: root

    required property var permissionClient
    radius: Theme.radiusLg
    color: Theme.shellSurfaceRaised
    border.width: 1
    border.color: Theme.shellStroke
    implicitHeight: 250

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: Theme.space4
        spacing: Theme.space3

        ColumnLayout {
            Layout.fillWidth: true
            spacing: 4

            Label {
                text: "Ask Velyx"
                color: Theme.textPrimary
                font.family: Theme.fontDisplay
                font.pixelSize: 18
                font.weight: Font.DemiBold
            }

            Label {
                text: root.permissionClient.lastAgentAction.length > 0
                    ? "Last operator action: " + root.permissionClient.lastAgentAction + " • " + root.permissionClient.lastAgentResult
                    : "Use structured commands to move the system, not individual processes."
                color: Theme.textSecondary
                font.pixelSize: 11
                wrapMode: Text.WordWrap
            }
        }

        TextField {
            id: commandField
            Layout.fillWidth: true
            placeholderText: "run dev_start  •  switch safe-web  •  status  •  recovery"
            color: Theme.textPrimary
            placeholderTextColor: Theme.textMuted
            selectByMouse: true
            onAccepted: {
                root.permissionClient.runAgentCommand(text)
                text = ""
            }
        }

        Flow {
            Layout.fillWidth: true
            spacing: 8

            Button {
                text: "Start Development"
                onClicked: root.permissionClient.runAgentCommand("run dev_start")
            }

            Button {
                text: "Safe Browse"
                onClicked: root.permissionClient.runAgentCommand("run safe_browse")
            }

            Button {
                text: "Recovery"
                onClicked: root.permissionClient.runAgentCommand("recovery")
            }

            Button {
                text: "Update"
                onClicked: root.permissionClient.runAgentCommand("update")
            }

            Button {
                text: "Restart Runtime"
                onClicked: root.permissionClient.runAgentCommand("restart-runtime")
            }
        }

        Rectangle {
            Layout.fillWidth: true
            Layout.fillHeight: true
            radius: Theme.radiusMd
            color: Qt.rgba(Theme.accentCool.r, Theme.accentCool.g, Theme.accentCool.b, 0.08)
            border.width: 1
            border.color: Theme.shellStroke

            Label {
                anchors.fill: parent
                anchors.margins: Theme.space4
                text: root.permissionClient.agentSummary.length > 0
                    ? root.permissionClient.agentSummary
                    : "Summary unavailable"
                color: Theme.textSecondary
                font.pixelSize: 11
                wrapMode: Text.WordWrap
                verticalAlignment: Text.AlignTop
            }
        }
    }
}
