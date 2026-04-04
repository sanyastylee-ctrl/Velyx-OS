#include "settingsclient.h"

#include <QDBusConnection>
#include <QDBusInterface>
#include <QDBusReply>

namespace {
constexpr auto kSettingsService = "com.velyx.Settings";
constexpr auto kSettingsPath = "/com/velyx/Settings";
constexpr auto kSettingsInterface = "com.velyx.Settings1";
}

SettingsClient::SettingsClient(QObject *parent)
    : QObject(parent)
{
    refresh();
}

void SettingsClient::refresh()
{
    refreshKeys();
    refreshValues();
}

bool SettingsClient::setValue(const QString &key, const QString &value)
{
    QDBusInterface settings(kSettingsService, kSettingsPath, kSettingsInterface, QDBusConnection::sessionBus());
    if (!settings.isValid()) {
        return false;
    }

    QDBusReply<bool> reply = settings.call("SetValue", key, value);
    if (!reply.isValid() || !reply.value()) {
        return false;
    }

    refresh();
    return true;
}

QString SettingsClient::getValue(const QString &key)
{
    QDBusInterface settings(kSettingsService, kSettingsPath, kSettingsInterface, QDBusConnection::sessionBus());
    if (!settings.isValid()) {
        return QString();
    }

    QDBusReply<QString> reply = settings.call("GetValue", key);
    if (!reply.isValid()) {
        return QString();
    }
    return reply.value();
}

QString SettingsClient::theme() const
{
    return m_theme;
}

QString SettingsClient::bluetoothEnabled() const
{
    return m_bluetoothEnabled;
}

QString SettingsClient::aiEnabled() const
{
    return m_aiEnabled;
}

QString SettingsClient::locale() const
{
    return m_locale;
}

QStringList SettingsClient::keys() const
{
    return m_keys;
}

void SettingsClient::refreshKeys()
{
    QDBusInterface settings(kSettingsService, kSettingsPath, kSettingsInterface, QDBusConnection::sessionBus());
    if (!settings.isValid()) {
        return;
    }

    QDBusReply<QStringList> reply = settings.call("ListKeys");
    if (!reply.isValid()) {
        return;
    }

    if (m_keys != reply.value()) {
        m_keys = reply.value();
        emit keysChanged();
    }
}

void SettingsClient::refreshValues()
{
    const auto theme = getValue("appearance.theme");
    const auto bluetooth = getValue("bluetooth.enabled");
    const auto ai = getValue("ai.enabled");
    const auto locale = getValue("system.locale");

    if (!theme.isEmpty() && m_theme != theme) {
        m_theme = theme;
        emit themeChanged();
    }

    bool changed = false;
    if (!bluetooth.isEmpty() && m_bluetoothEnabled != bluetooth) {
        m_bluetoothEnabled = bluetooth;
        changed = true;
    }
    if (!ai.isEmpty() && m_aiEnabled != ai) {
        m_aiEnabled = ai;
        changed = true;
    }
    if (!locale.isEmpty() && m_locale != locale) {
        m_locale = locale;
        changed = true;
    }

    if (changed) {
        emit valuesChanged();
    }
}
