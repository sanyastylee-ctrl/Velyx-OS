qt_add_library(VelyxDesignSystem STATIC)

set_source_files_properties(
    qml/Velyx/DesignSystem/Theme.qml
    PROPERTIES QT_QML_SINGLETON_TYPE TRUE
)

qt_add_qml_module(VelyxDesignSystem
    URI Velyx.DesignSystem
    VERSION 1.0
    QML_FILES
        qml/Velyx/DesignSystem/Theme.qml
)

target_link_libraries(VelyxDesignSystem PUBLIC Qt6::Core Qt6::Qml Qt6::Quick)
