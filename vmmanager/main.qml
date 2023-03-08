import QtQuick 2.15
import QtQuick.Window 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import QtQml.Models 2.15

import ViewEnums 1.0

ApplicationWindow {
    id: root

    width: 900
    height: 600
    visible: true
    flags: Qt.FramelessWindowHint | Qt.Window //|  Qt.CustomizeWindowHint

    header: ToolBar {
        id: toolBar

        height: Constants.toolButtonSize

        Item {//frameless window cannot be moved!
            id: _dragHandler

            anchors.fill: parent
            DragHandler {
                acceptedDevices: PointerDevice.GenericPointer
                grabPermissions:  PointerHandler.CanTakeOverFromItems | PointerHandler.CanTakeOverFromHandlersOfDifferentType | PointerHandler.ApprovesTakeOverByAnything
                onActiveChanged: if (active) root.startSystemMove()
            }
        }

        //! TODO: Change the size of frameless window!

        Label {
            id: vmLabel
            text: "Virtual Machines"
            anchors.verticalCenter: parent.verticalCenter
            anchors.left: parent.left
            anchors.margins: Constants.baseMargin
            color: Constants.textColor0
        }

        Label {
            id: ghafLabel
            text: "GHAF"
            anchors.centerIn: parent
            color: Constants.textColor0
        }

        ToolButton {
            id: menuButton

            anchors.verticalCenter: parent.verticalCenter
            anchors.right: closeButton.left
            height: parent.height
            width: Constants.toolButtonSize

            contentItem: ToolButtonContentItem {
                anchors.fill: parent
                control: parent
                text: qsTr("⋮")
            }

            background: ToolButtonBackground {
                anchors.fill: parent
                control: parent
            }

            onClicked: generalMenu.open()
        }

        ToolButton {
            id: closeButton

            anchors.verticalCenter: parent.verticalCenter
            anchors.right: parent.right
            height: parent.height
            width: Constants.toolButtonSize

            contentItem: ToolButtonContentItem {
                anchors.fill: parent
                control: parent
                image: "/pic/close"
            }

            background: ToolButtonBackground {
                anchors.fill: parent
                control: parent
            }

            onClicked: Qt.quit()
        }

        background: Rectangle {
            anchors.fill: parent
            color: Constants.barColor
        }
    }

    GeneralMenu {
        id: generalMenu

        x: menuButton.x + menuButton.width/2 - getPointerX()
        y: toolBar.y + toolBar.height
        width: 200

        Action {
            text: "Settings"
            onTriggered: rootContext.settingsRequiested()
        }

        Action {
            text: "Update view"
            shortcut: "Ctrl+R"
            onTriggered: rootContext.updateModel()
        }

        Action {
            text: "Show dialog (test)"
            onTriggered: popup.open()
        }
    }

    background: Rectangle {
        anchors.fill: parent
        color: Constants.backgroundColor0
    }

    //main view
    GridView {
        id: grid

        visible: rootContext.currentPage === Views.MainVMView

        anchors.fill: parent
        anchors.margins: Constants.baseMargin

        cellHeight: 160
        cellWidth: 220

        model: DelegateModel {
            model: VMDataModel
            delegate: TileItemDelegate {
                vmName: name
                vmStatus: status

                onIsCurrentChanged: {
                    grid.currentIndex = model.index//grid.model.index
                    console.log("item " + grid.currentIndex)
                }
            }
        }
    }

    //VM details view
    DetailsView {
        anchors.fill: parent
        visible: rootContext.currentPage === Views.DetailsView

        onVisibleChanged: {
            if (visible) {
                //then update properties
                currentItem = grid.currentItem
                //this is to avoid unneccessary updates which will happen because of binding
            }
        }
    }

    //login view
    LoginView {
        anchors.fill: parent
        visible: rootContext.currentPage === Views.LoginView
    }

    //general settings view
    GeneralSettingsView {
        anchors.fill: parent
        visible: rootContext.currentPage === Views.GeneralSettings
    }

    //for popup background
    ScreenBlur {
        id: popupBackground

        anchors.fill: parent
        isBlur: popup.opened
    }

    //dialog popup
    DialogPopup {
        id: popup
        text: "Test dialog popup"
        anchors.centerIn: parent
        //make it versatile!
    }
}
