import QtQuick 2.15
import QtQuick.Controls 2.15
//import QtGraphicalEffects 1.15 for color overlay

Button {
    id: root

    property string image: ""

    width: 35
    height: width

    background: Rectangle {
        id: backgroundRect

        color: "transparent"
        border.color: "white"
        radius: 5

        Behavior on border.width {
                NumberAnimation { duration: 200 }
            }

        ColoredImage {
            id: imageItem

            anchors.fill: parent
            anchors.margins: Constants.baseMargin
            source: image
            color: Constants.textColor0

            Behavior on anchors.margins {
                    NumberAnimation { duration: 200 }
                }
        }

        states: [
            State {
                when: root.enabled && !root.pressed && !root.hovered
                PropertyChanges {
                    target: backgroundRect
                    border.width: 0
                }
                PropertyChanges {
                    target: imageItem
                    anchors.margins: Constants.baseMargin
                }
            },
            State {
                when: root.enabled && !root.pressed && root.hovered
                PropertyChanges {
                    target: backgroundRect
                    border.width: 0
                }
                PropertyChanges {
                    target: imageItem
                    anchors.margins: 0
                }
            },
            State {
                when: root.enabled && root.pressed;
                PropertyChanges {
                    target: backgroundRect
                    border.width: 2
                }
                PropertyChanges {
                    target: imageItem
                    anchors.margins: Constants.baseMargin
                }
            }
        ]
    }
}
