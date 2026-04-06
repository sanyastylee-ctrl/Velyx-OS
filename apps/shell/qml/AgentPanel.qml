import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

Rectangle {
    id: root

    required property var permissionClient
    radius: 18
    color: "#141c29"
    border.width: 1
    border.color: Qt.rgba(1, 1, 1, 0.08)
    implicitHeight: 220

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: 14
        spacing: 10

        Label {
            text: "Operator"
            color: "#f3f6fb"
            font.pixelSize: 16
            font.weight: Font.DemiBold
        }

        Label {
            text: root.permissionClient.lastAgentAction.length > 0
                ? "Last: " + root.permissionClient.lastAgentAction + " • " + root.permissionClient.lastAgentResult
                : "Use structured commands to operate the system safely."
            color: "#8f99ad"
            font.pixelSize: 11
            wrapMode: Text.WordWrap
        }

        TextField {
            id: commandField
            Layout.fillWidth: true
            placeholderText: "run dev_start • switch safe-web • status • recovery"
            color: "#f3f6fb"
            placeholderTextColor: "#6f7890"
            selectByMouse: true
            onAccepted: {
                root.permissionClient.runAgentCommand(text)
                text = ""
            }
        }

        RowLayout {
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
        }

        RowLayout {
            Layout.fillWidth: true
            spacing: 8

            Button {
                text: "Recovery"
                onClicked: root.permissionClient.runAgentCommand("recovery")
            }

            Button {
                text: "Update System"
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
            radius: 14
            color: "#151d2a"
            border.width: 1
            border.color: Qt.rgba(1, 1, 1, 0.08)

            Label {
                anchors.fill: parent
                anchors.margins: 12
                text: root.permissionClient.agentSummary.length > 0
                    ? root.permissionClient.agentSummary
                    : "Summary unavailable"
                color: "#c7d0df"
                font.pixelSize: 11
                wrapMode: Text.WordWrap
                verticalAlignment: Text.AlignTop
            }
        }
    }
}
