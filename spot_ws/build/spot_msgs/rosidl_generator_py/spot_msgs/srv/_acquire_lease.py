# generated from rosidl_generator_py/resource/_idl.py.em
# with input from spot_msgs:srv/AcquireLease.idl
# generated code does not contain a copyright notice


# Import statements for member types

import builtins  # noqa: E402, I100

import rosidl_parser.definition  # noqa: E402, I100


class Metaclass_AcquireLease_Request(type):
    """Metaclass of message 'AcquireLease_Request'."""

    _CREATE_ROS_MESSAGE = None
    _CONVERT_FROM_PY = None
    _CONVERT_TO_PY = None
    _DESTROY_ROS_MESSAGE = None
    _TYPE_SUPPORT = None

    __constants = {
    }

    @classmethod
    def __import_type_support__(cls):
        try:
            from rosidl_generator_py import import_type_support
            module = import_type_support('spot_msgs')
        except ImportError:
            import logging
            import traceback
            logger = logging.getLogger(
                'spot_msgs.srv.AcquireLease_Request')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__srv__acquire_lease__request
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__srv__acquire_lease__request
            cls._CONVERT_TO_PY = module.convert_to_py_msg__srv__acquire_lease__request
            cls._TYPE_SUPPORT = module.type_support_msg__srv__acquire_lease__request
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__srv__acquire_lease__request

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class AcquireLease_Request(metaclass=Metaclass_AcquireLease_Request):
    """Message class 'AcquireLease_Request'."""

    __slots__ = [
        '_client_name',
        '_resource_name',
        '_force',
    ]

    _fields_and_field_types = {
        'client_name': 'string',
        'resource_name': 'string',
        'force': 'boolean',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.UnboundedString(),  # noqa: E501
        rosidl_parser.definition.UnboundedString(),  # noqa: E501
        rosidl_parser.definition.BasicType('boolean'),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        self.client_name = kwargs.get('client_name', str())
        self.resource_name = kwargs.get('resource_name', str())
        self.force = kwargs.get('force', bool())

    def __repr__(self):
        typename = self.__class__.__module__.split('.')
        typename.pop()
        typename.append(self.__class__.__name__)
        args = []
        for s, t in zip(self.__slots__, self.SLOT_TYPES):
            field = getattr(self, s)
            fieldstr = repr(field)
            # We use Python array type for fields that can be directly stored
            # in them, and "normal" sequences for everything else.  If it is
            # a type that we store in an array, strip off the 'array' portion.
            if (
                isinstance(t, rosidl_parser.definition.AbstractSequence) and
                isinstance(t.value_type, rosidl_parser.definition.BasicType) and
                t.value_type.typename in ['float', 'double', 'int8', 'uint8', 'int16', 'uint16', 'int32', 'uint32', 'int64', 'uint64']
            ):
                if len(field) == 0:
                    fieldstr = '[]'
                else:
                    assert fieldstr.startswith('array(')
                    prefix = "array('X', "
                    suffix = ')'
                    fieldstr = fieldstr[len(prefix):-len(suffix)]
            args.append(s[1:] + '=' + fieldstr)
        return '%s(%s)' % ('.'.join(typename), ', '.join(args))

    def __eq__(self, other):
        if not isinstance(other, self.__class__):
            return False
        if self.client_name != other.client_name:
            return False
        if self.resource_name != other.resource_name:
            return False
        if self.force != other.force:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def client_name(self):
        """Message field 'client_name'."""
        return self._client_name

    @client_name.setter
    def client_name(self, value):
        if __debug__:
            assert \
                isinstance(value, str), \
                "The 'client_name' field must be of type 'str'"
        self._client_name = value

    @builtins.property
    def resource_name(self):
        """Message field 'resource_name'."""
        return self._resource_name

    @resource_name.setter
    def resource_name(self, value):
        if __debug__:
            assert \
                isinstance(value, str), \
                "The 'resource_name' field must be of type 'str'"
        self._resource_name = value

    @builtins.property
    def force(self):
        """Message field 'force'."""
        return self._force

    @force.setter
    def force(self, value):
        if __debug__:
            assert \
                isinstance(value, bool), \
                "The 'force' field must be of type 'bool'"
        self._force = value


# Import statements for member types

# already imported above
# import builtins

# already imported above
# import rosidl_parser.definition


class Metaclass_AcquireLease_Response(type):
    """Metaclass of message 'AcquireLease_Response'."""

    _CREATE_ROS_MESSAGE = None
    _CONVERT_FROM_PY = None
    _CONVERT_TO_PY = None
    _DESTROY_ROS_MESSAGE = None
    _TYPE_SUPPORT = None

    __constants = {
    }

    @classmethod
    def __import_type_support__(cls):
        try:
            from rosidl_generator_py import import_type_support
            module = import_type_support('spot_msgs')
        except ImportError:
            import logging
            import traceback
            logger = logging.getLogger(
                'spot_msgs.srv.AcquireLease_Response')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__srv__acquire_lease__response
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__srv__acquire_lease__response
            cls._CONVERT_TO_PY = module.convert_to_py_msg__srv__acquire_lease__response
            cls._TYPE_SUPPORT = module.type_support_msg__srv__acquire_lease__response
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__srv__acquire_lease__response

            from bosdyn_api_msgs.msg import Lease
            if Lease.__class__._TYPE_SUPPORT is None:
                Lease.__class__.__import_type_support__()

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class AcquireLease_Response(metaclass=Metaclass_AcquireLease_Response):
    """Message class 'AcquireLease_Response'."""

    __slots__ = [
        '_success',
        '_message',
        '_lease',
    ]

    _fields_and_field_types = {
        'success': 'boolean',
        'message': 'string',
        'lease': 'bosdyn_api_msgs/Lease',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.BasicType('boolean'),  # noqa: E501
        rosidl_parser.definition.UnboundedString(),  # noqa: E501
        rosidl_parser.definition.NamespacedType(['bosdyn_api_msgs', 'msg'], 'Lease'),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        self.success = kwargs.get('success', bool())
        self.message = kwargs.get('message', str())
        from bosdyn_api_msgs.msg import Lease
        self.lease = kwargs.get('lease', Lease())

    def __repr__(self):
        typename = self.__class__.__module__.split('.')
        typename.pop()
        typename.append(self.__class__.__name__)
        args = []
        for s, t in zip(self.__slots__, self.SLOT_TYPES):
            field = getattr(self, s)
            fieldstr = repr(field)
            # We use Python array type for fields that can be directly stored
            # in them, and "normal" sequences for everything else.  If it is
            # a type that we store in an array, strip off the 'array' portion.
            if (
                isinstance(t, rosidl_parser.definition.AbstractSequence) and
                isinstance(t.value_type, rosidl_parser.definition.BasicType) and
                t.value_type.typename in ['float', 'double', 'int8', 'uint8', 'int16', 'uint16', 'int32', 'uint32', 'int64', 'uint64']
            ):
                if len(field) == 0:
                    fieldstr = '[]'
                else:
                    assert fieldstr.startswith('array(')
                    prefix = "array('X', "
                    suffix = ')'
                    fieldstr = fieldstr[len(prefix):-len(suffix)]
            args.append(s[1:] + '=' + fieldstr)
        return '%s(%s)' % ('.'.join(typename), ', '.join(args))

    def __eq__(self, other):
        if not isinstance(other, self.__class__):
            return False
        if self.success != other.success:
            return False
        if self.message != other.message:
            return False
        if self.lease != other.lease:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def success(self):
        """Message field 'success'."""
        return self._success

    @success.setter
    def success(self, value):
        if __debug__:
            assert \
                isinstance(value, bool), \
                "The 'success' field must be of type 'bool'"
        self._success = value

    @builtins.property
    def message(self):
        """Message field 'message'."""
        return self._message

    @message.setter
    def message(self, value):
        if __debug__:
            assert \
                isinstance(value, str), \
                "The 'message' field must be of type 'str'"
        self._message = value

    @builtins.property
    def lease(self):
        """Message field 'lease'."""
        return self._lease

    @lease.setter
    def lease(self, value):
        if __debug__:
            from bosdyn_api_msgs.msg import Lease
            assert \
                isinstance(value, Lease), \
                "The 'lease' field must be a sub message of type 'Lease'"
        self._lease = value


class Metaclass_AcquireLease(type):
    """Metaclass of service 'AcquireLease'."""

    _TYPE_SUPPORT = None

    @classmethod
    def __import_type_support__(cls):
        try:
            from rosidl_generator_py import import_type_support
            module = import_type_support('spot_msgs')
        except ImportError:
            import logging
            import traceback
            logger = logging.getLogger(
                'spot_msgs.srv.AcquireLease')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._TYPE_SUPPORT = module.type_support_srv__srv__acquire_lease

            from spot_msgs.srv import _acquire_lease
            if _acquire_lease.Metaclass_AcquireLease_Request._TYPE_SUPPORT is None:
                _acquire_lease.Metaclass_AcquireLease_Request.__import_type_support__()
            if _acquire_lease.Metaclass_AcquireLease_Response._TYPE_SUPPORT is None:
                _acquire_lease.Metaclass_AcquireLease_Response.__import_type_support__()


class AcquireLease(metaclass=Metaclass_AcquireLease):
    from spot_msgs.srv._acquire_lease import AcquireLease_Request as Request
    from spot_msgs.srv._acquire_lease import AcquireLease_Response as Response

    def __init__(self):
        raise NotImplementedError('Service classes can not be instantiated')
