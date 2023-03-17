import QtQuick 2.15
import QtQuick.Controls 2.15

Rectangle {
    id: root

    property QtObject control: null

    color: Constants.backgroundColor1

    states: [
        State {
            name: "normal"
            when: control.enabled && !control.pressed
            PropertyChanges {
                target: root
                opacity: 0.0
            }
        },
        State {
            name: "pressed"
            when: control.enabled && control.pressed
            PropertyChanges {
                target: root
                opacity: 0.5
            }
        }
    ]
}
