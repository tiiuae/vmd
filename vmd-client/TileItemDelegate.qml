import QtQuick 2.15
import QtQuick.Controls 2.15

Item {
    id: root

    property alias vmName: menu.vmName
    property alias vmStatus: menu.vmStatus
    property bool isCurrent: mouse.hovered
    property int vmSafetyStatus: 0

    width: 200
    height: 150

    Rectangle {
        id: appStatusItem

        width: parent.width
        height: 120

        anchors.top: parent.top

        color: Constants.backgroundColor1

        Image {
            id: safetyIcon

            anchors.top: parent.top
            anchors.left: parent.left
            anchors.margins: Constants.spacing

            source: {//add enum!
                if (vmSafetyStatus === 0) {
                    return "/pic/no_risk"
                }
                if (vmSafetyStatus === 1) {
                    return "/pic/medium_risk"
                }
                if (vmSafetyStatus === 2) {
                    return "/pic/high_risk"
                }
            }
        }

        Image {
            id: appIcon

            anchors.centerIn: parent

            width: mouse.hovered? implicitWidth*1.2 : implicitWidth
            height: mouse.hovered? implicitHeight*1.2 : implicitHeight

            Behavior on height {
                NumberAnimation {duration: 200}
            }

            Behavior on width {
                NumberAnimation {duration: 200}
            }

            source: "/pic/filler"//should be app's icon
        }
    }

    TileMenu {
        id: menu

        anchors.top: parent.top
        width: parent.width
        height: appStatusItem.height
        hovered: mouse.hovered
    }

    Label {
        id: nameLabel

        text: vmName + " " + vmStatus
        anchors.top: appStatusItem.bottom
        anchors.horizontalCenter: appStatusItem.horizontalCenter
        horizontalAlignment: Text.AlignHCenter
        anchors.margins: 5
        width: appStatusItem.width
        height: 25
        font.pixelSize: 18
        color: Constants.textColor1
    }

    HoverHandler {
        id: mouse
    }
}

