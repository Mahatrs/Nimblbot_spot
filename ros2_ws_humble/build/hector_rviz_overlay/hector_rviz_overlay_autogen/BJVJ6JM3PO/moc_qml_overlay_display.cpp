/****************************************************************************
** Meta object code from reading C++ file 'qml_overlay_display.hpp'
**
** Created by: The Qt Meta Object Compiler version 67 (Qt 5.15.3)
**
** WARNING! All changes made in this file will be lost!
*****************************************************************************/

#include <memory>
#include "../../../../src/rviz-overlay-ros2/hector_rviz_overlay/hector_rviz_overlay/include/hector_rviz_overlay/displays/qml_overlay_display.hpp"
#include <QtCore/qbytearray.h>
#include <QtCore/qmetatype.h>
#if !defined(Q_MOC_OUTPUT_REVISION)
#error "The header file 'qml_overlay_display.hpp' doesn't include <QObject>."
#elif Q_MOC_OUTPUT_REVISION != 67
#error "This file was generated using the moc from 5.15.3. It"
#error "cannot be used with the include files from this version of Qt."
#error "(The moc has changed too much.)"
#endif

QT_BEGIN_MOC_NAMESPACE
QT_WARNING_PUSH
QT_WARNING_DISABLE_DEPRECATED
struct qt_meta_stringdata_hector_rviz_overlay__QmlOverlayDisplay_t {
    QByteArrayData data[7];
    char stringdata0[133];
};
#define QT_MOC_LITERAL(idx, ofs, len) \
    Q_STATIC_BYTE_ARRAY_DATA_HEADER_INITIALIZER_WITH_OFFSET(len, \
    qptrdiff(offsetof(qt_meta_stringdata_hector_rviz_overlay__QmlOverlayDisplay_t, stringdata0) + ofs \
        - idx * sizeof(QByteArrayData)) \
    )
static const qt_meta_stringdata_hector_rviz_overlay__QmlOverlayDisplay_t qt_meta_stringdata_hector_rviz_overlay__QmlOverlayDisplay = {
    {
QT_MOC_LITERAL(0, 0, 38), // "hector_rviz_overlay::QmlOverl..."
QT_MOC_LITERAL(1, 39, 22), // "onOverlayStatusChanged"
QT_MOC_LITERAL(2, 62, 0), // ""
QT_MOC_LITERAL(3, 63, 18), // "QmlOverlay::Status"
QT_MOC_LITERAL(4, 82, 6), // "status"
QT_MOC_LITERAL(5, 89, 23), // "onOverlayContextCreated"
QT_MOC_LITERAL(6, 113, 19) // "onLiveReloadChanged"

    },
    "hector_rviz_overlay::QmlOverlayDisplay\0"
    "onOverlayStatusChanged\0\0QmlOverlay::Status\0"
    "status\0onOverlayContextCreated\0"
    "onLiveReloadChanged"
};
#undef QT_MOC_LITERAL

static const uint qt_meta_data_hector_rviz_overlay__QmlOverlayDisplay[] = {

 // content:
       8,       // revision
       0,       // classname
       0,    0, // classinfo
       3,   14, // methods
       0,    0, // properties
       0,    0, // enums/sets
       0,    0, // constructors
       0,       // flags
       0,       // signalCount

 // slots: name, argc, parameters, tag, flags
       1,    1,   29,    2, 0x09 /* Protected */,
       5,    0,   32,    2, 0x09 /* Protected */,
       6,    0,   33,    2, 0x09 /* Protected */,

 // slots: parameters
    QMetaType::Void, 0x80000000 | 3,    4,
    QMetaType::Void,
    QMetaType::Void,

       0        // eod
};

void hector_rviz_overlay::QmlOverlayDisplay::qt_static_metacall(QObject *_o, QMetaObject::Call _c, int _id, void **_a)
{
    if (_c == QMetaObject::InvokeMetaMethod) {
        auto *_t = static_cast<QmlOverlayDisplay *>(_o);
        (void)_t;
        switch (_id) {
        case 0: _t->onOverlayStatusChanged((*reinterpret_cast< QmlOverlay::Status(*)>(_a[1]))); break;
        case 1: _t->onOverlayContextCreated(); break;
        case 2: _t->onLiveReloadChanged(); break;
        default: ;
        }
    }
}

QT_INIT_METAOBJECT const QMetaObject hector_rviz_overlay::QmlOverlayDisplay::staticMetaObject = { {
    QMetaObject::SuperData::link<OverlayDisplay::staticMetaObject>(),
    qt_meta_stringdata_hector_rviz_overlay__QmlOverlayDisplay.data,
    qt_meta_data_hector_rviz_overlay__QmlOverlayDisplay,
    qt_static_metacall,
    nullptr,
    nullptr
} };


const QMetaObject *hector_rviz_overlay::QmlOverlayDisplay::metaObject() const
{
    return QObject::d_ptr->metaObject ? QObject::d_ptr->dynamicMetaObject() : &staticMetaObject;
}

void *hector_rviz_overlay::QmlOverlayDisplay::qt_metacast(const char *_clname)
{
    if (!_clname) return nullptr;
    if (!strcmp(_clname, qt_meta_stringdata_hector_rviz_overlay__QmlOverlayDisplay.stringdata0))
        return static_cast<void*>(this);
    return OverlayDisplay::qt_metacast(_clname);
}

int hector_rviz_overlay::QmlOverlayDisplay::qt_metacall(QMetaObject::Call _c, int _id, void **_a)
{
    _id = OverlayDisplay::qt_metacall(_c, _id, _a);
    if (_id < 0)
        return _id;
    if (_c == QMetaObject::InvokeMetaMethod) {
        if (_id < 3)
            qt_static_metacall(this, _c, _id, _a);
        _id -= 3;
    } else if (_c == QMetaObject::RegisterMethodArgumentMetaType) {
        if (_id < 3)
            *reinterpret_cast<int*>(_a[0]) = -1;
        _id -= 3;
    }
    return _id;
}
QT_WARNING_POP
QT_END_MOC_NAMESPACE
