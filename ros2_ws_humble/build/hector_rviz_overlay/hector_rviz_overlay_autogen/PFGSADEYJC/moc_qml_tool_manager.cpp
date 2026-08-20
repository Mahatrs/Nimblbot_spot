/****************************************************************************
** Meta object code from reading C++ file 'qml_tool_manager.hpp'
**
** Created by: The Qt Meta Object Compiler version 67 (Qt 5.15.3)
**
** WARNING! All changes made in this file will be lost!
*****************************************************************************/

#include <memory>
#include "../../../../src/rviz-overlay-ros2/hector_rviz_overlay/hector_rviz_overlay/include/hector_rviz_overlay/helper/qml_tool_manager.hpp"
#include <QtCore/qbytearray.h>
#include <QtCore/qmetatype.h>
#if !defined(Q_MOC_OUTPUT_REVISION)
#error "The header file 'qml_tool_manager.hpp' doesn't include <QObject>."
#elif Q_MOC_OUTPUT_REVISION != 67
#error "This file was generated using the moc from 5.15.3. It"
#error "cannot be used with the include files from this version of Qt."
#error "(The moc has changed too much.)"
#endif

QT_BEGIN_MOC_NAMESPACE
QT_WARNING_PUSH
QT_WARNING_DISABLE_DEPRECATED
struct qt_meta_stringdata_hector_rviz_overlay__QmlTool_t {
    QByteArrayData data[11];
    char stringdata0[126];
};
#define QT_MOC_LITERAL(idx, ofs, len) \
    Q_STATIC_BYTE_ARRAY_DATA_HEADER_INITIALIZER_WITH_OFFSET(len, \
    qptrdiff(offsetof(qt_meta_stringdata_hector_rviz_overlay__QmlTool_t, stringdata0) + ofs \
        - idx * sizeof(QByteArrayData)) \
    )
static const qt_meta_stringdata_hector_rviz_overlay__QmlTool_t qt_meta_stringdata_hector_rviz_overlay__QmlTool = {
    {
QT_MOC_LITERAL(0, 0, 28), // "hector_rviz_overlay::QmlTool"
QT_MOC_LITERAL(1, 29, 17), // "isSelectedChanged"
QT_MOC_LITERAL(2, 47, 0), // ""
QT_MOC_LITERAL(3, 48, 10), // "isSelected"
QT_MOC_LITERAL(4, 59, 11), // "shortcutKey"
QT_MOC_LITERAL(5, 71, 13), // "accessAllKeys"
QT_MOC_LITERAL(6, 85, 4), // "name"
QT_MOC_LITERAL(7, 90, 11), // "description"
QT_MOC_LITERAL(8, 102, 7), // "classId"
QT_MOC_LITERAL(9, 110, 10), // "iconSource"
QT_MOC_LITERAL(10, 121, 4) // "tool"

    },
    "hector_rviz_overlay::QmlTool\0"
    "isSelectedChanged\0\0isSelected\0shortcutKey\0"
    "accessAllKeys\0name\0description\0classId\0"
    "iconSource\0tool"
};
#undef QT_MOC_LITERAL

static const uint qt_meta_data_hector_rviz_overlay__QmlTool[] = {

 // content:
       8,       // revision
       0,       // classname
       0,    0, // classinfo
       1,   14, // methods
       8,   20, // properties
       0,    0, // enums/sets
       0,    0, // constructors
       0,       // flags
       1,       // signalCount

 // signals: name, argc, parameters, tag, flags
       1,    0,   19,    2, 0x06 /* Public */,

 // signals: parameters
    QMetaType::Void,

 // properties: name, type, flags
       3, QMetaType::Bool, 0x00495001,
       4, QMetaType::QChar, 0x00095401,
       5, QMetaType::Bool, 0x00095401,
       6, QMetaType::QString, 0x00095401,
       7, QMetaType::QString, 0x00095401,
       8, QMetaType::QString, 0x00095401,
       9, QMetaType::QString, 0x00095401,
      10, QMetaType::QObjectStar, 0x00095401,

 // properties: notify_signal_id
       0,
       0,
       0,
       0,
       0,
       0,
       0,
       0,

       0        // eod
};

void hector_rviz_overlay::QmlTool::qt_static_metacall(QObject *_o, QMetaObject::Call _c, int _id, void **_a)
{
    if (_c == QMetaObject::InvokeMetaMethod) {
        auto *_t = static_cast<QmlTool *>(_o);
        (void)_t;
        switch (_id) {
        case 0: _t->isSelectedChanged(); break;
        default: ;
        }
    } else if (_c == QMetaObject::IndexOfMethod) {
        int *result = reinterpret_cast<int *>(_a[0]);
        {
            using _t = void (QmlTool::*)();
            if (*reinterpret_cast<_t *>(_a[1]) == static_cast<_t>(&QmlTool::isSelectedChanged)) {
                *result = 0;
                return;
            }
        }
    }
#ifndef QT_NO_PROPERTIES
    else if (_c == QMetaObject::ReadProperty) {
        auto *_t = static_cast<QmlTool *>(_o);
        (void)_t;
        void *_v = _a[0];
        switch (_id) {
        case 0: *reinterpret_cast< bool*>(_v) = _t->isSelected(); break;
        case 1: *reinterpret_cast< QChar*>(_v) = _t->shortcutKey(); break;
        case 2: *reinterpret_cast< bool*>(_v) = _t->accessAllKeys(); break;
        case 3: *reinterpret_cast< QString*>(_v) = _t->name(); break;
        case 4: *reinterpret_cast< QString*>(_v) = _t->description(); break;
        case 5: *reinterpret_cast< QString*>(_v) = _t->classId(); break;
        case 6: *reinterpret_cast< QString*>(_v) = _t->iconSource(); break;
        case 7: *reinterpret_cast< QObject**>(_v) = _t->tool(); break;
        default: break;
        }
    } else if (_c == QMetaObject::WriteProperty) {
    } else if (_c == QMetaObject::ResetProperty) {
    }
#endif // QT_NO_PROPERTIES
    (void)_a;
}

QT_INIT_METAOBJECT const QMetaObject hector_rviz_overlay::QmlTool::staticMetaObject = { {
    QMetaObject::SuperData::link<QObject::staticMetaObject>(),
    qt_meta_stringdata_hector_rviz_overlay__QmlTool.data,
    qt_meta_data_hector_rviz_overlay__QmlTool,
    qt_static_metacall,
    nullptr,
    nullptr
} };


const QMetaObject *hector_rviz_overlay::QmlTool::metaObject() const
{
    return QObject::d_ptr->metaObject ? QObject::d_ptr->dynamicMetaObject() : &staticMetaObject;
}

void *hector_rviz_overlay::QmlTool::qt_metacast(const char *_clname)
{
    if (!_clname) return nullptr;
    if (!strcmp(_clname, qt_meta_stringdata_hector_rviz_overlay__QmlTool.stringdata0))
        return static_cast<void*>(this);
    return QObject::qt_metacast(_clname);
}

int hector_rviz_overlay::QmlTool::qt_metacall(QMetaObject::Call _c, int _id, void **_a)
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
#ifndef QT_NO_PROPERTIES
    else if (_c == QMetaObject::ReadProperty || _c == QMetaObject::WriteProperty
            || _c == QMetaObject::ResetProperty || _c == QMetaObject::RegisterPropertyMetaType) {
        qt_static_metacall(this, _c, _id, _a);
        _id -= 8;
    } else if (_c == QMetaObject::QueryPropertyDesignable) {
        _id -= 8;
    } else if (_c == QMetaObject::QueryPropertyScriptable) {
        _id -= 8;
    } else if (_c == QMetaObject::QueryPropertyStored) {
        _id -= 8;
    } else if (_c == QMetaObject::QueryPropertyEditable) {
        _id -= 8;
    } else if (_c == QMetaObject::QueryPropertyUser) {
        _id -= 8;
    }
#endif // QT_NO_PROPERTIES
    return _id;
}

// SIGNAL 0
void hector_rviz_overlay::QmlTool::isSelectedChanged()
{
    QMetaObject::activate(this, &staticMetaObject, 0, nullptr);
}
struct qt_meta_stringdata_hector_rviz_overlay__QmlToolManager_t {
    QByteArrayData data[21];
    char stringdata0[250];
};
#define QT_MOC_LITERAL(idx, ofs, len) \
    Q_STATIC_BYTE_ARRAY_DATA_HEADER_INITIALIZER_WITH_OFFSET(len, \
    qptrdiff(offsetof(qt_meta_stringdata_hector_rviz_overlay__QmlToolManager_t, stringdata0) + ofs \
        - idx * sizeof(QByteArrayData)) \
    )
static const qt_meta_stringdata_hector_rviz_overlay__QmlToolManager_t qt_meta_stringdata_hector_rviz_overlay__QmlToolManager = {
    {
QT_MOC_LITERAL(0, 0, 35), // "hector_rviz_overlay::QmlToolM..."
QT_MOC_LITERAL(1, 36, 9), // "toolAdded"
QT_MOC_LITERAL(2, 46, 0), // ""
QT_MOC_LITERAL(3, 47, 11), // "toolRemoved"
QT_MOC_LITERAL(4, 59, 11), // "toolChanged"
QT_MOC_LITERAL(5, 71, 12), // "toolsChanged"
QT_MOC_LITERAL(6, 84, 17), // "disconnectSignals"
QT_MOC_LITERAL(7, 102, 11), // "onToolAdded"
QT_MOC_LITERAL(8, 114, 18), // "rviz_common::Tool*"
QT_MOC_LITERAL(9, 133, 13), // "onToolRemoved"
QT_MOC_LITERAL(10, 147, 13), // "onToolChanged"
QT_MOC_LITERAL(11, 161, 7), // "addTool"
QT_MOC_LITERAL(12, 169, 17), // "class_lookup_name"
QT_MOC_LITERAL(13, 187, 7), // "getTool"
QT_MOC_LITERAL(14, 195, 8), // "class_id"
QT_MOC_LITERAL(15, 204, 10), // "removeTool"
QT_MOC_LITERAL(16, 215, 1), // "i"
QT_MOC_LITERAL(17, 217, 4), // "tool"
QT_MOC_LITERAL(18, 222, 9), // "removeAll"
QT_MOC_LITERAL(19, 232, 5), // "tools"
QT_MOC_LITERAL(20, 238, 11) // "currentTool"

    },
    "hector_rviz_overlay::QmlToolManager\0"
    "toolAdded\0\0toolRemoved\0toolChanged\0"
    "toolsChanged\0disconnectSignals\0"
    "onToolAdded\0rviz_common::Tool*\0"
    "onToolRemoved\0onToolChanged\0addTool\0"
    "class_lookup_name\0getTool\0class_id\0"
    "removeTool\0i\0tool\0removeAll\0tools\0"
    "currentTool"
};
#undef QT_MOC_LITERAL

static const uint qt_meta_data_hector_rviz_overlay__QmlToolManager[] = {

 // content:
       8,       // revision
       0,       // classname
       0,    0, // classinfo
      14,   14, // methods
       2,  118, // properties
       0,    0, // enums/sets
       0,    0, // constructors
       0,       // flags
       4,       // signalCount

 // signals: name, argc, parameters, tag, flags
       1,    1,   84,    2, 0x06 /* Public */,
       3,    1,   87,    2, 0x06 /* Public */,
       4,    1,   90,    2, 0x06 /* Public */,
       5,    0,   93,    2, 0x06 /* Public */,

 // slots: name, argc, parameters, tag, flags
       6,    0,   94,    2, 0x08 /* Private */,
       7,    1,   95,    2, 0x08 /* Private */,
       9,    1,   98,    2, 0x08 /* Private */,
      10,    1,  101,    2, 0x08 /* Private */,

 // methods: name, argc, parameters, tag, flags
      11,    0,  104,    2, 0x02 /* Public */,
      11,    1,  105,    2, 0x02 /* Public */,
      13,    1,  108,    2, 0x02 /* Public */,
      15,    1,  111,    2, 0x02 /* Public */,
      15,    1,  114,    2, 0x02 /* Public */,
      18,    0,  117,    2, 0x02 /* Public */,

 // signals: parameters
    QMetaType::Void, QMetaType::QObjectStar,    2,
    QMetaType::Void, QMetaType::QObjectStar,    2,
    QMetaType::Void, QMetaType::QObjectStar,    2,
    QMetaType::Void,

 // slots: parameters
    QMetaType::Void,
    QMetaType::Void, 0x80000000 | 8,    2,
    QMetaType::Void, 0x80000000 | 8,    2,
    QMetaType::Void, 0x80000000 | 8,    2,

 // methods: parameters
    QMetaType::QObjectStar,
    QMetaType::QObjectStar, QMetaType::QString,   12,
    QMetaType::QObjectStar, QMetaType::QString,   14,
    QMetaType::Void, QMetaType::Int,   16,
    QMetaType::Void, QMetaType::QObjectStar,   17,
    QMetaType::Void,

 // properties: name, type, flags
      19, QMetaType::QVariantList, 0x00495001,
      20, QMetaType::QObjectStar, 0x00495103,

 // properties: notify_signal_id
       3,
       2,

       0        // eod
};

void hector_rviz_overlay::QmlToolManager::qt_static_metacall(QObject *_o, QMetaObject::Call _c, int _id, void **_a)
{
    if (_c == QMetaObject::InvokeMetaMethod) {
        auto *_t = static_cast<QmlToolManager *>(_o);
        (void)_t;
        switch (_id) {
        case 0: _t->toolAdded((*reinterpret_cast< QObject*(*)>(_a[1]))); break;
        case 1: _t->toolRemoved((*reinterpret_cast< QObject*(*)>(_a[1]))); break;
        case 2: _t->toolChanged((*reinterpret_cast< QObject*(*)>(_a[1]))); break;
        case 3: _t->toolsChanged(); break;
        case 4: _t->disconnectSignals(); break;
        case 5: _t->onToolAdded((*reinterpret_cast< rviz_common::Tool*(*)>(_a[1]))); break;
        case 6: _t->onToolRemoved((*reinterpret_cast< rviz_common::Tool*(*)>(_a[1]))); break;
        case 7: _t->onToolChanged((*reinterpret_cast< rviz_common::Tool*(*)>(_a[1]))); break;
        case 8: { QObject* _r = _t->addTool();
            if (_a[0]) *reinterpret_cast< QObject**>(_a[0]) = std::move(_r); }  break;
        case 9: { QObject* _r = _t->addTool((*reinterpret_cast< const QString(*)>(_a[1])));
            if (_a[0]) *reinterpret_cast< QObject**>(_a[0]) = std::move(_r); }  break;
        case 10: { QObject* _r = _t->getTool((*reinterpret_cast< const QString(*)>(_a[1])));
            if (_a[0]) *reinterpret_cast< QObject**>(_a[0]) = std::move(_r); }  break;
        case 11: _t->removeTool((*reinterpret_cast< int(*)>(_a[1]))); break;
        case 12: _t->removeTool((*reinterpret_cast< QObject*(*)>(_a[1]))); break;
        case 13: _t->removeAll(); break;
        default: ;
        }
    } else if (_c == QMetaObject::RegisterMethodArgumentMetaType) {
        switch (_id) {
        default: *reinterpret_cast<int*>(_a[0]) = -1; break;
        case 5:
            switch (*reinterpret_cast<int*>(_a[1])) {
            default: *reinterpret_cast<int*>(_a[0]) = -1; break;
            case 0:
                *reinterpret_cast<int*>(_a[0]) = qRegisterMetaType< rviz_common::Tool* >(); break;
            }
            break;
        case 6:
            switch (*reinterpret_cast<int*>(_a[1])) {
            default: *reinterpret_cast<int*>(_a[0]) = -1; break;
            case 0:
                *reinterpret_cast<int*>(_a[0]) = qRegisterMetaType< rviz_common::Tool* >(); break;
            }
            break;
        case 7:
            switch (*reinterpret_cast<int*>(_a[1])) {
            default: *reinterpret_cast<int*>(_a[0]) = -1; break;
            case 0:
                *reinterpret_cast<int*>(_a[0]) = qRegisterMetaType< rviz_common::Tool* >(); break;
            }
            break;
        }
    } else if (_c == QMetaObject::IndexOfMethod) {
        int *result = reinterpret_cast<int *>(_a[0]);
        {
            using _t = void (QmlToolManager::*)(QObject * );
            if (*reinterpret_cast<_t *>(_a[1]) == static_cast<_t>(&QmlToolManager::toolAdded)) {
                *result = 0;
                return;
            }
        }
        {
            using _t = void (QmlToolManager::*)(QObject * );
            if (*reinterpret_cast<_t *>(_a[1]) == static_cast<_t>(&QmlToolManager::toolRemoved)) {
                *result = 1;
                return;
            }
        }
        {
            using _t = void (QmlToolManager::*)(QObject * );
            if (*reinterpret_cast<_t *>(_a[1]) == static_cast<_t>(&QmlToolManager::toolChanged)) {
                *result = 2;
                return;
            }
        }
        {
            using _t = void (QmlToolManager::*)();
            if (*reinterpret_cast<_t *>(_a[1]) == static_cast<_t>(&QmlToolManager::toolsChanged)) {
                *result = 3;
                return;
            }
        }
    }
#ifndef QT_NO_PROPERTIES
    else if (_c == QMetaObject::ReadProperty) {
        auto *_t = static_cast<QmlToolManager *>(_o);
        (void)_t;
        void *_v = _a[0];
        switch (_id) {
        case 0: *reinterpret_cast< QVariantList*>(_v) = _t->tools(); break;
        case 1: *reinterpret_cast< QObject**>(_v) = _t->currentTool(); break;
        default: break;
        }
    } else if (_c == QMetaObject::WriteProperty) {
        auto *_t = static_cast<QmlToolManager *>(_o);
        (void)_t;
        void *_v = _a[0];
        switch (_id) {
        case 1: _t->setCurrentTool(*reinterpret_cast< QObject**>(_v)); break;
        default: break;
        }
    } else if (_c == QMetaObject::ResetProperty) {
    }
#endif // QT_NO_PROPERTIES
}

QT_INIT_METAOBJECT const QMetaObject hector_rviz_overlay::QmlToolManager::staticMetaObject = { {
    QMetaObject::SuperData::link<QObject::staticMetaObject>(),
    qt_meta_stringdata_hector_rviz_overlay__QmlToolManager.data,
    qt_meta_data_hector_rviz_overlay__QmlToolManager,
    qt_static_metacall,
    nullptr,
    nullptr
} };


const QMetaObject *hector_rviz_overlay::QmlToolManager::metaObject() const
{
    return QObject::d_ptr->metaObject ? QObject::d_ptr->dynamicMetaObject() : &staticMetaObject;
}

void *hector_rviz_overlay::QmlToolManager::qt_metacast(const char *_clname)
{
    if (!_clname) return nullptr;
    if (!strcmp(_clname, qt_meta_stringdata_hector_rviz_overlay__QmlToolManager.stringdata0))
        return static_cast<void*>(this);
    return QObject::qt_metacast(_clname);
}

int hector_rviz_overlay::QmlToolManager::qt_metacall(QMetaObject::Call _c, int _id, void **_a)
{
    _id = QObject::qt_metacall(_c, _id, _a);
    if (_id < 0)
        return _id;
    if (_c == QMetaObject::InvokeMetaMethod) {
        if (_id < 14)
            qt_static_metacall(this, _c, _id, _a);
        _id -= 14;
    } else if (_c == QMetaObject::RegisterMethodArgumentMetaType) {
        if (_id < 14)
            qt_static_metacall(this, _c, _id, _a);
        _id -= 14;
    }
#ifndef QT_NO_PROPERTIES
    else if (_c == QMetaObject::ReadProperty || _c == QMetaObject::WriteProperty
            || _c == QMetaObject::ResetProperty || _c == QMetaObject::RegisterPropertyMetaType) {
        qt_static_metacall(this, _c, _id, _a);
        _id -= 2;
    } else if (_c == QMetaObject::QueryPropertyDesignable) {
        _id -= 2;
    } else if (_c == QMetaObject::QueryPropertyScriptable) {
        _id -= 2;
    } else if (_c == QMetaObject::QueryPropertyStored) {
        _id -= 2;
    } else if (_c == QMetaObject::QueryPropertyEditable) {
        _id -= 2;
    } else if (_c == QMetaObject::QueryPropertyUser) {
        _id -= 2;
    }
#endif // QT_NO_PROPERTIES
    return _id;
}

// SIGNAL 0
void hector_rviz_overlay::QmlToolManager::toolAdded(QObject * _t1)
{
    void *_a[] = { nullptr, const_cast<void*>(reinterpret_cast<const void*>(std::addressof(_t1))) };
    QMetaObject::activate(this, &staticMetaObject, 0, _a);
}

// SIGNAL 1
void hector_rviz_overlay::QmlToolManager::toolRemoved(QObject * _t1)
{
    void *_a[] = { nullptr, const_cast<void*>(reinterpret_cast<const void*>(std::addressof(_t1))) };
    QMetaObject::activate(this, &staticMetaObject, 1, _a);
}

// SIGNAL 2
void hector_rviz_overlay::QmlToolManager::toolChanged(QObject * _t1)
{
    void *_a[] = { nullptr, const_cast<void*>(reinterpret_cast<const void*>(std::addressof(_t1))) };
    QMetaObject::activate(this, &staticMetaObject, 2, _a);
}

// SIGNAL 3
void hector_rviz_overlay::QmlToolManager::toolsChanged()
{
    QMetaObject::activate(this, &staticMetaObject, 3, nullptr);
}
QT_WARNING_POP
QT_END_MOC_NAMESPACE
