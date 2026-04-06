#include "inputcontroller.h"

#include <QGuiApplication>

#if defined(Q_OS_LINUX)
#include <QByteArray>
#include <QVector>
#include <X11/Xlib.h>
#include <X11/keysym.h>
#include <xcb/xcb.h>

struct InputController::X11State {
    Display *display {nullptr};
    Window root {0};
    int tabKey {0};
    int qKey {0};
    int rKey {0};
    QVector<int> digitKeys;
};
#endif

InputController::InputController(QObject *parent)
    : QObject(parent)
{
}

InputController::~InputController()
{
#if defined(Q_OS_LINUX)
    releaseX11Grabs();
#endif
}

void InputController::install(QGuiApplication &app)
{
#if defined(Q_OS_LINUX)
    if (installX11Grabs()) {
        app.installNativeEventFilter(this);
        m_mode = "x11-global";
        emit inputStatusChanged(m_mode, "global shortcuts ready");
        return;
    }
#endif

    m_mode = "app-local";
    emit inputStatusChanged(m_mode, "fallback shortcuts active only while shell is focused");
}

QString InputController::mode() const
{
    return m_mode;
}

bool InputController::nativeEventFilter(const QByteArray &eventType, void *message, qintptr *result)
{
    Q_UNUSED(result);
#if defined(Q_OS_LINUX)
    if (m_mode != "x11-global" || eventType != "xcb_generic_event_t" || m_x11 == nullptr) {
        return false;
    }

    auto *event = static_cast<xcb_generic_event_t *>(message);
    if ((event->response_type & ~0x80) != XCB_KEY_PRESS) {
        return false;
    }

    auto *keyEvent = reinterpret_cast<xcb_key_press_event_t *>(event);
    const unsigned int state = keyEvent->state & ~(LockMask | Mod2Mask);

    if (state != Mod1Mask) {
        return false;
    }

    if (keyEvent->detail == m_x11->tabKey) {
        emitShortcut("Alt+Tab");
        emit activateNextRequested();
        return true;
    }

    if (keyEvent->detail == m_x11->qKey) {
        emitShortcut("Alt+Q");
        emit closeActiveRequested();
        return true;
    }

    if (keyEvent->detail == m_x11->rKey) {
        emitShortcut("Alt+R");
        emit restartActiveRequested();
        return true;
    }

    for (int index = 0; index < m_x11->digitKeys.size(); ++index) {
        if (keyEvent->detail == m_x11->digitKeys[index]) {
            emitShortcut(QString("Alt+%1").arg(index + 1));
            emit activateIndexRequested(index);
            return true;
        }
    }
#else
    Q_UNUSED(eventType);
    Q_UNUSED(message);
#endif

    return false;
}

void InputController::emitShortcut(const QString &shortcut, const QString &details)
{
    emit inputStatusChanged(m_mode, details.isEmpty() ? shortcut : QString("%1 | %2").arg(shortcut, details));
}

#if defined(Q_OS_LINUX)
static void grabKeyVariants(Display *display, Window root, int keycode)
{
    const unsigned int modifiers[] = {
        Mod1Mask,
        Mod1Mask | LockMask,
        Mod1Mask | Mod2Mask,
        Mod1Mask | LockMask | Mod2Mask,
    };

    for (unsigned int modifier : modifiers) {
        XGrabKey(display, keycode, modifier, root, True, GrabModeAsync, GrabModeAsync);
    }
}

static void ungrabKeyVariants(Display *display, Window root, int keycode)
{
    const unsigned int modifiers[] = {
        Mod1Mask,
        Mod1Mask | LockMask,
        Mod1Mask | Mod2Mask,
        Mod1Mask | LockMask | Mod2Mask,
    };

    for (unsigned int modifier : modifiers) {
        XUngrabKey(display, keycode, modifier, root);
    }
}

bool InputController::installX11Grabs()
{
    if (m_x11 != nullptr) {
        return true;
    }

    auto *state = new X11State;
    state->display = XOpenDisplay(nullptr);
    if (state->display == nullptr) {
        delete state;
        return false;
    }

    state->root = DefaultRootWindow(state->display);
    state->tabKey = XKeysymToKeycode(state->display, XK_Tab);
    state->qKey = XKeysymToKeycode(state->display, XK_q);
    state->rKey = XKeysymToKeycode(state->display, XK_r);
    for (int digit = 1; digit <= 9; ++digit) {
        state->digitKeys.push_back(XKeysymToKeycode(state->display, XK_0 + digit));
    }

    if (state->tabKey == 0 || state->qKey == 0 || state->rKey == 0) {
        XCloseDisplay(state->display);
        delete state;
        return false;
    }

    grabKeyVariants(state->display, state->root, state->tabKey);
    grabKeyVariants(state->display, state->root, state->qKey);
    grabKeyVariants(state->display, state->root, state->rKey);
    for (int keycode : state->digitKeys) {
        if (keycode != 0) {
            grabKeyVariants(state->display, state->root, keycode);
        }
    }
    XSync(state->display, False);
    m_x11 = state;
    return true;
}

void InputController::releaseX11Grabs()
{
    if (m_x11 == nullptr) {
        return;
    }

    ungrabKeyVariants(m_x11->display, m_x11->root, m_x11->tabKey);
    ungrabKeyVariants(m_x11->display, m_x11->root, m_x11->qKey);
    ungrabKeyVariants(m_x11->display, m_x11->root, m_x11->rKey);
    for (int keycode : m_x11->digitKeys) {
        if (keycode != 0) {
            ungrabKeyVariants(m_x11->display, m_x11->root, keycode);
        }
    }
    XSync(m_x11->display, False);
    XCloseDisplay(m_x11->display);
    delete m_x11;
    m_x11 = nullptr;
}
#endif
