#include "rootcontext.h"
#include <cstdio>
#include <iostream>
#include <memory>
#include <stdexcept>
#include <string>
#include <array>
#include <QDebug>

/*
    Now there is no clear way to get the VM info, but probably it will be provided by some command line utils.

    For testing purposes the test.txt file is used. This file must be in the same directiry as executable file.
    The template is shown below:
    <VM_name> <status>
*/

RootContext::RootContext()
{
    updateModel();
}

void RootContext::mainViewRequiested()
{
    m_currentView = Views::MainVMView;
    emit currentViewChanged();
}

void RootContext::detailsRequested()
{
    m_currentView = Views::DetailsView;
    emit currentViewChanged();
}

void RootContext::settingsRequiested()
{
    m_currentView = Views::LoginView;
    emit currentViewChanged();
}

void RootContext::loginRequest(const QString &passwd)
{
    //if login ok - display settings
    m_currentView = Views::GeneralSettings;
    emit currentViewChanged();
}

void RootContext::updateModel()
{
    if(mVMDataModel.rowCount(QModelIndex()) > 0)
        mVMDataModel.clear();

    //exec lsvm command
    QString vmInfo = execCommand("cat test.txt");
    vmInfo = vmInfo.trimmed();//simplified()?

    //parse the execution result and add to the model
    QTextStream stream (&vmInfo, QIODevice::ReadOnly);
    while(!stream.atEnd()) {
        QString temp1, temp2;
        stream >> temp1 >> temp2;
        mVMDataModel.addData(Parameter(temp1, temp2));
    }
}

void RootContext::switchPower(bool on, QString name)
{
    qDebug() << on << name;

//    if (on)
//        execCommand("vm-start");
//    else
    //        execCommand("vm-stop");
}

void RootContext::saveSettings()
{

}

QString RootContext::execCommand(const char *cmd)
{
    std::array<char, 128> buffer;
    std::string result;
    std::unique_ptr<FILE, decltype(&pclose)> pipe(popen(cmd, "r"), pclose);
    if (!pipe) {
        throw std::runtime_error("popen() failed!");
    }
    while (fgets(buffer.data(), buffer.size(), pipe.get()) != nullptr) {
        result += buffer.data();
    }

    return QString::fromStdString(result);
}
