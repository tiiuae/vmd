pragma Singleton
import QtQuick 2.15

QtObject {
    //sizes
    readonly property int minimumWindowWidth: 900
    readonly property int minimumWindowHeight: 600
    readonly property int toolButtonSize: 50
    readonly property int secondaryHeaderHeight: 35

    //margins
    readonly property int spacing: 5
    readonly property int smallSpacing: 3
    readonly property int baseMargin: 5

    //corner radiuses
    readonly property int backgroundRadius: 0

    //font sizes
    readonly property int delegateFontSize: 20
    readonly property int mainFontSize: 22
    readonly property int largeFontSize: 30

    //colors
    readonly property color baseColor0: "#95ADB6"
    readonly property color baseColor1: "#0C2D48"

    readonly property color barColor: "#2E3B4C"

    readonly property color backgroundColor0: "#FFFFFF"
    readonly property color backgroundColor1: "#EAF2F7"
    readonly property color backgroundColor2: "#2E3B4C"
    readonly property color iconBackground: "#597393"
    readonly property color backgroundColor3:"#DDEAF3"

    readonly property color shadowColor: Qt.rgba(0, 0, 0, 0.04)

    readonly property color indicatorOn: "#35CE8D"
    readonly property color indicatorOff: "#AE4634"
    readonly property color graphColor: "#29b6f6"

    readonly property color textColor0: "#ffffff"
    readonly property color textColor1: "#000000"
}
