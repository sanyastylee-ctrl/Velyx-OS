import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Velyx.DesignSystem
import Velyx.UI

Card {
    id: root

    property string understoodIntent: "Unknown"
    property string plannedAction: "Инструмент не выбран"
    property string resultText: "AI пока не выполнял действий"
    property string explanationSource: "system"
    property string suggestedAction: ""
    property string auditRef: "ai_audit.log"
    signal suggestedActionRequested(string action)

    ColumnLayout {
        anchors.fill: parent
        spacing: Theme.space3

        SectionHeader {
            title: "AI Result"
            subtitle: "Объяснимый результат системного AI"
        }

        ListRow { title: "Что понял AI"; subtitle: root.understoodIntent }
        ListRow { title: "Какой tool выбран"; subtitle: root.plannedAction }
        ListRow { title: "Результат"; subtitle: root.resultText }
        ListRow { title: "Источник"; subtitle: root.explanationSource }
        ListRow { title: "Audit"; subtitle: root.auditRef }

        Button {
            visible: root.suggestedAction.length > 0
            text: "Исправить"
            onClicked: root.suggestedActionRequested(root.suggestedAction)
        }
    }
}
