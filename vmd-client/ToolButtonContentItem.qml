import QtQuick 2.15
import QtQuick.Controls 2.15

Item {
    id: root

    property QtObject control: null
    property string text: ""
    property string image: ""
    property color baseColor: Constants.textColor0
    property color pressColor: Constants.backgroundColor2


    Label {
        id: contentText

        anchors.centerIn: parent
        text: root.text
        visible: text != ""
    }

    ColoredImage {
        id: contentImage

        anchors.centerIn: parent
        source: root.image
        visible: image != ""
    }

    states: [
        State {
            name: "normal"
            when: control.enabled && !control.pressed
            PropertyChanges {
                target: contentText
                color:baseColor
            }
            PropertyChanges {
                target: contentImage
                color: baseColor
            }
        },
        State {
            name: "pressed"
            when: control.enabled && control.pressed
            PropertyChanges {
                target: contentText
                color:pressColor
            }
            PropertyChanges {
                target: contentImage
                color:pressColor
            }
        }
    ]
}
