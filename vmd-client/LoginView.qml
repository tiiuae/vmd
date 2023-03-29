import QtQuick 2.15
import QtQuick.Controls 2.15

Item {
    id: root

    Button {
        id: closeButton

        width: Constants.secondaryHeaderHeight
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

    Image {
        id: imageItem

        width: 40
        height: 40
        source: "/pic/lock"

        anchors.horizontalCenter: parent.horizontalCenter
        anchors.bottom: passwordInput.top
        anchors.margins: Constants.baseMargin * 3
    }

    TextField {
        id: passwordInput

        width: 240
        height: 30
        echoMode: TextInput.Password
        clip: true

        color: Constants.textColor1

        anchors.centerIn: parent
        anchors.margins: Constants.baseMargin

        background: Rectangle {
            anchors.fill: parent
            color: Constants.backgroundColor3
        }

        onAccepted: {
            rootContext.loginRequest(text);
        }
    }

    Button {
        id: loginButton

        width: passwordInput.width
        height:  passwordInput.height

        anchors.horizontalCenter: passwordInput.horizontalCenter
        anchors.top: passwordInput.bottom
        anchors.margins: Constants.baseMargin

        contentItem: Label  {
            anchors.centerIn: parent
            text: "CONTINUE"
            font.bold: true
            horizontalAlignment: Qt.AlignHCenter
            color: loginButton.pressed ? Constants.textColor1 : Constants.textColor0
        }

        background: Rectangle {
            anchors.fill: parent
            color: loginButton.pressed ?  Constants.backgroundColor1 : Constants.barColor
        }

        onClicked: rootContext.mainViewRequiested()
    }
}
