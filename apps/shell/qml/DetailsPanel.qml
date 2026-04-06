import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Velyx.DesignSystem
import Velyx.UI

Rectangle {
    id: root

    required property var permissionClient
    property bool showAdvanced: false

    radius: Theme.radiusLg
    color: Theme.shellSurface
    border.width: 1
    border.color: Theme.shellStroke

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: Theme.space5
        spacing: Theme.space4

        RowLayout {
            Layout.fillWidth: true
            spacing: Theme.space3

            ColumnLayout {
                Layout.fillWidth: true
                spacing: 4

                Label {
                    text: root.permissionClient.selectedAppInfo.display_name || "No selection"
                    color: Theme.textPrimary
                    font.family: Theme.fontDisplay
                    font.pixelSize: 20
                    font.weight: Font.DemiBold
                }

                Label {
                    text: root.permissionClient.selectedAppInfo.app_id || "Select an app or running window for more detail."
                    color: Theme.textMuted
                    font.pixelSize: 12
                    wrapMode: Text.WordWrap
                }
            }

            Button {
                text: root.showAdvanced ? "Hide advanced" : "Show advanced"
                onClicked: root.showAdvanced = !root.showAdvanced
            }
        }

        Rectangle {
            Layout.fillWidth: true
            radius: Theme.radiusMd
            color: Qt.rgba(
                (root.permissionClient.launchStatus === "denied"
                    ? Theme.danger
                    : ((root.permissionClient.launchStatus === "manifest_invalid"
                        || root.permissionClient.launchStatus === "executable_invalid"
                        || root.permissionClient.launchStatus === "profile_invalid"
                        || root.permissionClient.launchStatus === "sandbox_failed"
                        || root.permissionClient.launchStatus === "security_failed"
                        || root.permissionClient.launchStatus === "failed")
                       ? Theme.warning
                       : Theme.accentCool)).r,
                (root.permissionClient.launchStatus === "denied"
                    ? Theme.danger
                    : ((root.permissionClient.launchStatus === "manifest_invalid"
                        || root.permissionClient.launchStatus === "executable_invalid"
                        || root.permissionClient.launchStatus === "profile_invalid"
                        || root.permissionClient.launchStatus === "sandbox_failed"
                        || root.permissionClient.launchStatus === "security_failed"
                        || root.permissionClient.launchStatus === "failed")
                       ? Theme.warning
                       : Theme.accentCool)).g,
                (root.permissionClient.launchStatus === "denied"
                    ? Theme.danger
                    : ((root.permissionClient.launchStatus === "manifest_invalid"
                        || root.permissionClient.launchStatus === "executable_invalid"
                        || root.permissionClient.launchStatus === "profile_invalid"
                        || root.permissionClient.launchStatus === "sandbox_failed"
                        || root.permissionClient.launchStatus === "security_failed"
                        || root.permissionClient.launchStatus === "failed")
                       ? Theme.warning
                       : Theme.accentCool)).b,
                0.12
            )
            border.width: 1
            border.color: Theme.shellStroke
            implicitHeight: 116

            ColumnLayout {
                anchors.fill: parent
                anchors.margins: Theme.space4
                spacing: 8

                Label {
                    text: "Current message"
                    color: Theme.textMuted
                    font.pixelSize: 11
                }

                Label {
                    Layout.fillWidth: true
                    wrapMode: Text.WordWrap
                    text: root.permissionClient.launchResultMessage.length > 0
                        ? root.permissionClient.launchResultMessage
                        : "System is ready for the next action."
                    color: Theme.textPrimary
                    font.pixelSize: 13
                }
            }
        }

        ListRow {
            title: "Security"
            subtitle: (root.permissionClient.selectedAppInfo.trust_level || "-")
                + "  •  " + (root.permissionClient.selectedAppInfo.sandbox_profile || "-")
        }

        ListRow {
            title: "Permissions"
            subtitle: root.permissionClient.selectedAppInfo.requested_permissions || "No permissions requested"
        }

        ListRow {
            title: "Last outcome"
            subtitle: root.permissionClient.lastResult.length > 0
                ? root.permissionClient.lastResult + "  •  " + root.permissionClient.lastReason
                : "No recent action"
        }

        ListRow {
            title: "AI"
            subtitle: root.permissionClient.aiMode + "  •  "
                + (root.permissionClient.aiModelAvailable
                    ? (root.permissionClient.aiModelName.length > 0 ? root.permissionClient.aiModelName : "model ready")
                    : "model unavailable")
        }

        ListRow {
            title: "AI summary"
            subtitle: root.permissionClient.aiLastSummary.length > 0
                ? root.permissionClient.aiLastSummary
                : "No AI summary yet"
        }

        ListRow {
            title: "Assistant"
            subtitle: root.permissionClient.assistantMode + "  •  " + root.permissionClient.assistantExecutionStatus
        }

        ListRow {
            title: "Assistant response"
            subtitle: root.permissionClient.assistantLastResponse.length > 0
                ? root.permissionClient.assistantLastResponse
                : "No assistant response yet"
        }

        Rectangle {
            Layout.fillWidth: true
            visible: !root.showAdvanced
            radius: Theme.radiusMd
            color: Qt.rgba(1, 1, 1, 0.03)
            border.width: 1
            border.color: Theme.shellStroke
            implicitHeight: 90

            ColumnLayout {
                anchors.fill: parent
                anchors.margins: Theme.space4
                spacing: 4

                Label {
                    text: "Operational details"
                    color: Theme.textPrimary
                    font.pixelSize: 13
                    font.weight: Font.DemiBold
                }

                Label {
                    text: "Advanced runtime fields stay available here, but they do not dominate the main workspace."
                    color: Theme.textMuted
                    font.pixelSize: 11
                    wrapMode: Text.WordWrap
                }
            }
        }

        ScrollView {
            Layout.fillWidth: true
            Layout.fillHeight: true
            visible: root.showAdvanced
            clip: true

            ColumnLayout {
                width: parent.width
                spacing: 8

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
