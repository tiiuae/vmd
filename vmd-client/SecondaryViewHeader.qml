import QtQuick 2.15
import QtQuick.Controls 2.15

Item {
    id: root

    property alias text: label.text

    height: Constants.secondaryHeaderHeight

    Button {
        id: closeButton

        width: parent.height
        height: parent.height

        anchors.left: parent.left
        anchors.verticalCenter: parent.verticalCenter
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

    Label {
        id: label

        anchors.verticalCenter: parent.verticalCenter
        anchors.left: closeButton.right
        anchors.margins: Constants.baseMargin
        height: parent.height

        verticalAlignment: Qt.AlignVCenter
        font.bold: true
    }
}
