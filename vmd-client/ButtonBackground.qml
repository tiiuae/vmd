import QtQuick 2.15
import QtQuick.Controls 2.15

Rectangle {
    id: root

    property QtObject control: null

    states: [
        State {
            name: "normal"
            when: control.enabled && !control.pressed
            PropertyChanges {
                target: root
                color: Constants.backgroundColor3
            }
        },
        State {
            name: "pressed"
            when: control.enabled && control.pressed
            PropertyChanges {
                target: root
                color: Constants.backgroundColor2
            }
        }
    ]
}
