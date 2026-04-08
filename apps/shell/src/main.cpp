#include "aiclient.h"
#include "inputcontroller.h"
#include "permissionclient.h"
#include "settingsclient.h"

#include <QGuiApplication>
#include <QDateTime>
#include <QDebug>
#include <QDir>
#include <QEvent>
#include <QFile>
#include <QFileInfo>
#include <QJsonDocument>
#include <QJsonObject>
#include <QKeyEvent>
#include <QMouseEvent>
#include <QObject>
#include <QProcessEnvironment>
#include <QQmlApplicationEngine>
#include <QQmlContext>
#include <QQmlError>
#include <QPointF>
#include <QOpenGLContext>
#include <QOpenGLFunctions>
#include <QQuickStyle>
#include <QQuickWindow>
#include <QScreen>
#include <QSGRendererInterface>
#include <QStandardPaths>
#include <QSurfaceFormat>
#include <QTimer>
#include <QTextStream>
#include <QUrl>

namespace {
QString logFilePath()
{
    const QString home = qEnvironmentVariable("HOME");
    const QString stateDir = qEnvironmentVariable("VELYX_STATE_DIR", QDir(home).filePath(".velyx"));
    QDir().mkpath(stateDir);
    return QDir(stateDir).filePath("shell-startup.log");
}

void appendLogLine(const QString &line)
{
    QFile file(logFilePath());
    if (file.open(QIODevice::Append | QIODevice::Text)) {
        QTextStream stream(&file);
        stream << line << Qt::endl;
    }
}

void shellMessageHandler(QtMsgType type, const QMessageLogContext &context, const QString &message)
{
    const QString timestamp = QDateTime::currentDateTimeUtc().toString(Qt::ISODate);
    QString level = "debug";
    switch (type) {
    case QtDebugMsg:
        level = "debug";
        break;
    case QtInfoMsg:
        level = "info";
        break;
    case QtWarningMsg:
        level = "warning";
        break;
    case QtCriticalMsg:
        level = "critical";
        break;
    case QtFatalMsg:
        level = "fatal";
        break;
    }

    QString line = QStringLiteral("%1 shell[%2] %3").arg(timestamp, level, message);
    if (context.file != nullptr && context.line > 0) {
        line += QStringLiteral(" (%1:%2)").arg(QString::fromUtf8(context.file)).arg(context.line);
    }
    fprintf(stderr, "%s\n", line.toLocal8Bit().constData());
    fflush(stderr);
    appendLogLine(line);
    if (type == QtFatalMsg) {
        abort();
    }
}

void logStartupSnapshot(const QQmlApplicationEngine &engine)
{
    const QProcessEnvironment env = QProcessEnvironment::systemEnvironment();
    const QStringList keys = {
        QStringLiteral("HOME"),
        QStringLiteral("USER"),
        QStringLiteral("SHELL"),
        QStringLiteral("XDG_RUNTIME_DIR"),
        QStringLiteral("XDG_SESSION_TYPE"),
        QStringLiteral("XDG_SESSION_CLASS"),
        QStringLiteral("XDG_CURRENT_DESKTOP"),
        QStringLiteral("XDG_SESSION_DESKTOP"),
        QStringLiteral("DESKTOP_SESSION"),
        QStringLiteral("DBUS_SESSION_BUS_ADDRESS"),
        QStringLiteral("VELYX_GRAPHICS_MODE"),
        QStringLiteral("VELYX_GRAPHICS_MODE_REQUESTED"),
        QStringLiteral("VELYX_GRAPHICS_MODE_ACTIVE"),
        QStringLiteral("VELYX_GRAPHICS_FALLBACK_OCCURRED"),
        QStringLiteral("VELYX_GRAPHICS_FALLBACK_REASON"),
        QStringLiteral("QT_QPA_PLATFORM"),
        QStringLiteral("QT_QUICK_BACKEND"),
        QStringLiteral("QSG_RHI_BACKEND"),
        QStringLiteral("QT_XCB_GL_INTEGRATION"),
        QStringLiteral("LIBGL_ALWAYS_SOFTWARE"),
        QStringLiteral("QT_OPENGL"),
        QStringLiteral("QT_PLUGIN_PATH"),
        QStringLiteral("QML2_IMPORT_PATH"),
        QStringLiteral("QML_IMPORT_PATH"),
        QStringLiteral("LD_LIBRARY_PATH"),
        QStringLiteral("VELYX_SHELL_DEBUG")
    };

    for (const QString &key : keys) {
        qInfo().noquote() << "env" << key << "=" << env.value(key, QStringLiteral("<unset>"));
    }
    qInfo().noquote() << "cwd =" << QDir::currentPath();
    qInfo().noquote() << "writableState =" << QStandardPaths::writableLocation(QStandardPaths::AppDataLocation);
    qInfo().noquote() << "engine import paths =" << engine.importPathList().join(QStringLiteral(":"));
    qInfo().noquote() << "tty available =" << QFileInfo::exists(QStringLiteral("/dev/tty1"));
    qInfo().noquote() << "fb available =" << QFileInfo::exists(QStringLiteral("/dev/fb0"));
}

QString graphicsApiName(QSGRendererInterface::GraphicsApi api)
{
    switch (api) {
    case QSGRendererInterface::Unknown:
        return QStringLiteral("unknown");
    case QSGRendererInterface::Software:
        return QStringLiteral("software");
    case QSGRendererInterface::OpenVG:
        return QStringLiteral("openvg");
    case QSGRendererInterface::OpenGL:
        return QStringLiteral("opengl");
    case QSGRendererInterface::Direct3D11:
        return QStringLiteral("d3d11");
    case QSGRendererInterface::Vulkan:
        return QStringLiteral("vulkan");
    case QSGRendererInterface::Metal:
        return QStringLiteral("metal");
    case QSGRendererInterface::Null:
        return QStringLiteral("null");
    case QSGRendererInterface::Direct3D12:
        return QStringLiteral("d3d12");
    }
    return QStringLiteral("unknown");
}

QString renderableTypeName(QSurfaceFormat::RenderableType type)
{
    switch (type) {
    case QSurfaceFormat::DefaultRenderableType:
        return QStringLiteral("default");
    case QSurfaceFormat::OpenGL:
        return QStringLiteral("opengl");
    case QSurfaceFormat::OpenGLES:
        return QStringLiteral("opengles");
    case QSurfaceFormat::OpenVG:
        return QStringLiteral("openvg");
    }
    return QStringLiteral("unknown");
}

void attachGraphicsDiagnostics(QObject *rootObject)
{
    auto *window = qobject_cast<QQuickWindow *>(rootObject);
    if (!window) {
        return;
    }

    QObject::connect(
        window,
        &QQuickWindow::sceneGraphInitialized,
        window,
        [window]() {
            const auto *rendererInterface = window->rendererInterface();
            const auto api = rendererInterface != nullptr
                ? rendererInterface->graphicsApi()
                : QSGRendererInterface::Unknown;
            const QSurfaceFormat format = window->format();
            qInfo().noquote()
                << "graphics backend api=" << graphicsApiName(api)
                << "renderable=" << renderableTypeName(format.renderableType())
                << "version=" << QStringLiteral("%1.%2").arg(format.majorVersion()).arg(format.minorVersion())
                << "profile=" << static_cast<int>(format.profile())
                << "requested_mode=" << qEnvironmentVariable("VELYX_GRAPHICS_MODE_REQUESTED")
                << "active_mode=" << qEnvironmentVariable("VELYX_GRAPHICS_MODE_ACTIVE")
                << "fallback=" << qEnvironmentVariable("VELYX_GRAPHICS_FALLBACK_OCCURRED", QStringLiteral("0"))
                << "reason=" << qEnvironmentVariable("VELYX_GRAPHICS_FALLBACK_REASON");

            if (api == QSGRendererInterface::OpenGL) {
                if (QOpenGLContext *context = QOpenGLContext::currentContext()) {
                    QOpenGLFunctions *functions = context->functions();
                    const auto vendor = reinterpret_cast<const char *>(functions->glGetString(GL_VENDOR));
                    const auto renderer = reinterpret_cast<const char *>(functions->glGetString(GL_RENDERER));
                    const auto version = reinterpret_cast<const char *>(functions->glGetString(GL_VERSION));
                    qInfo().noquote()
                        << "graphics opengl vendor=" << QString::fromUtf8(vendor ? vendor : "")
                        << "renderer=" << QString::fromUtf8(renderer ? renderer : "")
                        << "version=" << QString::fromUtf8(version ? version : "");
                } else {
                    qWarning().noquote() << "graphics backend openGL context unavailable during sceneGraphInitialized";
                }
            }
        },
        Qt::DirectConnection);
}

QVariantMap buildShellRuntimeInfo(const QGuiApplication &app)
{
    QVariantMap info;
    const QProcessEnvironment env = QProcessEnvironment::systemEnvironment();
    const QScreen *screen = app.primaryScreen();
    const qreal screenWidth = screen != nullptr ? screen->availableGeometry().width() : 1366.0;
    const qreal screenHeight = screen != nullptr ? screen->availableGeometry().height() : 768.0;
    const qreal pixelDensity = screen != nullptr ? screen->physicalDotsPerInch() : 96.0;
    const qreal devicePixelRatio = screen != nullptr ? screen->devicePixelRatio() : 1.0;
    const qreal densityScale = qBound(0.9, pixelDensity / 96.0, 1.35);
    const qreal geometryScale = qBound(0.78, qMin(screenWidth / 1366.0, screenHeight / 768.0), 1.2);
    const qreal uiScale = qBound(0.8, densityScale * geometryScale, 1.25);
    const QString environmentMode = env.value(QStringLiteral("VELYX_ENVIRONMENT_MODE"), QStringLiteral("vm"));

    info.insert(QStringLiteral("environmentMode"), environmentMode);
    info.insert(QStringLiteral("isVm"), environmentMode == QStringLiteral("vm"));
    info.insert(QStringLiteral("isBareMetal"), environmentMode == QStringLiteral("bare-metal"));
    info.insert(QStringLiteral("screenWidth"), screenWidth);
    info.insert(QStringLiteral("screenHeight"), screenHeight);
    info.insert(QStringLiteral("pixelDensity"), pixelDensity);
    info.insert(QStringLiteral("devicePixelRatio"), devicePixelRatio);
    info.insert(QStringLiteral("uiScale"), uiScale);
    info.insert(QStringLiteral("cursorSize"), env.value(QStringLiteral("VELYX_CURSOR_SIZE"), QStringLiteral("24")).toInt());
    return info;
}

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

bool inputDebugEnabled()
{
    return qEnvironmentVariableIntValue("VELYX_SHELL_DEBUG") == 1
        || qEnvironmentVariableIntValue("VELYX_INPUT_DEBUG") == 1;
}

class InputDebugFilter final : public QObject
{
public:
    explicit InputDebugFilter(QObject *parent = nullptr)
        : QObject(parent)
    {
    }

protected:
    bool eventFilter(QObject *watched, QEvent *event) override
    {
        if (!inputDebugEnabled()) {
            return QObject::eventFilter(watched, event);
        }

        const char *className = watched != nullptr && watched->metaObject() != nullptr
            ? watched->metaObject()->className()
            : "<null>";

        switch (event->type()) {
        case QEvent::MouseButtonPress:
        case QEvent::MouseButtonRelease:
        case QEvent::MouseMove: {
            auto *mouseEvent = static_cast<QMouseEvent *>(event);
            if (event->type() == QEvent::MouseMove && ++m_moveCount % 25 != 0) {
                break;
            }
            const QString type = event->type() == QEvent::MouseButtonPress
                ? QStringLiteral("mouse-press")
                : (event->type() == QEvent::MouseButtonRelease
                    ? QStringLiteral("mouse-release")
                    : QStringLiteral("mouse-move"));
            qInfo().noquote()
                << "input" << type
                << "target=" << className
                << "pos=" << mouseEvent->position()
                << "global=" << mouseEvent->globalPosition()
                << "button=" << static_cast<int>(mouseEvent->button())
                << "buttons=" << static_cast<int>(mouseEvent->buttons())
                << "focusObject=" << (qApp->focusObject() != nullptr && qApp->focusObject()->metaObject() != nullptr
                        ? qApp->focusObject()->metaObject()->className()
                        : "<none>");
            break;
        }
        case QEvent::KeyPress:
        case QEvent::KeyRelease: {
            auto *keyEvent = static_cast<QKeyEvent *>(event);
            const QString type = event->type() == QEvent::KeyPress
                ? QStringLiteral("key-press")
                : QStringLiteral("key-release");
            qInfo().noquote()
                << "input" << type
                << "target=" << className
                << "key=" << keyEvent->key()
                << "text=" << keyEvent->text();
            break;
        }
        case QEvent::FocusIn:
        case QEvent::FocusOut:
            qInfo().noquote()
                << "input"
                << (event->type() == QEvent::FocusIn ? "focus-in" : "focus-out")
                << "target=" << className;
            break;
        default:
            break;
        }

        return QObject::eventFilter(watched, event);
    }

private:
    int m_moveCount {0};
};
}

int main(int argc, char *argv[])
{
    qInstallMessageHandler(shellMessageHandler);
    QGuiApplication app(argc, argv);
    QGuiApplication::setApplicationName("Velyx Shell");
    QQuickStyle::setStyle("Basic");

    InputDebugFilter inputDebugFilter;
    app.installEventFilter(&inputDebugFilter);

    QQmlApplicationEngine engine;
    QObject::connect(
        &engine,
        &QQmlEngine::warnings,
        &app,
        [](const QList<QQmlError> &warnings) {
            for (const QQmlError &warning : warnings) {
                qWarning().noquote() << "qml warning:" << warning.toString();
            }
        });
    AiClient aiClient;
    PermissionClient permissionClient;
    InputController inputController;
    SettingsClient settingsClient;
    const QVariantMap shellRuntimeInfo = buildShellRuntimeInfo(app);
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
    engine.rootContext()->setContextProperty("shellRuntime", shellRuntimeInfo);
    qInfo().noquote()
        << "runtime mode=" << shellRuntimeInfo.value(QStringLiteral("environmentMode")).toString()
        << "screen=" << QStringLiteral("%1x%2")
            .arg(shellRuntimeInfo.value(QStringLiteral("screenWidth")).toInt())
            .arg(shellRuntimeInfo.value(QStringLiteral("screenHeight")).toInt())
        << "uiScale=" << shellRuntimeInfo.value(QStringLiteral("uiScale")).toDouble()
        << "dpr=" << shellRuntimeInfo.value(QStringLiteral("devicePixelRatio")).toDouble()
        << "dpi=" << shellRuntimeInfo.value(QStringLiteral("pixelDensity")).toDouble();
    QObject::connect(
        &engine,
        &QQmlApplicationEngine::objectCreationFailed,
        &app,
        []() {
            qCritical() << "qml object creation failed";
            QCoreApplication::exit(-1);
        },
        Qt::QueuedConnection);
    logStartupSnapshot(engine);

    QString overlayMainPath;
    if (shouldLoadDevOverlay(&overlayMainPath)) {
        const QFileInfo overlayInfo(overlayMainPath);
        engine.addImportPath(overlayInfo.absolutePath());
        qInfo().noquote() << "loading dev overlay from" << overlayMainPath;
        engine.load(QUrl::fromLocalFile(overlayMainPath));
    } else {
        qInfo() << "loading module Velyx.Apps.Shell/Main";
        engine.loadFromModule("Velyx.Apps.Shell", "Main");
    }
    qInfo() << "root object count after load =" << engine.rootObjects().size();
    if (!engine.rootObjects().isEmpty()) {
        attachGraphicsDiagnostics(engine.rootObjects().constFirst());
    }

    if (qEnvironmentVariableIntValue("VELYX_SHELL_PROBE") == 1) {
        qInfo() << "shell probe mode active";
        QTimer::singleShot(250, &app, []() {
            qInfo() << "shell probe completed";
            QCoreApplication::exit(0);
        });
    }

    return app.exec();
}
