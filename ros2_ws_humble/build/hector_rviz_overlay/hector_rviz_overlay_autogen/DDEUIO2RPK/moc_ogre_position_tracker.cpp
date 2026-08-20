/****************************************************************************
** Meta object code from reading C++ file 'ogre_position_tracker.hpp'
**
** Created by: The Qt Meta Object Compiler version 67 (Qt 5.15.3)
**
** WARNING! All changes made in this file will be lost!
*****************************************************************************/

#include <memory>
#include "../../../../src/rviz-overlay-ros2/hector_rviz_overlay/hector_rviz_overlay/include/hector_rviz_overlay/positioning/ogre_position_tracker.hpp"
#include <QtCore/qbytearray.h>
#include <QtCore/qmetatype.h>
#if !defined(Q_MOC_OUTPUT_REVISION)
#error "The header file 'ogre_position_tracker.hpp' doesn't include <QObject>."
#elif Q_MOC_OUTPUT_REVISION != 67
#error "This file was generated using the moc from 5.15.3. It"
#error "cannot be used with the include files from this version of Qt."
#error "(The moc has changed too much.)"
#endif

QT_BEGIN_MOC_NAMESPACE
QT_WARNING_PUSH
QT_WARNING_DISABLE_DEPRECATED
struct qt_meta_stringdata_hector_rviz_overlay__positioning__OgrePositionTracker_t {
    QByteArrayData data[1];
    char stringdata0[54];
};
#define QT_MOC_LITERAL(idx, ofs, len) \
    Q_STATIC_BYTE_ARRAY_DATA_HEADER_INITIALIZER_WITH_OFFSET(len, \
    qptrdiff(offsetof(qt_meta_stringdata_hector_rviz_overlay__positioning__OgrePositionTracker_t, stringdata0) + ofs \
        - idx * sizeof(QByteArrayData)) \
    )
static const qt_meta_stringdata_hector_rviz_overlay__positioning__OgrePositionTracker_t qt_meta_stringdata_hector_rviz_overlay__positioning__OgrePositionTracker = {
    {
QT_MOC_LITERAL(0, 0, 53) // "hector_rviz_overlay::position..."

    },
    "hector_rviz_overlay::positioning::OgrePositionTracker"
};
#undef QT_MOC_LITERAL

static const uint qt_meta_data_hector_rviz_overlay__positioning__OgrePositionTracker[] = {

 // content:
       8,       // revision
       0,       // classname
       0,    0, // classinfo
       0,    0, // methods
       0,    0, // properties
       0,    0, // enums/sets
       0,    0, // constructors
       0,       // flags
       0,       // signalCount

       0        // eod
};

void hector_rviz_overlay::positioning::OgrePositionTracker::qt_static_metacall(QObject *_o, QMetaObject::Call _c, int _id, void **_a)
{
    (void)_o;
    (void)_id;
    (void)_c;
    (void)_a;
}

QT_INIT_METAOBJECT const QMetaObject hector_rviz_overlay::positioning::OgrePositionTracker::staticMetaObject = { {
    QMetaObject::SuperData::link<PositionTracker::staticMetaObject>(),
    qt_meta_stringdata_hector_rviz_overlay__positioning__OgrePositionTracker.data,
    qt_meta_data_hector_rviz_overlay__positioning__OgrePositionTracker,
    qt_static_metacall,
    nullptr,
    nullptr
} };


const QMetaObject *hector_rviz_overlay::positioning::OgrePositionTracker::metaObject() const
{
    return QObject::d_ptr->metaObject ? QObject::d_ptr->dynamicMetaObject() : &staticMetaObject;
}

void *hector_rviz_overlay::positioning::OgrePositionTracker::qt_metacast(const char *_clname)
{
    if (!_clname) return nullptr;
    if (!strcmp(_clname, qt_meta_stringdata_hector_rviz_overlay__positioning__OgrePositionTracker.stringdata0))
        return static_cast<void*>(this);
    return PositionTracker::qt_metacast(_clname);
}

int hector_rviz_overlay::positioning::OgrePositionTracker::qt_metacall(QMetaObject::Call _c, int _id, void **_a)
{
    _id = PositionTracker::qt_metacall(_c, _id, _a);
    return _id;
}
QT_WARNING_POP
QT_END_MOC_NAMESPACE
