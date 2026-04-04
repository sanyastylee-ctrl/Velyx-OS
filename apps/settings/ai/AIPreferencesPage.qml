import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Velyx.DesignSystem
import Velyx.UI

Card {
    ColumnLayout {
        anchors.fill: parent
        spacing: Theme.space4

        SectionHeader {
            title: "AI Preferences"
            subtitle: "Глобальные режимы AI-слоя"
        }

        ListRow { title: "AI Layer"; subtitle: "Enabled / Disabled" }
        ListRow { title: "Backend"; subtitle: "Mock / Local / Remote" }
        ListRow { title: "Execution Mode"; subtitle: "Read-only / Ask-before-act / Safe-auto" }
        ListRow { title: "Privacy Mode"; subtitle: "Local-only / Hybrid / Cloud-disabled" }
    }
}
