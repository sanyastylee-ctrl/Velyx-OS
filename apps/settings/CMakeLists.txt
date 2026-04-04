qt_add_executable(velyx-settings
    src/main.cpp
)

qt_add_qml_module(velyx-settings
    URI Velyx.Apps.Settings
    VERSION 1.0
    QML_FILES
        qml/Main.qml
        ai/AIPreferencesPage.qml
        ai/AIPrivacyPage.qml
        ai/AIAuditPage.qml
    DEPENDENCIES
        Velyx.DesignSystem/1.0
        Velyx.UI/1.0
)

target_link_libraries(velyx-settings
    PRIVATE
        Qt6::Core
        Qt6::Gui
        Qt6::Qml
        Qt6::Quick
        Qt6::QuickControls2
        VelyxDesignSystem
        VelyxCoreUi
)
