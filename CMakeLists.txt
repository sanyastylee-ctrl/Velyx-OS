cmake_minimum_required(VERSION 3.27)

project(VelyxOS VERSION 0.1.0 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)
set(CMAKE_AUTOUIC ON)

find_package(Qt6 6.8 REQUIRED COMPONENTS Core Gui Qml Quick QuickControls2 DBus)

qt_standard_project_setup(REQUIRES 6.8)

add_subdirectory(packages/design-system)
add_subdirectory(packages/core-ui)
add_subdirectory(apps/shell)
add_subdirectory(apps/settings)
add_subdirectory(apps/file-manager)

enable_testing()
