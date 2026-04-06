qt_add_executable(velyx-shell
    src/main.cpp
    src/aiclient.cpp
    src/aiclient.h
    src/inputcontroller.cpp
    src/inputcontroller.h
    src/permissionclient.cpp
    src/permissionclient.h
)

qt_add_qml_module(velyx-shell
    URI Velyx.Apps.Shell
    VERSION 1.0
    QML_FILES
        qml/Main.qml
        qml/PermissionDialog.qml
        qml/StatusChip.qml
        qml/AssistantPanel.qml
        qml/AiSuggestionPanel.qml
        qml/FirstBootOverlay.qml
        qml/DiagnosticsPanel.qml
        qml/DevModePanel.qml
        qml/SpaceCard.qml
        qml/IntentCard.qml
        qml/AgentPanel.qml
        qml/AutomationPanel.qml
        qml/ControlCenterPanel.qml
        qml/AppCard.qml
        qml/SystemStatusPanel.qml
        qml/SpaceOverviewPanel.qml
        qml/OpenAppsPanel.qml
        qml/DetailsPanel.qml
        qml/SuggestedActionsPanel.qml
        ai/AIOverlay.qml
        ai/AICommandPalette.qml
        ai/AIResultCard.qml
        ai/AIConfirmationDialog.qml
    DEPENDENCIES
        Velyx.DesignSystem/1.0
        Velyx.UI/1.0
)

target_link_libraries(velyx-shell
    PRIVATE
        Qt6::Core
        Qt6::Gui
        Qt6::Qml
        Qt6::Quick
        Qt6::QuickControls2
        Qt6::DBus
        VelyxDesignSystem
        VelyxCoreUi
)

if(UNIX AND NOT APPLE)
    find_package(X11 REQUIRED)
    target_link_libraries(velyx-shell PRIVATE X11::X11)
endif()
