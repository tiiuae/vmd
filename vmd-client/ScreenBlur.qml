import QtQuick 2.15

Rectangle {
    id: root

    property bool isBlur: false

    color: Constants.backgroundColor2
    opacity: isBlur ? 0.8 : 0.0

    Behavior on opacity {
            NumberAnimation { duration: 250 }
        }
}
