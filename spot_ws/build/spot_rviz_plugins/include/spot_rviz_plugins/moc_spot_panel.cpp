/****************************************************************************
** Meta object code from reading C++ file 'spot_panel.hpp'
**
** Created by: The Qt Meta Object Compiler version 67 (Qt 5.15.3)
**
** WARNING! All changes made in this file will be lost!
*****************************************************************************/

#include <memory>
#include "../../../../src/spot_ros2/spot_rviz_plugins/include/spot_rviz_plugins/spot_panel.hpp"
#include <QtCore/qbytearray.h>
#include <QtCore/qmetatype.h>
#if !defined(Q_MOC_OUTPUT_REVISION)
#error "The header file 'spot_panel.hpp' doesn't include <QObject>."
#elif Q_MOC_OUTPUT_REVISION != 67
#error "This file was generated using the moc from 5.15.3. It"
#error "cannot be used with the include files from this version of Qt."
#error "(The moc has changed too much.)"
#endif

QT_BEGIN_MOC_NAMESPACE
QT_WARNING_PUSH
QT_WARNING_DISABLE_DEPRECATED
struct qt_meta_stringdata_spot_rviz_plugins__SpotPanel_t {
    QByteArrayData data[17];
    char stringdata0[163];
};
#define QT_MOC_LITERAL(idx, ofs, len) \
    Q_STATIC_BYTE_ARRAY_DATA_HEADER_INITIALIZER_WITH_OFFSET(len, \
    qptrdiff(offsetof(qt_meta_stringdata_spot_rviz_plugins__SpotPanel_t, stringdata0) + ofs \
        - idx * sizeof(QByteArrayData)) \
    )
static const qt_meta_stringdata_spot_rviz_plugins__SpotPanel_t qt_meta_stringdata_spot_rviz_plugins__SpotPanel = {
    {
QT_MOC_LITERAL(0, 0, 28), // "spot_rviz_plugins::SpotPanel"
QT_MOC_LITERAL(1, 29, 3), // "sit"
QT_MOC_LITERAL(2, 33, 0), // ""
QT_MOC_LITERAL(3, 34, 5), // "stand"
QT_MOC_LITERAL(4, 40, 10), // "claimLease"
QT_MOC_LITERAL(5, 51, 12), // "releaseLease"
QT_MOC_LITERAL(6, 64, 7), // "powerOn"
QT_MOC_LITERAL(7, 72, 8), // "powerOff"
QT_MOC_LITERAL(8, 81, 9), // "setMaxVel"
QT_MOC_LITERAL(9, 91, 10), // "gentleStop"
QT_MOC_LITERAL(10, 102, 8), // "hardStop"
QT_MOC_LITERAL(11, 111, 11), // "releaseStop"
QT_MOC_LITERAL(12, 123, 4), // "stop"
QT_MOC_LITERAL(13, 128, 9), // "selfRight"
QT_MOC_LITERAL(14, 138, 4), // "dock"
QT_MOC_LITERAL(15, 143, 6), // "undock"
QT_MOC_LITERAL(16, 150, 12) // "setRobotName"

    },
    "spot_rviz_plugins::SpotPanel\0sit\0\0"
    "stand\0claimLease\0releaseLease\0powerOn\0"
    "powerOff\0setMaxVel\0gentleStop\0hardStop\0"
    "releaseStop\0stop\0selfRight\0dock\0undock\0"
    "setRobotName"
};
#undef QT_MOC_LITERAL

static const uint qt_meta_data_spot_rviz_plugins__SpotPanel[] = {

 // content:
       8,       // revision
       0,       // classname
       0,    0, // classinfo
      15,   14, // methods
       0,    0, // properties
       0,    0, // enums/sets
       0,    0, // constructors
       0,       // flags
       0,       // signalCount

 // slots: name, argc, parameters, tag, flags
       1,    0,   89,    2, 0x08 /* Private */,
       3,    0,   90,    2, 0x08 /* Private */,
       4,    0,   91,    2, 0x08 /* Private */,
       5,    0,   92,    2, 0x08 /* Private */,
       6,    0,   93,    2, 0x08 /* Private */,
       7,    0,   94,    2, 0x08 /* Private */,
       8,    0,   95,    2, 0x08 /* Private */,
       9,    0,   96,    2, 0x08 /* Private */,
      10,    0,   97,    2, 0x08 /* Private */,
      11,    0,   98,    2, 0x08 /* Private */,
      12,    0,   99,    2, 0x08 /* Private */,
      13,    0,  100,    2, 0x08 /* Private */,
      14,    0,  101,    2, 0x08 /* Private */,
      15,    0,  102,    2, 0x08 /* Private */,
      16,    0,  103,    2, 0x08 /* Private */,

 // slots: parameters
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void,

       0        // eod
};

void spot_rviz_plugins::SpotPanel::qt_static_metacall(QObject *_o, QMetaObject::Call _c, int _id, void **_a)
{
    if (_c == QMetaObject::InvokeMetaMethod) {
        auto *_t = static_cast<SpotPanel *>(_o);
        (void)_t;
        switch (_id) {
        case 0: _t->sit(); break;
        case 1: _t->stand(); break;
        case 2: _t->claimLease(); break;
        case 3: _t->releaseLease(); break;
        case 4: _t->powerOn(); break;
        case 5: _t->powerOff(); break;
        case 6: _t->setMaxVel(); break;
        case 7: _t->gentleStop(); break;
        case 8: _t->hardStop(); break;
        case 9: _t->releaseStop(); break;
        case 10: _t->stop(); break;
        case 11: _t->selfRight(); break;
        case 12: _t->dock(); break;
        case 13: _t->undock(); break;
        case 14: _t->setRobotName(); break;
        default: ;
        }
    }
    (void)_a;
}

QT_INIT_METAOBJECT const QMetaObject spot_rviz_plugins::SpotPanel::staticMetaObject = { {
    QMetaObject::SuperData::link<rviz_common::Panel::staticMetaObject>(),
    qt_meta_stringdata_spot_rviz_plugins__SpotPanel.data,
    qt_meta_data_spot_rviz_plugins__SpotPanel,
    qt_static_metacall,
    nullptr,
    nullptr
} };


const QMetaObject *spot_rviz_plugins::SpotPanel::metaObject() const
{
    return QObject::d_ptr->metaObject ? QObject::d_ptr->dynamicMetaObject() : &staticMetaObject;
}

void *spot_rviz_plugins::SpotPanel::qt_metacast(const char *_clname)
{
    if (!_clname) return nullptr;
    if (!strcmp(_clname, qt_meta_stringdata_spot_rviz_plugins__SpotPanel.stringdata0))
        return static_cast<void*>(this);
    return rviz_common::Panel::qt_metacast(_clname);
}

int spot_rviz_plugins::SpotPanel::qt_metacall(QMetaObject::Call _c, int _id, void **_a)
{
    _id = rviz_common::Panel::qt_metacall(_c, _id, _a);
    if (_id < 0)
        return _id;
    if (_c == QMetaObject::InvokeMetaMethod) {
        if (_id < 15)
            qt_static_metacall(this, _c, _id, _a);
        _id -= 15;
    } else if (_c == QMetaObject::RegisterMethodArgumentMetaType) {
        if (_id < 15)
            *reinterpret_cast<int*>(_a[0]) = -1;
        _id -= 15;
    }
    return _id;
}
QT_WARNING_POP
QT_END_MOC_NAMESPACE
