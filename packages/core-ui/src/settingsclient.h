#pragma once

#include <QObject>
#include <QStringList>

class SettingsClient : public QObject
{
    Q_OBJECT
    Q_PROPERTY(QString theme READ theme NOTIFY themeChanged)
    Q_PROPERTY(QString bluetoothEnabled READ bluetoothEnabled NOTIFY valuesChanged)
    Q_PROPERTY(QString aiEnabled READ aiEnabled NOTIFY valuesChanged)
    Q_PROPERTY(QString locale READ locale NOTIFY valuesChanged)
    Q_PROPERTY(QStringList keys READ keys NOTIFY keysChanged)

public:
    explicit SettingsClient(QObject *parent = nullptr);

    Q_INVOKABLE void refresh();
    Q_INVOKABLE bool setValue(const QString &key, const QString &value);
    Q_INVOKABLE QString getValue(const QString &key);

    QString theme() const;
    QString bluetoothEnabled() const;
    QString aiEnabled() const;
    QString locale() const;
    QStringList keys() const;

signals:
    void themeChanged();
    void valuesChanged();
    void keysChanged();

private:
    void refreshKeys();
    void refreshValues();

    QString m_theme = "dark";
    QString m_bluetoothEnabled = "true";
    QString m_aiEnabled = "true";
    QString m_locale = "ru_RU";
    QStringList m_keys;
};
