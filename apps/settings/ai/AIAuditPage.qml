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
            title: "AI Audit"
            subtitle: "История блокировок, подтверждений и исполненных действий AI."
        }

        ListRow { title: "Blocked"; subtitle: "Политика остановила действие" }
        ListRow { title: "Confirmation Required"; subtitle: "Ожидается решение пользователя" }
        ListRow { title: "Executed"; subtitle: "Действие прошло через tool executor" }
        ListRow { title: "Canceled"; subtitle: "Пользователь отменил выполнение" }
    }
}
