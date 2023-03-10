import QtQuick 2.15
import QtQuick.Window 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15

Item {
    id: root

    property Item currentItem: null

    //way to send response!
    //command with id/name

    SecondaryViewHeader {
        id: header

        anchors.left: parent.left
        anchors.top: parent.top
        text: "Settings / " + (currentItem ? currentItem.vmName : "")
    }

    Row {
        anchors.right: parent.right
        anchors.top: parent.top
        anchors.margins: Constants.baseMargin

        height: header.height
        spacing: Constants.spacing

        Button {
            width: 100
            height: header.height

            contentItem: ToolButtonContentItem {
                anchors.fill: parent
                control: parent
                text: "Shutdown"
                baseColor: Constants.backgroundColor2
                pressColor: Constants.textColor0
            }

            background: ToolButtonBackground {
                anchors.fill: parent
                control: parent
                color: Constants.backgroundColor2
            }

//            onClicked: rootContext.switchPower()
        }

        Button {
            width: 100
            height: header.height

            contentItem: ToolButtonContentItem {
                anchors.fill: parent
                control: parent
                text: "Pause"
                baseColor: Constants.backgroundColor2
                pressColor: Constants.textColor0
            }

            background: ToolButtonBackground {
                anchors.fill: parent
                control: parent
                color: Constants.backgroundColor2
            }
        }
    }

    Row {
        id: row

        anchors.top: header.bottom
        anchors.left: parent.left
        anchors.margins: 5
        spacing: 10


        InfoTileItem {
            text: "Memory used: 150MB"
            value: 0.3
        }
        InfoTileItem {
            text: "CPU usage: 10%"
            value: 0.1
        }
        InfoTileItem {
            text: "Network load: 40%"
            value: 0.4
        }
    }
}
