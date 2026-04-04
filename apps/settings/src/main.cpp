#include "settingsclient.h"

#include <QGuiApplication>
#include <QQmlApplicationEngine>
#include <QQmlContext>
#include <QQuickStyle>

int main(int argc, char *argv[])
{
    QGuiApplication app(argc, argv);
    QGuiApplication::setApplicationName("Velyx Settings");
    QQuickStyle::setStyle("Basic");

    QQmlApplicationEngine engine;
    SettingsClient settingsClient;
    engine.rootContext()->setContextProperty("settingsClient", &settingsClient);
    QObject::connect(
        &engine,
        &QQmlApplicationEngine::objectCreationFailed,
        &app,
        []() { QCoreApplication::exit(-1); },
        Qt::QueuedConnection);
    engine.loadFromModule("Velyx.Apps.Settings", "Main");

    return app.exec();
}
