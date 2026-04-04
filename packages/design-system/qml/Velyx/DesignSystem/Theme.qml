pragma Singleton
import QtQuick

QtObject {
    readonly property color windowBg: "#0f1115"
    readonly property color surface0: "#151922"
    readonly property color surface1: "#1b2130"
    readonly property color surface2: "#222a3b"
    readonly property color surface3: "#2d3950"
    readonly property color accent: "#7dd3b0"
    readonly property color accentStrong: "#a4efd0"
    readonly property color textPrimary: "#f5f7fb"
    readonly property color textSecondary: "#bcc6d8"
    readonly property color textMuted: "#8190ab"
    readonly property color strokeSubtle: "#37445b"
    readonly property color focusRing: "#d6ffef"
    readonly property color warning: "#ffd27d"
    readonly property color danger: "#ff878f"

    readonly property int radiusSm: 10
    readonly property int radiusMd: 16
    readonly property int radiusLg: 24

    readonly property int space1: 4
    readonly property int space2: 8
    readonly property int space3: 12
    readonly property int space4: 16
    readonly property int space5: 20
    readonly property int space6: 24
    readonly property int space8: 32

    readonly property int iconSm: 16
    readonly property int iconMd: 20
    readonly property int iconLg: 24

    readonly property int motionFast: 120
    readonly property int motionBase: 180
    readonly property int motionSlow: 260

    readonly property string fontSans: "Segoe UI"
    readonly property string fontDisplay: "Bahnschrift"
}
