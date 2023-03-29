import QtQuick 2.15
import QtQuick.Controls 2.15
import QtGraphicalEffects 1.15

TileItem {
    id: root

    property bool isCurrent: mouse.hovered

    layer.enabled: !menu.visible
    layer.effect: DropShadow {
        transparentBorder: true
        horizontalOffset: 2
        verticalOffset: 2
        color: Constants.shadowColor
        samples: 9
    }

    TileMenu {
        id: menu

        vmName: root.vmName
        vmStatus: root.vmStatus
        anchors.top: parent.top
        width: parent.width
        height: parent.height
        hovered: mouse.hovered
    }

    HoverHandler {
        id: mouse
    }
}

