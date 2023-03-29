import QtQuick 2.15
import QtQuick.Window 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15

Item {
    id: root

    property Item currentItem: null

    //way to send response!
    //command with id/name

    function startMovement() {
        console.log("vmTile x,y:" + vmTile.x + ", " + vmTile.y)
        infoArea.opacity = 0.0
        vmTile.x = currentItem.x >= Constants.baseMargin ? currentItem.x : Constants.baseMargin
        vmTile.y = currentItem.y >= Constants.baseMargin ? currentItem.y : Constants.baseMargin
        //do not start animation if x,y === Constants.baseMargin ? it looks like a delay
        appearingAnimation.start()
    }

    TileItem {
        id: vmTile

        vmName: currentItem ? currentItem.vmName : ""
        vmStatus: currentItem ? currentItem.vmStatus : ""
    }

    SequentialAnimation {
        id: appearingAnimation

        PropertyAnimation {
            target: vmTile
            properties: "x,y"
            to: Constants.baseMargin
            duration: 400
        }

        PropertyAnimation {
            target: infoArea
            property: "opacity"
            to: 1.0
            duration: 400
        }
    }

    Button {
        id: closeButton

        width:  Constants.secondaryHeaderHeight
        height:  Constants.secondaryHeaderHeight

        anchors.right: parent.right
        anchors.top: parent.top
        anchors.margins: Constants.baseMargin

        contentItem: ToolButtonContentItem {
            anchors.fill: parent
            control: parent
            image: "/pic/close"
            baseColor: Constants.backgroundColor2
            pressColor: Constants.textColor0
        }

        background: ToolButtonBackground {
            anchors.fill: parent
            control: parent
            color: Constants.backgroundColor2
        }

        onClicked: rootContext.mainViewRequiested()
    }

    ColumnLayout {
        id: infoArea

        anchors.top: vmTile.bottom
        anchors.left: parent.left
        anchors.margins: Constants.baseMargin * 4
        spacing: Constants.spacing
        opacity: 0.0

        Label {
            id: infoSectionlabel

            text: "Information"
            font {
                bold: true
                //            pixelSize: Constants.mainFontSize
            }
        }

        Row {
            id: row

            spacing: 50

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

        Separator {}

        Row {
            Layout.preferredHeight: 40
            spacing: Constants.spacing

            SafetyIndicator {
                status: currentItem ? currentItem.vmSafetyStatus : 0
            }

            Label {
                text: {
                    if (!currentItem)
                        return ""
                    if (currentItem.vmSafetyStatus === 0)
                        return "No security threat!"
                    if (currentItem.vmSafetyStatus === 1)
                        return "Medium risk"
                    if (currentItem.vmSafetyStatus === 2)
                        return "High risk!"
                }
            }
        }

        Separator {}

        Row {
            Layout.preferredHeight: 40
            spacing: Constants.spacing

            Button {
                width: 100
                height: 35

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
                height: 35

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

        Separator {}
    }
}
