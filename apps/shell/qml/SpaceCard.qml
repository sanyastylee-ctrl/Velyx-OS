import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Velyx.DesignSystem

Rectangle {
    id: root

    required property var space
    signal activateRequested(string spaceId)

    function stateTone() {
        if (space.runtime_state === "ready")
            return Theme.success
        if (space.runtime_state === "failed")
            return Theme.danger
        return Theme.warning
    }

    function appCount() {
        return space.apps && space.apps.length !== undefined ? space.apps.length : 0
    }

    radius: Theme.radiusLg
    color: space.active === "true" ? Theme.shellSurfaceOverlay : Theme.shellSurfaceRaised
    border.width: 1
    border.color: space.active === "true" ? Qt.rgba(Theme.accentCool.r, Theme.accentCool.g, Theme.accentCool.b, 0.42) : Theme.shellStroke
    implicitHeight: 136

    Behavior on color {
        ColorAnimation { duration: Theme.motionBase }
    }

    Behavior on border.color {
        ColorAnimation { duration: Theme.motionBase }
    }

    Rectangle {
        anchors.left: parent.left
        anchors.top: parent.top
        anchors.bottom: parent.bottom
        anchors.margins: 1
        width: 4
        radius: 3
        color: root.space.active === "true" ? Theme.accentCool : Qt.rgba(1, 1, 1, 0.05)
    }

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: Theme.space4
        spacing: Theme.space3

        RowLayout {
            Layout.fillWidth: true
            spacing: Theme.space3

            ColumnLayout {
                Layout.fillWidth: true
                spacing: 3

                Label {
                    text: root.space.display_name || root.space.space_id
                    color: Theme.textPrimary
                    font.family: Theme.fontDisplay
                    font.pixelSize: 18
                    font.weight: Font.DemiBold
                    elide: Text.ElideRight
                }

                Label {
                    text: root.space.description && root.space.description.length > 0
                        ? root.space.description
                        : root.space.space_id
                    color: Theme.textSecondary
                    font.family: Theme.fontSans
                    font.pixelSize: 12
                    wrapMode: Text.WordWrap
                    maximumLineCount: 2
                    elide: Text.ElideRight
                }
            }

            Rectangle {
                radius: Theme.radiusSm
                color: Qt.rgba(1, 1, 1, 0.05)
                border.width: 1
                border.color: Theme.shellStroke
                implicitWidth: 78
                implicitHeight: 30

                Label {
                    anchors.centerIn: parent
                    text: root.space.source || "user"
                    color: Theme.textSecondary
                    font.family: Theme.fontSans
                    font.pixelSize: 11
                    font.weight: Font.DemiBold
                }
            }
        }

        RowLayout {
            Layout.fillWidth: true
            spacing: 8

            Rectangle {
                radius: Theme.radiusSm
                color: Qt.rgba(root.stateTone().r, root.stateTone().g, root.stateTone().b, 0.14)
                border.width: 1
                border.color: Qt.rgba(root.stateTone().r, root.stateTone().g, root.stateTone().b, 0.22)
                implicitHeight: 28
                implicitWidth: 98

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
                        text: root.space.runtime_state || "unknown"
                        color: Theme.textPrimary
                        font.pixelSize: 11
                        font.weight: Font.DemiBold
                    }
                }
            }

            Rectangle {
                radius: Theme.radiusSm
                color: Qt.rgba(Theme.accentCool.r, Theme.accentCool.g, Theme.accentCool.b, 0.1)
                border.width: 1
                border.color: Qt.rgba(Theme.accentCool.r, Theme.accentCool.g, Theme.accentCool.b, 0.16)
                implicitHeight: 28
                implicitWidth: 86

                Label {
                    anchors.centerIn: parent
                    text: root.appCount() + " apps"
                    color: Theme.textSecondary
                    font.pixelSize: 11
                }
            }

            Label {
                Layout.fillWidth: true
                text: root.space.active === "true"
                    ? "Current context"
                    : ("Mode: " + (root.space.security_mode || "standard"))
                color: root.space.active === "true" ? Theme.accentCoolStrong : Theme.textMuted
                font.pixelSize: 11
                elide: Text.ElideRight
            }

            Button {
                text: root.space.active === "true" ? "Current" : "Enter"
                enabled: root.space.active !== "true"
                onClicked: root.activateRequested(root.space.space_id)
            }
        }
    }
}
