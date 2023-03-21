import QtQuick 2.15
import QtQuick.Controls 2.15

ProgressBar {
    id: root

//    padding: 2

    background: Rectangle {
        implicitWidth: 200
        implicitHeight: 15
        color: Constants.backgroundColor3
        radius: 2
    }

    contentItem: Item {
        implicitWidth: 200
        implicitHeight: 15

        Rectangle {
            width: root.visualPosition * parent.width
            height: parent.height
            radius: 2
            color: Constants.iconBackground
        }
    }
}
