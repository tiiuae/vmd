import QtQuick 2.15
import QtQuick.Controls 2.15

Menu {
    id: root

    function getPointerX() {
        return pointerItem.x + pointerItem.width/2;
    }

    topPadding: Constants.baseMargin
    bottomPadding: Constants.baseMargin
    //right and left ones aren't defined

    background: Rectangle {
        implicitWidth: 200
        implicitHeight: root.count + 2*Constants.baseMargin
        color: Constants.backgroundColor1

        Canvas {
            id: pointerItem

            x: root.width * 0.8
            y: parent.y - height

            implicitWidth: 20
            implicitHeight: 10

            onPaint: {
                var ctx = getContext("2d")
                ctx.fillStyle = parent.color
                ctx.moveTo(0, height)
                ctx.lineTo(width/2, 0)
                ctx.lineTo(width, height)
                ctx.closePath()
                ctx.fill()
            }
        }
    }

    delegate: MenuItem {
        id: menuItem

        implicitWidth: 200
        implicitHeight: 40

        contentItem: Label {
            id: textItem

            text: menuItem.text
            font: menuItem.font
            horizontalAlignment: Text.AlignLeft
            verticalAlignment: Text.AlignVCenter
            //            opacity: enabled ? 1.0 : 0.3
            color: menuItem.highlighted ? Constants.textColor0 : Constants.textColor1
        }

        background: Rectangle {
            id: backgroundRect

            implicitWidth: 200
            implicitHeight: 40

            color: menuItem.highlighted ? Constants.backgroundColor2 : Constants.backgroundColor1
        }
    }
}
