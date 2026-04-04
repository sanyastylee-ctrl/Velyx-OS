import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Velyx.DesignSystem
import Velyx.UI

Card {
    id: root

    property bool active: false
    property string understoodIntent: "Unknown"
    property string selectedTool: "none"
    property string resultText: "AI пока не выполнял действий"
    property string explanationSource: "system"
    property string suggestedAction: ""
    signal submitCommand(string text)

    ColumnLayout {
        anchors.fill: parent
        spacing: Theme.space4

        SectionHeader {
            title: "Velyx AI"
            subtitle: "Системный AI-слой, работающий только через tools и policy."
        }

        AICommandPalette {
            Layout.fillWidth: true
            onSubmitRequested: function(text) {
                root.submitCommand(text)
            }
        }

        AIResultCard {
            Layout.fillWidth: true
            understoodIntent: root.understoodIntent
            plannedAction: root.selectedTool
            resultText: root.resultText
            explanationSource: root.explanationSource
            suggestedAction: root.suggestedAction
            onSuggestedActionRequested: function(action) {
                root.submitCommand(action)
            }
        }
    }
}
