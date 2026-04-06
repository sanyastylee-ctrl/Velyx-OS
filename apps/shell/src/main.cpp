#include "aiclient.h"
#include "inputcontroller.h"
#include "permissionclient.h"
#include "settingsclient.h"

#include <QGuiApplication>
#include <QDir>
#include <QFile>
#include <QFileInfo>
#include <QJsonDocument>
#include <QJsonObject>
#include <QQmlApplicationEngine>
#include <QQmlContext>
#include <QQuickStyle>
#include <QStandardPaths>
#include <QUrl>

namespace {
bool shouldLoadDevOverlay(QString *overlayMainPath)
{
    const QString home = QStandardPaths::writableLocation(QStandardPaths::HomeLocation);
    const QString stateFilePath = QDir(home).filePath(".velyx/dev_mode.json");
    QFile stateFile(stateFilePath);
    if (!stateFile.open(QIODevice::ReadOnly)) {
        return false;
    }

    const QJsonDocument document = QJsonDocument::fromJson(stateFile.readAll());
    if (!document.isObject()) {
        return false;
    }

    const QJsonObject object = document.object();
    if (!object.value("enabled").toBool(false)) {
        return false;
    }

    const QString overlayRoot = object.value("overlay_root").toString(QDir(home).filePath(".velyx/dev_overlay"));
    const QString overlayMain = QDir(overlayRoot).filePath("apps/shell/qml/Main.qml");
    if (!QFile::exists(overlayMain)) {
        return false;
    }

    if (overlayMainPath != nullptr) {
        *overlayMainPath = overlayMain;
    }
    return true;
}
}

int main(int argc, char *argv[])
{
    QGuiApplication app(argc, argv);
    QGuiApplication::setApplicationName("Velyx Shell");
    QQuickStyle::setStyle("Basic");

    QQmlApplicationEngine engine;
    AiClient aiClient;
    PermissionClient permissionClient;
    InputController inputController;
    SettingsClient settingsClient;
    inputController.install(app);
    QObject::connect(
        &inputController,
        &InputController::activateNextRequested,
        &permissionClient,
        &PermissionClient::activateNextApp);
    QObject::connect(
        &inputController,
        &InputController::closeActiveRequested,
        &permissionClient,
        &PermissionClient::closeActiveApp);
    QObject::connect(
        &inputController,
        &InputController::restartActiveRequested,
        &permissionClient,
        &PermissionClient::restartActiveInstance);
    QObject::connect(
        &inputController,
        &InputController::activateIndexRequested,
        &permissionClient,
        &PermissionClient::activateAppByIndex);
    QObject::connect(
        &inputController,
        &InputController::inputStatusChanged,
        &permissionClient,
        &PermissionClient::setInputControlMode);
    engine.rootContext()->setContextProperty("aiClient", &aiClient);
    engine.rootContext()->setContextProperty("permissionClient", &permissionClient);
    engine.rootContext()->setContextProperty("settingsClient", &settingsClient);
    QObject::connect(
        &engine,
        &QQmlApplicationEngine::objectCreationFailed,
        &app,
        []() { QCoreApplication::exit(-1); },
        Qt::QueuedConnection);

    QString overlayMainPath;
    if (shouldLoadDevOverlay(&overlayMainPath)) {
        const QFileInfo overlayInfo(overlayMainPath);
        engine.addImportPath(overlayInfo.absolutePath());
        engine.load(QUrl::fromLocalFile(overlayMainPath));
    } else {
        engine.loadFromModule("Velyx.Apps.Shell", "Main");
    }

    return app.exec();
}
