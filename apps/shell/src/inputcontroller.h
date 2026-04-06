#pragma once

#include <QObject>
#include <QAbstractNativeEventFilter>
#include <QString>

class QGuiApplication;

class InputController : public QObject, public QAbstractNativeEventFilter
{
    Q_OBJECT

public:
    explicit InputController(QObject *parent = nullptr);
    ~InputController() override;

    void install(QGuiApplication &app);
    QString mode() const;

    bool nativeEventFilter(const QByteArray &eventType, void *message, qintptr *result) override;

signals:
    void activateNextRequested();
    void closeActiveRequested();
    void restartActiveRequested();
    void activateIndexRequested(int index);
    void inputStatusChanged(const QString &mode, const QString &details);

private:
    void emitShortcut(const QString &shortcut, const QString &details = QString());

#if defined(Q_OS_LINUX)
    bool installX11Grabs();
    void releaseX11Grabs();
#endif

    QString m_mode {"disabled"};

#if defined(Q_OS_LINUX)
    struct X11State;
    X11State *m_x11 {nullptr};
#endif
};
