/****************************************************************************
** Meta object code from reading C++ file 'overlay.hpp'
**
** Created by: The Qt Meta Object Compiler version 67 (Qt 5.15.3)
**
** WARNING! All changes made in this file will be lost!
*****************************************************************************/

#include <memory>
#include "../../../../src/rviz-overlay-ros2/hector_rviz_overlay/hector_rviz_overlay/include/hector_rviz_overlay/overlay.hpp"
#include <QtCore/qbytearray.h>
#include <QtCore/qmetatype.h>
#if !defined(Q_MOC_OUTPUT_REVISION)
#error "The header file 'overlay.hpp' doesn't include <QObject>."
#elif Q_MOC_OUTPUT_REVISION != 67
#error "This file was generated using the moc from 5.15.3. It"
#error "cannot be used with the include files from this version of Qt."
#error "(The moc has changed too much.)"
#endif

QT_BEGIN_MOC_NAMESPACE
QT_WARNING_PUSH
QT_WARNING_DISABLE_DEPRECATED
struct qt_meta_stringdata_hector_rviz_overlay__Overlay_t {
    QByteArrayData data[3];
    char stringdata0[48];
};
#define QT_MOC_LITERAL(idx, ofs, len) \
    Q_STATIC_BYTE_ARRAY_DATA_HEADER_INITIALIZER_WITH_OFFSET(len, \
    qptrdiff(offsetof(qt_meta_stringdata_hector_rviz_overlay__Overlay_t, stringdata0) + ofs \
        - idx * sizeof(QByteArrayData)) \
    )
static const qt_meta_stringdata_hector_rviz_overlay__Overlay_t qt_meta_stringdata_hector_rviz_overlay__Overlay = {
    {
QT_MOC_LITERAL(0, 0, 28), // "hector_rviz_overlay::Overlay"
QT_MOC_LITERAL(1, 29, 17), // "visibilityChanged"
QT_MOC_LITERAL(2, 47, 0) // ""

    },
    "hector_rviz_overlay::Overlay\0"
    "visibilityChanged\0"
};
#undef QT_MOC_LITERAL

static const uint qt_meta_data_hector_rviz_overlay__Overlay[] = {

 // content:
       8,       // revision
       0,       // classname
       0,    0, // classinfo
       1,   14, // methods
       0,    0, // properties
       0,    0, // enums/sets
       0,    0, // constructors
       0,       // flags
       1,       // signalCount

 // signals: name, argc, parameters, tag, flags
       1,    0,   19,    2, 0x06 /* Public */,

 // signals: parameters
    QMetaType::Void,

       0        // eod
};

void hector_rviz_overlay::Overlay::qt_static_metacall(QObject *_o, QMetaObject::Call _c, int _id, void **_a)
{
    if (_c == QMetaObject::InvokeMetaMethod) {
        auto *_t = static_cast<Overlay *>(_o);
        (void)_t;
        switch (_id) {
        case 0: _t->visibilityChanged(); break;
        default: ;
        }
    } else if (_c == QMetaObject::IndexOfMethod) {
        int *result = reinterpret_cast<int *>(_a[0]);
        {
            using _t = void (Overlay::*)();
            if (*reinterpret_cast<_t *>(_a[1]) == static_cast<_t>(&Overlay::visibilityChanged)) {
                *result = 0;
                return;
            }
        }
    }
    (void)_a;
}

QT_INIT_METAOBJECT const QMetaObject hector_rviz_overlay::Overlay::staticMetaObject = { {
    QMetaObject::SuperData::link<QObject::staticMetaObject>(),
    qt_meta_stringdata_hector_rviz_overlay__Overlay.data,
    qt_meta_data_hector_rviz_overlay__Overlay,
    qt_static_metacall,
    nullptr,
    nullptr
} };


const QMetaObject *hector_rviz_overlay::Overlay::metaObject() const
{
    return QObject::d_ptr->metaObject ? QObject::d_ptr->dynamicMetaObject() : &staticMetaObject;
}

void *hector_rviz_overlay::Overlay::qt_metacast(const char *_clname)
{
    if (!_clname) return nullptr;
    if (!strcmp(_clname, qt_meta_stringdata_hector_rviz_overlay__Overlay.stringdata0))
        return static_cast<void*>(this);
    return QObject::qt_metacast(_clname);
}

int hector_rviz_overlay::Overlay::qt_metacall(QMetaObject::Call _c, int _id, void **_a)
{
    _id = QObject::qt_metacall(_c, _id, _a);
    if (_id < 0)
        return _id;
    if (_c == QMetaObject::InvokeMetaMethod) {
        if (_id < 1)
            qt_static_metacall(this, _c, _id, _a);
        _id -= 1;
    } else if (_c == QMetaObject::RegisterMethodArgumentMetaType) {
        if (_id < 1)
            *reinterpret_cast<int*>(_a[0]) = -1;
        _id -= 1;
    }
    return _id;
}

// SIGNAL 0
void hector_rviz_overlay::Overlay::visibilityChanged()
{
    QMetaObject::activate(this, &staticMetaObject, 0, nullptr);
}
QT_WARNING_POP
QT_END_MOC_NAMESPACE
