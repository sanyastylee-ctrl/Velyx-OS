import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Velyx.DesignSystem

Rectangle {
    id: root

    required property var app
    property bool selected: false
    property bool compact: false
    signal selectRequested(string appId)
    signal launchRequested(string appId)
    signal stopRequested(string appId)
    signal restartRequested(string appId)
    signal activateRequested(string appId)

    function stateTone() {
        const state = app.runtime_state || app.state || "idle"
        if (state === "running")
            return "#2f9e6f"
        if (state === "failed" || state === "broken")
            return "#cf5c61"
        if (state === "starting" || state === "degraded")
            return "#d6a44a"
        return "#6b7280"
    }

    radius: 18
    color: selected ? "#1c2434" : "#141a25"
    border.width: 1
    border.color: selected ? "#5b8cff" : Qt.rgba(1, 1, 1, 0.08)
    implicitHeight: compact ? 116 : 132

    MouseArea {
        anchors.fill: parent
        onClicked: root.selectRequested(root.app.app_id)
    }

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: 16
        spacing: 8

        RowLayout {
            Layout.fillWidth: true

            ColumnLayout {
                Layout.fillWidth: true
                spacing: 3

                Label {
                    text: root.app.display_name || root.app.app_id
                    color: "#f3f6fb"
                    font.pixelSize: compact ? 15 : 16
                    font.weight: Font.DemiBold
                    elide: Text.ElideRight
                }

                Label {
                    text: root.app.app_id
                    color: "#8f99ad"
                    font.pixelSize: 12
                    elide: Text.ElideRight
                }
            }

            Rectangle {
                radius: 10
                color: Qt.rgba(1, 1, 1, 0.05)
                implicitWidth: 70
                implicitHeight: 28

                RowLayout {
                    anchors.fill: parent
                    anchors.margins: 8
                    spacing: 6

                    Rectangle {
                        width: 8
                        height: 8
                        radius: 4
                        color: root.stateTone()
                    }

                    Label {
                        text: root.app.runtime_state || root.app.state || "idle"
                        color: "#e8edf7"
                        font.pixelSize: 11
                    }
                }
            }
        }

        Label {
            Layout.fillWidth: true
            text: "Trust " + (root.app.trust_level || "-")
                + " • " + (root.app.in_active_space === true ? "In space" : "Outside space")
                + (root.app.version ? " • v" + root.app.version : "")
            color: "#a4afc3"
            font.pixelSize: 11
            elide: Text.ElideRight
        }

        Label {
            Layout.fillWidth: true
            text: root.app.window_title && root.app.window_title.length > 0
                ? root.app.window_title
                : ((root.app.window_state && root.app.window_state !== "no_window")
                    ? root.app.window_state
                    : "No real window")
            color: "#7f8aa0"
            font.pixelSize: 11
            elide: Text.ElideRight
            visible: compact || (root.app.window_title || "").length > 0 || (root.app.window_state || "").length > 0
        }

        Item { Layout.fillHeight: true }

        RowLayout {
            Layout.fillWidth: true
            spacing: 8

            Button {
                text: "Launch"
                enabled: (root.app.runtime_state || root.app.state || "idle") !== "running"
                onClicked: root.launchRequested(root.app.app_id)
            }

            Button {
                text: "Stop"
                enabled: (root.app.runtime_state || root.app.state || "idle") === "running"
                onClicked: root.stopRequested(root.app.app_id)
            }

            Button {
                text: "Restart"
                onClicked: root.restartRequested(root.app.app_id)
            }

            Button {
                text: "Active"
                enabled: (root.app.runtime_state || root.app.state || "idle") === "running"
                onClicked: root.activateRequested(root.app.app_id)
            }
        }
    }
}
