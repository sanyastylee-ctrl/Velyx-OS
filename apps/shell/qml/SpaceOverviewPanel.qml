import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Velyx.DesignSystem

Rectangle {
    id: root

    required property var permissionClient
    property int inSpaceCount: 0
    property int runningInSpaceCount: 0
    property int outsideCount: 0

    radius: 24
    color: "#111722"
    border.width: 1
    border.color: Qt.rgba(1, 1, 1, 0.08)

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: 20
        spacing: 14

        RowLayout {
            Layout.fillWidth: true
            spacing: 12

            ColumnLayout {
                Layout.fillWidth: true
                spacing: 4

                Label {
                    text: root.permissionClient.activeSpaceName.length > 0
                        ? root.permissionClient.activeSpaceName
                        : "No active space"
                    color: "#f3f6fb"
                    font.pixelSize: 28
                    font.weight: Font.DemiBold
                }

                Label {
                    text: root.permissionClient.activeSpaceId.length > 0
                        ? root.permissionClient.activeSpaceId
                        : "Select a working context"
                    color: "#8f99ad"
                    font.pixelSize: 13
                }
            }

            StatusChip {
                compact: true
                label: "Space state"
                value: root.permissionClient.activeSpaceState.length > 0 ? root.permissionClient.activeSpaceState : "unknown"
                tone: root.permissionClient.activeSpaceState === "ready" ? "success"
                    : (root.permissionClient.activeSpaceState === "failed" ? "danger" : "warning")
            }
        }

        Label {
            Layout.fillWidth: true
            wrapMode: Text.WordWrap
            text: root.permissionClient.recoveryNeeded
                ? "Recovery needed. Runtime is available, but the system expects attention before continuing normal work."
                : "Context drives the session. Apps in this space are primary, running apps outside it remain visible but secondary."
            color: root.permissionClient.recoveryNeeded ? "#ffb4b9" : "#b4bfd3"
            font.pixelSize: 13
        }

        Flow {
            Layout.fillWidth: true
            spacing: 10

            StatusChip {
                compact: true
                label: "Security"
                value: root.permissionClient.activeSpaceSecurityMode.length > 0 ? root.permissionClient.activeSpaceSecurityMode : "-"
                tone: "accent"
            }

            StatusChip {
                compact: true
                label: "Preferred app"
                value: root.permissionClient.activeSpacePreferredApp.length > 0 ? root.permissionClient.activeSpacePreferredApp : "none"
                tone: "accent"
            }

            StatusChip {
                compact: true
                label: "Apps in space"
                value: root.inSpaceCount.toString()
                tone: "success"
            }

            StatusChip {
                compact: true
                label: "Running here"
                value: root.runningInSpaceCount.toString()
                tone: root.runningInSpaceCount > 0 ? "success" : "warning"
            }

            StatusChip {
                compact: true
                label: "Outside current"
                value: root.outsideCount.toString()
                tone: root.outsideCount > 0 ? "warning" : "neutral"
            }
        }
    }
}
