qt_add_library(VelyxCoreUi STATIC
    src/settingsclient.cpp
    src/settingsclient.h
)

qt_add_qml_module(VelyxCoreUi
    URI Velyx.UI
    VERSION 1.0
    QML_FILES
        qml/Velyx/UI/Card.qml
        qml/Velyx/UI/SectionHeader.qml
        qml/Velyx/UI/SearchField.qml
        qml/Velyx/UI/ListRow.qml
    DEPENDENCIES
        Velyx.DesignSystem/1.0
)

target_link_libraries(VelyxCoreUi PUBLIC Qt6::Core Qt6::Qml Qt6::Quick Qt6::QuickControls2 VelyxDesignSystem)
target_link_libraries(VelyxCoreUi PUBLIC Qt6::DBus)
target_include_directories(VelyxCoreUi PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/src)
