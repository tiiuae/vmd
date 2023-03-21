import QtQuick 2.15

Image {
    id: root

    property int status: 0

    source: {//add enum!
        if (status === 0) {
            return "/pic/no_risk"
        }
        if (status === 1) {
            return "/pic/medium_risk"
        }
        if (status === 2) {
            return "/pic/high_risk"
        }
    }
}
