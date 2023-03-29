#include "datasource.h"
#include <QJsonObject>
#include <QJsonParseError>
#include <QJsonArray>

//Just to recall the stuff - request to get smth from free API

DataSource::DataSource(QObject *parent)
    : QObject{parent}
{
    networkManager = new QNetworkAccessManager(this);
}

//! According to API spec: 1 - requiest the VM's list, 2 - get info about VM by id

bool DataSource::request()
{
    //create request
    QNetworkRequest request = QNetworkRequest(QUrl("https://api.restful-api.dev/objects"));//list of devices, test

    request.setHeader(QNetworkRequest::ContentTypeHeader, "application/json");

    networkReply = networkManager->get(request);

    connect(networkReply, &QNetworkReply::readyRead, this, &DataSource::readReply);
    connect(networkReply, &QNetworkReply::finished, this, &DataSource::finishReplyReading);

    return true;
}

void DataSource::readReply()
{
    qDebug() << "Data came to slot";
    buffer.append(networkReply->readAll());
}

void DataSource::finishReplyReading()
{
    if(networkReply->error() != QNetworkReply::NoError)
    {
        qDebug() << "Error : " << networkReply->errorString();
    }
    else
    {
        //retrieve data from buffer and parse it
        QMap<int,QString> result;
        QJsonParseError error;
        QJsonDocument doc = QJsonDocument::fromJson(buffer, &error);
        if (error.error != QJsonParseError::NoError)
        {
            qDebug() << error.errorString();
        }
        if (doc.isArray())
        {

            QJsonArray array = doc.array();

            foreach (const QJsonValue & v, array)
            {
                QJsonObject obj = v.toObject();
                int id = obj.value("id").toString().toInt();
                QString name = obj.value("name").toString();
                result.insert(id, name);

                qDebug() << id << name;
            }
        }

        //then empty the buffer
        qDebug() << "Data buffer \n" << buffer;
        buffer.clear();

        emit resultReady(result);
    }

    networkReply->deleteLater();
}
