import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Velyx.DesignSystem
import Velyx.UI

Rectangle {
    id: root

    required property var permissionClient
    property bool showAdvanced: false

    radius: 24
    color: "#111722"
    border.width: 1
    border.color: Qt.rgba(1, 1, 1, 0.08)

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: 18
        spacing: 12

        RowLayout {
            Layout.fillWidth: true

            ColumnLayout {
                Layout.fillWidth: true
                spacing: 4

                Label {
                    text: root.permissionClient.selectedAppInfo.display_name || "No app selected"
                    color: "#f3f6fb"
                    font.pixelSize: 18
                    font.weight: Font.DemiBold
                }

                Label {
                    text: root.permissionClient.selectedAppInfo.app_id || "Choose an app or running window"
                    color: "#8f99ad"
                    font.pixelSize: 12
                }
            }

            Button {
                text: root.showAdvanced ? "Hide debug" : "Show debug"
                onClicked: root.showAdvanced = !root.showAdvanced
            }
        }

        ListRow {
            title: "Security"
            subtitle: (root.permissionClient.selectedAppInfo.trust_level || "-")
                + " • " + (root.permissionClient.selectedAppInfo.sandbox_profile || "-")
        }

        ListRow {
            title: "Permissions"
            subtitle: root.permissionClient.selectedAppInfo.requested_permissions || "No permissions requested"
        }

        ListRow {
            title: "Last outcome"
            subtitle: root.permissionClient.lastResult.length > 0
                ? root.permissionClient.lastResult + " • " + root.permissionClient.lastReason
                : "No recent action"
        }

        Rectangle {
            Layout.fillWidth: true
            radius: 16
            color: "#151d2a"
            border.width: 1
            border.color: Qt.rgba(1, 1, 1, 0.08)
            implicitHeight: 110

            ColumnLayout {
                anchors.fill: parent
                anchors.margins: 14
                spacing: 8

                Label {
                    text: "Current message"
                    color: "#8f99ad"
                    font.pixelSize: 12
                }

                Label {
                    Layout.fillWidth: true
                    wrapMode: Text.WordWrap
                    text: root.permissionClient.launchResultMessage.length > 0
                        ? root.permissionClient.launchResultMessage
                        : "System is ready for the next action."
                    color: root.permissionClient.launchStatus === "denied"
                        ? "#ffb4b9"
                        : ((root.permissionClient.launchStatus === "manifest_invalid"
                            || root.permissionClient.launchStatus === "executable_invalid"
                            || root.permissionClient.launchStatus === "profile_invalid"
                            || root.permissionClient.launchStatus === "sandbox_failed"
                            || root.permissionClient.launchStatus === "security_failed"
                            || root.permissionClient.launchStatus === "failed")
                           ? "#ffd08a"
                           : "#e8edf7")
                    font.pixelSize: 13
                }
            }
        }

        Item {
            Layout.fillWidth: true
            Layout.fillHeight: true

            ColumnLayout {
                anchors.fill: parent
                spacing: 8
                visible: root.showAdvanced

                ListRow { title: "Runtime"; subtitle: root.permissionClient.selectedAppInfo.runtime_state || "idle" }
                ListRow { title: "PID"; subtitle: root.permissionClient.selectedAppInfo.runtime_pid || "-" }
                ListRow { title: "Window"; subtitle: root.permissionClient.selectedAppInfo.window_id || "-" }
                ListRow { title: "Window title"; subtitle: root.permissionClient.selectedAppInfo.window_title || "-" }
                ListRow { title: "Geometry"; subtitle: root.permissionClient.selectedAppInfo.window_geometry || "-" }
                ListRow { title: "Executable"; subtitle: root.permissionClient.selectedAppInfo.executable_path || "-" }
                ListRow { title: "Manifest valid"; subtitle: root.permissionClient.selectedAppInfo.manifest_valid || "-" }
                ListRow { title: "Executable valid"; subtitle: root.permissionClient.selectedAppInfo.executable_valid || "-" }
                ListRow { title: "Profile valid"; subtitle: root.permissionClient.selectedAppInfo.profile_valid || "-" }
                ListRow { title: "Retry count"; subtitle: root.permissionClient.selectedAppInfo.session_retry_count || "0" }
                ListRow { title: "Last update"; subtitle: root.permissionClient.lastUpdateResult.length > 0 ? root.permissionClient.lastUpdateResult : "-" }
            }
        }
    }
}
