import QtQuick

Rectangle {
    id: root

    required property var permissionClient
    readonly property bool hasPermissionClient: !!root.permissionClient

    anchors.fill: parent
    color: Qt.rgba(0.03, 0.05, 0.08, 0.88)
    visible: root.hasPermissionClient && root.permissionClient.firstBootRequired
    enabled: visible
    z: 100
    focus: visible

    HoverHandler {
        acceptedDevices: PointerDevice.Mouse | PointerDevice.TouchPad
        cursorShape: Qt.ArrowCursor
    }

    Loader {
        anchors.fill: parent
        active: root.visible
        asynchronous: false
        sourceComponent: FirstBoot {
            permissionClient: root.permissionClient
        }
    }
}
