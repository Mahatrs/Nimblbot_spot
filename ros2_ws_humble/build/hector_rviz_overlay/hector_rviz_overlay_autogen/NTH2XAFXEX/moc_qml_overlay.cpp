/****************************************************************************
** Meta object code from reading C++ file 'qml_overlay.hpp'
**
** Created by: The Qt Meta Object Compiler version 67 (Qt 5.15.3)
**
** WARNING! All changes made in this file will be lost!
*****************************************************************************/

#include <memory>
#include "../../../../src/rviz-overlay-ros2/hector_rviz_overlay/hector_rviz_overlay/include/hector_rviz_overlay/ui/qml_overlay.hpp"
#include <QtCore/qbytearray.h>
#include <QtCore/qmetatype.h>
#if !defined(Q_MOC_OUTPUT_REVISION)
#error "The header file 'qml_overlay.hpp' doesn't include <QObject>."
#elif Q_MOC_OUTPUT_REVISION != 67
#error "This file was generated using the moc from 5.15.3. It"
#error "cannot be used with the include files from this version of Qt."
#error "(The moc has changed too much.)"
#endif

QT_BEGIN_MOC_NAMESPACE
QT_WARNING_PUSH
QT_WARNING_DISABLE_DEPRECATED
struct qt_meta_stringdata_hector_rviz_overlay__QmlOverlay_t {
    QByteArrayData data[13];
    char stringdata0[197];
};
#define QT_MOC_LITERAL(idx, ofs, len) \
    Q_STATIC_BYTE_ARRAY_DATA_HEADER_INITIALIZER_WITH_OFFSET(len, \
    qptrdiff(offsetof(qt_meta_stringdata_hector_rviz_overlay__QmlOverlay_t, stringdata0) + ofs \
        - idx * sizeof(QByteArrayData)) \
    )
static const qt_meta_stringdata_hector_rviz_overlay__QmlOverlay_t qt_meta_stringdata_hector_rviz_overlay__QmlOverlay = {
    {
QT_MOC_LITERAL(0, 0, 31), // "hector_rviz_overlay::QmlOverlay"
QT_MOC_LITERAL(1, 32, 13), // "statusChanged"
QT_MOC_LITERAL(2, 46, 0), // ""
QT_MOC_LITERAL(3, 47, 39), // "hector_rviz_overlay::QmlOverl..."
QT_MOC_LITERAL(4, 87, 6), // "status"
QT_MOC_LITERAL(5, 94, 14), // "contextCreated"
QT_MOC_LITERAL(6, 109, 14), // "onSceneChanged"
QT_MOC_LITERAL(7, 124, 17), // "onRenderRequested"
QT_MOC_LITERAL(8, 142, 19), // "onVisibilityChanged"
QT_MOC_LITERAL(9, 162, 11), // "handleEvent"
QT_MOC_LITERAL(10, 174, 8), // "receiver"
QT_MOC_LITERAL(11, 183, 7), // "QEvent*"
QT_MOC_LITERAL(12, 191, 5) // "event"

    },
    "hector_rviz_overlay::QmlOverlay\0"
    "statusChanged\0\0hector_rviz_overlay::QmlOverlay::Status\0"
    "status\0contextCreated\0onSceneChanged\0"
    "onRenderRequested\0onVisibilityChanged\0"
    "handleEvent\0receiver\0QEvent*\0event"
};
#undef QT_MOC_LITERAL

static const uint qt_meta_data_hector_rviz_overlay__QmlOverlay[] = {

 // content:
       8,       // revision
       0,       // classname
       0,    0, // classinfo
       6,   14, // methods
       0,    0, // properties
       0,    0, // enums/sets
       0,    0, // constructors
       0,       // flags
       2,       // signalCount

 // signals: name, argc, parameters, tag, flags
       1,    1,   44,    2, 0x06 /* Public */,
       5,    0,   47,    2, 0x06 /* Public */,

 // slots: name, argc, parameters, tag, flags
       6,    0,   48,    2, 0x09 /* Protected */,
       7,    0,   49,    2, 0x09 /* Protected */,
       8,    0,   50,    2, 0x09 /* Protected */,

 // methods: name, argc, parameters, tag, flags
       9,    2,   51,    2, 0x02 /* Public */,

 // signals: parameters
    QMetaType::Void, 0x80000000 | 3,    4,
    QMetaType::Void,

 // slots: parameters
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void,

 // methods: parameters
    QMetaType::Bool, QMetaType::QObjectStar, 0x80000000 | 11,   10,   12,

       0        // eod
};

void hector_rviz_overlay::QmlOverlay::qt_static_metacall(QObject *_o, QMetaObject::Call _c, int _id, void **_a)
{
    if (_c == QMetaObject::InvokeMetaMethod) {
        auto *_t = static_cast<QmlOverlay *>(_o);
        (void)_t;
        switch (_id) {
        case 0: _t->statusChanged((*reinterpret_cast< hector_rviz_overlay::QmlOverlay::Status(*)>(_a[1]))); break;
        case 1: _t->contextCreated(); break;
        case 2: _t->onSceneChanged(); break;
        case 3: _t->onRenderRequested(); break;
        case 4: _t->onVisibilityChanged(); break;
        case 5: { bool _r = _t->handleEvent((*reinterpret_cast< QObject*(*)>(_a[1])),(*reinterpret_cast< QEvent*(*)>(_a[2])));
            if (_a[0]) *reinterpret_cast< bool*>(_a[0]) = std::move(_r); }  break;
        default: ;
        }
    } else if (_c == QMetaObject::RegisterMethodArgumentMetaType) {
        switch (_id) {
        default: *reinterpret_cast<int*>(_a[0]) = -1; break;
        case 0:
            switch (*reinterpret_cast<int*>(_a[1])) {
            default: *reinterpret_cast<int*>(_a[0]) = -1; break;
            case 0:
                *reinterpret_cast<int*>(_a[0]) = qRegisterMetaType< hector_rviz_overlay::QmlOverlay::Status >(); break;
            }
            break;
        }
    } else if (_c == QMetaObject::IndexOfMethod) {
        int *result = reinterpret_cast<int *>(_a[0]);
        {
            using _t = void (QmlOverlay::*)(hector_rviz_overlay::QmlOverlay::Status );
            if (*reinterpret_cast<_t *>(_a[1]) == static_cast<_t>(&QmlOverlay::statusChanged)) {
                *result = 0;
                return;
            }
        }
        {
            using _t = void (QmlOverlay::*)();
            if (*reinterpret_cast<_t *>(_a[1]) == static_cast<_t>(&QmlOverlay::contextCreated)) {
                *result = 1;
                return;
            }
        }
    }
}

QT_INIT_METAOBJECT const QMetaObject hector_rviz_overlay::QmlOverlay::staticMetaObject = { {
    QMetaObject::SuperData::link<UiOverlay::staticMetaObject>(),
    qt_meta_stringdata_hector_rviz_overlay__QmlOverlay.data,
    qt_meta_data_hector_rviz_overlay__QmlOverlay,
    qt_static_metacall,
    nullptr,
    nullptr
} };


const QMetaObject *hector_rviz_overlay::QmlOverlay::metaObject() const
{
    return QObject::d_ptr->metaObject ? QObject::d_ptr->dynamicMetaObject() : &staticMetaObject;
}

void *hector_rviz_overlay::QmlOverlay::qt_metacast(const char *_clname)
{
    if (!_clname) return nullptr;
    if (!strcmp(_clname, qt_meta_stringdata_hector_rviz_overlay__QmlOverlay.stringdata0))
        return static_cast<void*>(this);
    return UiOverlay::qt_metacast(_clname);
}

int hector_rviz_overlay::QmlOverlay::qt_metacall(QMetaObject::Call _c, int _id, void **_a)
{
    _id = UiOverlay::qt_metacall(_c, _id, _a);
    if (_id < 0)
        return _id;
    if (_c == QMetaObject::InvokeMetaMethod) {
        if (_id < 6)
            qt_static_metacall(this, _c, _id, _a);
        _id -= 6;
    } else if (_c == QMetaObject::RegisterMethodArgumentMetaType) {
        if (_id < 6)
            qt_static_metacall(this, _c, _id, _a);
        _id -= 6;
    }
    return _id;
}

// SIGNAL 0
void hector_rviz_overlay::QmlOverlay::statusChanged(hector_rviz_overlay::QmlOverlay::Status _t1)
{
    void *_a[] = { nullptr, const_cast<void*>(reinterpret_cast<const void*>(std::addressof(_t1))) };
    QMetaObject::activate(this, &staticMetaObject, 0, _a);
}

// SIGNAL 1
void hector_rviz_overlay::QmlOverlay::contextCreated()
{
    QMetaObject::activate(this, &staticMetaObject, 1, nullptr);
}
QT_WARNING_POP
QT_END_MOC_NAMESPACE
