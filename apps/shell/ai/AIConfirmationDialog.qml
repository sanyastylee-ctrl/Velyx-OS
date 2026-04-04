import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Velyx.DesignSystem
import Velyx.UI

Popup {
    id: root

    property string summary: "AI хочет выполнить действие"
    property string detailedReason: ""
    property string riskLevel: "sensitive_write"
    property string affectedApp: ""
    property string affectedPermission: ""

    signal confirmAccepted()
    signal confirmRejected()

    modal: true
    focus: true
    dim: true
    closePolicy: Popup.NoAutoClose
    anchors.centerIn: Overlay.overlay
    width: 500
    height: 340
    padding: 0

    background: Card {
        fillColor: "#171d2b"
    }

    contentItem: ColumnLayout {
        anchors.fill: parent
        anchors.margins: Theme.space6
        spacing: Theme.space4

        SectionHeader {
            title: "Подтверждение AI"
            subtitle: "Чувствительные действия требуют явного согласия."
        }

        Label {
            text: root.summary
            wrapMode: Text.WordWrap
            color: Theme.textPrimary
            font.pixelSize: 18
        }

        ListRow { title: "Причина"; subtitle: root.detailedReason }
        ListRow { title: "Приложение"; subtitle: root.affectedApp }
        ListRow { title: "Разрешение"; subtitle: root.affectedPermission }
        ListRow { title: "Уровень риска"; subtitle: root.riskLevel }

        Item { Layout.fillHeight: true }

        RowLayout {
            Layout.fillWidth: true

            Button {
                Layout.fillWidth: true
                text: "Отмена"
                onClicked: root.confirmRejected()
            }

            Button {
                Layout.fillWidth: true
                text: "Подтвердить"
                onClicked: root.confirmAccepted()
            }
        }
    }
}
