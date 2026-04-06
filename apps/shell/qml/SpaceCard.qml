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
            return "#2f9e6f"
        if (space.runtime_state === "failed")
            return "#cf5c61"
        return "#d6a44a"
    }

    radius: 18
    color: space.active === "true" ? "#1b2433" : "#141a25"
    border.width: 1
    border.color: space.active === "true" ? "#5b8cff" : Qt.rgba(1, 1, 1, 0.08)
    implicitHeight: 112

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: 16
        spacing: 8

        RowLayout {
            Layout.fillWidth: true

            ColumnLayout {
                Layout.fillWidth: true
                spacing: 2

                Label {
                    text: root.space.display_name || root.space.space_id
                    color: "#f3f6fb"
                    font.pixelSize: 16
                    font.weight: Font.DemiBold
                }

                Label {
                    text: root.space.description && root.space.description.length > 0
                        ? root.space.description
                        : root.space.space_id
                    color: "#8f99ad"
                    font.pixelSize: 12
                    wrapMode: Text.WordWrap
                }
            }

            Rectangle {
                radius: 10
                color: Qt.rgba(1, 1, 1, 0.06)
                border.width: 1
                border.color: Qt.rgba(1, 1, 1, 0.08)
                implicitWidth: 74
                implicitHeight: 28

                Label {
                    anchors.centerIn: parent
                    text: root.space.source || "user"
                    color: "#c7d0df"
                    font.pixelSize: 11
                    font.weight: Font.DemiBold
                }
            }
        }

        RowLayout {
            Layout.fillWidth: true
            spacing: 8

            Rectangle {
                radius: 10
                color: Qt.rgba(1, 1, 1, 0.05)
                implicitHeight: 26
                implicitWidth: 92

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
                        color: "#e8edf7"
                        font.pixelSize: 11
                    }
                }
            }

            Label {
                Layout.fillWidth: true
                text: "Security: " + (root.space.security_mode || "-")
                color: "#8f99ad"
                font.pixelSize: 11
                elide: Text.ElideRight
            }

            Button {
                text: root.space.active === "true" ? "Active" : "Activate"
                enabled: root.space.active !== "true"
                onClicked: root.activateRequested(root.space.space_id)
            }
        }
    }
}
