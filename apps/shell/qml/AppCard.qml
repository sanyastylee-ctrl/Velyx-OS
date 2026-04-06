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
            return Theme.success
        if (state === "failed" || state === "broken")
            return Theme.danger
        if (state === "starting" || state === "degraded")
            return Theme.warning
        return Theme.info
    }

    function roleLabel() {
        return root.app.in_active_space === true ? "Serving this space" : "Outside current space"
    }

    function stateLabel() {
        return root.app.runtime_state || root.app.state || "idle"
    }

    radius: Theme.radiusLg
    color: selected ? Theme.shellSurfaceOverlay : Theme.shellSurfaceRaised
    border.width: 1
    border.color: selected ? Qt.rgba(Theme.accentCool.r, Theme.accentCool.g, Theme.accentCool.b, 0.36) : Theme.shellStroke
    implicitHeight: compact ? 138 : 164

    Behavior on color {
        ColorAnimation { duration: Theme.motionBase }
    }

    Behavior on border.color {
        ColorAnimation { duration: Theme.motionBase }
    }

    MouseArea {
        anchors.fill: parent
        onClicked: root.selectRequested(root.app.app_id)
    }

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: Theme.space4
        spacing: compact ? Theme.space2 : Theme.space3

        RowLayout {
            Layout.fillWidth: true
            spacing: Theme.space3

            ColumnLayout {
                Layout.fillWidth: true
                spacing: 3

                Label {
                    text: root.app.display_name || root.app.app_id
                    color: Theme.textPrimary
                    font.family: Theme.fontSans
                    font.pixelSize: compact ? 16 : 17
                    font.weight: Font.DemiBold
                    elide: Text.ElideRight
                }

                Label {
                    text: root.compact
                        ? root.roleLabel()
                        : (root.app.app_id + "  •  " + root.roleLabel())
                    color: root.app.in_active_space === true ? Theme.accentCoolStrong : Theme.textMuted
                    font.pixelSize: 11
                    elide: Text.ElideRight
                }
            }

            Rectangle {
                radius: Theme.radiusSm
                color: Qt.rgba(root.stateTone().r, root.stateTone().g, root.stateTone().b, 0.14)
                border.width: 1
                border.color: Qt.rgba(root.stateTone().r, root.stateTone().g, root.stateTone().b, 0.2)
                implicitWidth: 90
                implicitHeight: 30

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
                        text: root.stateLabel()
                        color: Theme.textPrimary
                        font.pixelSize: 11
                        font.weight: Font.DemiBold
                    }
                }
            }
        }

        Label {
            Layout.fillWidth: true
            text: "Trust " + (root.app.trust_level || "-")
                + "  •  " + (root.app.source || "system")
                + (root.app.version ? "  •  v" + root.app.version : "")
            color: Theme.textSecondary
            font.pixelSize: 11
            elide: Text.ElideRight
        }

        Label {
            Layout.fillWidth: true
            text: root.app.window_title && root.app.window_title.length > 0
                ? root.app.window_title
                : ((root.app.window_state && root.app.window_state !== "no_window")
                    ? root.app.window_state
                    : "No real window yet")
            color: (root.app.window_title || "").length > 0 ? Theme.textSecondary : Theme.textMuted
            font.pixelSize: 11
            elide: Text.ElideRight
            visible: true
        }

        Item { Layout.fillHeight: true }

        RowLayout {
            Layout.fillWidth: true
            spacing: 8

            Button {
                text: (root.app.runtime_state || root.app.state || "idle") === "running" ? "Focus" : "Launch"
                onClicked: {
                    if ((root.app.runtime_state || root.app.state || "idle") === "running")
                        root.activateRequested(root.app.app_id)
                    else
                        root.launchRequested(root.app.app_id)
                }
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
        }
    }
}
