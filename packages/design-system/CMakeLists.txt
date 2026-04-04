qt_add_library(VelyxDesignSystem STATIC)

qt_add_qml_module(VelyxDesignSystem
    URI Velyx.DesignSystem
    VERSION 1.0
    QML_FILES
        qml/Velyx/DesignSystem/Theme.qml
)

target_link_libraries(VelyxDesignSystem PUBLIC Qt6::Core Qt6::Qml Qt6::Quick)
