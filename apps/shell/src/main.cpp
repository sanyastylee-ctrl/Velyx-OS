#include "aiclient.h"
#include "inputcontroller.h"
#include "permissionclient.h"
#include "settingsclient.h"

#include <QGuiApplication>
#include <QQmlApplicationEngine>
#include <QQmlContext>
#include <QQuickStyle>

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
    engine.loadFromModule("Velyx.Apps.Shell", "Main");

    return app.exec();
}
