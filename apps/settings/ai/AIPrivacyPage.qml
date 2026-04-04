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
            title: "AI Privacy"
            subtitle: "Какие данные и контекст доступны AI."
        }

        ListRow { title: "Active Window"; subtitle: "Разрешать AI видеть активное окно" }
        ListRow { title: "File Metadata"; subtitle: "Метаданные файлов, но не содержимое" }
        ListRow { title: "File Contents"; subtitle: "Только с отдельной политикой" }
        ListRow { title: "Notifications"; subtitle: "Отключено по умолчанию" }
        ListRow { title: "History"; subtitle: "Хранение истории AI-сессий" }
    }
}
