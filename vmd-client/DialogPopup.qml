import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15

Popup {
    id: root

    property alias text: dialogText.text

    //    //shadow for whole app window
    //    color: Constants.backgroundColor2
    //    opacity: 0.8

    width: 300
    height: 110

    background: Rectangle {
        id: dialogBackground

        anchors.fill: parent
        color: Constants.backgroundColor0
    }

        GridLayout {
            anchors.centerIn: parent
            anchors.margins: Constants.baseMargin
            rowSpacing: Constants.spacing *3
            columnSpacing: Constants.spacing *5
            columns: 2
            rows: 2

            Text {
                id: dialogText

                horizontalAlignment: Qt.AlignHCenter
                verticalAlignment: Qt.AlignVCenter
                Layout.columnSpan: 2
                Layout.alignment: Qt.AlignCenter
                text: "ololololo"
            }

            Button {
                id: okButton

                implicitWidth: 110
                implicitHeight: 30

                contentItem: ButtonContentItem {
                    anchors.fill: parent
                    control: parent
                    text: "OK"
                }

                background: ButtonBackground {
                    anchors.fill: parent
                    control: parent
                }

                onClicked: {
                    //rootContext.fucntionCall
                    root.close()
                }
            }

            Button {
                id: cancelButton

                implicitWidth: 110
                implicitHeight: 30

                contentItem: ButtonContentItem {
                    anchors.fill: parent
                    control: parent
                    text: "Cancel"
                }

                background: ButtonBackground {
                    anchors.fill: parent
                    control: parent
                }

                 onClicked: {
                     //rootContext.fucntionCall
                     root.close()
                 }
            }
        }
}
