import QtQuick 2.15
import QtQuick.Controls 2.15

Item {
    id: root

    SecondaryViewHeader {
        anchors.left: parent.left
        anchors.top: parent.top
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

        //+login button?
    }
}
