import QtQuick 2.15
import QtQuick.Controls 2.15

Item {
    id: root

    property QtObject control: null
    property string text: ""
    property string image: ""

    QtObject {
        id: internal

        property color contentColor: Constants.textColor1
    }

    Label {
        anchors.centerIn: parent
        text: root.text
        color: internal.contentColor
        visible: text != ""
    }

    ColoredImage {
        id: contentImage

        anchors.centerIn: parent
        source: root.image
        color: internal.contentColor
        visible: image != ""
    }

    states: [
        State {
            name: "normal"
            when: control.enabled && !control.pressed
            PropertyChanges {
                target: internal
                contentColor: Constants.textColor1
            }
        },
        State {
            name: "pressed"
            when: control.enabled && control.pressed
            PropertyChanges {
                target: internal
                contentColor: Constants.backgroundColor2
            }
        }
    ]
}
