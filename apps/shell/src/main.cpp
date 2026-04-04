#include "aiclient.h"
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
    SettingsClient settingsClient;
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
