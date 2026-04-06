import QtQuick

Rectangle {
    id: root

    required property var permissionClient

    anchors.fill: parent
    color: Qt.rgba(0.03, 0.05, 0.08, 0.88)
    visible: root.permissionClient.firstBootRequired
    z: 100

    FirstBoot {
        anchors.fill: parent
        permissionClient: root.permissionClient
    }
}
