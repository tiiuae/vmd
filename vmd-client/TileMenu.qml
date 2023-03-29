import QtQuick 2.15
import QtQuick.Layouts 1.15
import QtQuick.Controls 2.15
import QtGraphicalEffects 1.15

Rectangle {
    id: root

    property string vmName: ""
    property string vmStatus: ""

    property bool hovered: false

    QtObject {
        id: internal
        property bool powerOn: vmStatus === "running"
    }

    visible: opacity != 0
    color: Constants.backgroundColor2

    Row {
        anchors.centerIn: parent
        spacing: Constants.smallSpacing

        ActionButton {
            image: "/pic/power"
//            onClicked: rootContext.switchPower(!internal.powerOn, vmName) + rootContext.update() ?
        }
        ActionButton {
            image: "/pic/pause"
            visible: internal.powerOn
//            onClicked: rootContext.switchPower(!internal.powerOn, vmName) + rootContext.update() ?
        }
        ActionButton {
            image: "/pic/settings"
            visible: internal.powerOn
            onClicked: {
                rootContext.detailsRequested()
                console.log(vmName)
            }
        }
    }

    states: [
        State {
            when: !hovered &&!internal.powerOn;
            PropertyChanges {
                target: root
                opacity: 0.8
            }
        },
        State {
            when: hovered && !internal.powerOn;
            PropertyChanges {
                target: root
                opacity: 0.8
            }
        },
        State {
            when: hovered && internal.powerOn;
            PropertyChanges {
                target: root
                opacity: 0.8
            }
        },
        State {
            when: !hovered && internal.powerOn;
            PropertyChanges {
                target: root
                opacity: 0.0
            }
        }
    ]

    transitions: Transition {
            NumberAnimation { property: "opacity"; duration: 500}
        }

}
