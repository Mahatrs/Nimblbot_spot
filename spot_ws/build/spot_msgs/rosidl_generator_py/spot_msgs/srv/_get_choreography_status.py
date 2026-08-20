# generated from rosidl_generator_py/resource/_idl.py.em
# with input from spot_msgs:srv/GetChoreographyStatus.idl
# generated code does not contain a copyright notice


# Import statements for member types

import rosidl_parser.definition  # noqa: E402, I100


class Metaclass_GetChoreographyStatus_Request(type):
    """Metaclass of message 'GetChoreographyStatus_Request'."""

    _CREATE_ROS_MESSAGE = None
    _CONVERT_FROM_PY = None
    _CONVERT_TO_PY = None
    _DESTROY_ROS_MESSAGE = None
    _TYPE_SUPPORT = None

    __constants = {
        'STATUS_UNKNOWN': 0,
        'STATUS_DANCING': 1,
        'STATUS_COMPLETED_SEQUENCE': 2,
        'STATUS_PREPPING': 3,
        'STATUS_WAITING_FOR_START_TIME': 4,
        'STATUS_VALIDATING': 5,
        'STATUS_INTERRUPTED': 6,
        'STATUS_FALLEN': 7,
        'STATUS_POWERED_OFF': 8,
        'STATUS_OTHER': 9,
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
                'spot_msgs.srv.GetChoreographyStatus_Request')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__srv__get_choreography_status__request
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__srv__get_choreography_status__request
            cls._CONVERT_TO_PY = module.convert_to_py_msg__srv__get_choreography_status__request
            cls._TYPE_SUPPORT = module.type_support_msg__srv__get_choreography_status__request
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__srv__get_choreography_status__request

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
            'STATUS_UNKNOWN': cls.__constants['STATUS_UNKNOWN'],
            'STATUS_DANCING': cls.__constants['STATUS_DANCING'],
            'STATUS_COMPLETED_SEQUENCE': cls.__constants['STATUS_COMPLETED_SEQUENCE'],
            'STATUS_PREPPING': cls.__constants['STATUS_PREPPING'],
            'STATUS_WAITING_FOR_START_TIME': cls.__constants['STATUS_WAITING_FOR_START_TIME'],
            'STATUS_VALIDATING': cls.__constants['STATUS_VALIDATING'],
            'STATUS_INTERRUPTED': cls.__constants['STATUS_INTERRUPTED'],
            'STATUS_FALLEN': cls.__constants['STATUS_FALLEN'],
            'STATUS_POWERED_OFF': cls.__constants['STATUS_POWERED_OFF'],
            'STATUS_OTHER': cls.__constants['STATUS_OTHER'],
        }

    @property
    def STATUS_UNKNOWN(self):
        """Message constant 'STATUS_UNKNOWN'."""
        return Metaclass_GetChoreographyStatus_Request.__constants['STATUS_UNKNOWN']

    @property
    def STATUS_DANCING(self):
        """Message constant 'STATUS_DANCING'."""
        return Metaclass_GetChoreographyStatus_Request.__constants['STATUS_DANCING']

    @property
    def STATUS_COMPLETED_SEQUENCE(self):
        """Message constant 'STATUS_COMPLETED_SEQUENCE'."""
        return Metaclass_GetChoreographyStatus_Request.__constants['STATUS_COMPLETED_SEQUENCE']

    @property
    def STATUS_PREPPING(self):
        """Message constant 'STATUS_PREPPING'."""
        return Metaclass_GetChoreographyStatus_Request.__constants['STATUS_PREPPING']

    @property
    def STATUS_WAITING_FOR_START_TIME(self):
        """Message constant 'STATUS_WAITING_FOR_START_TIME'."""
        return Metaclass_GetChoreographyStatus_Request.__constants['STATUS_WAITING_FOR_START_TIME']

    @property
    def STATUS_VALIDATING(self):
        """Message constant 'STATUS_VALIDATING'."""
        return Metaclass_GetChoreographyStatus_Request.__constants['STATUS_VALIDATING']

    @property
    def STATUS_INTERRUPTED(self):
        """Message constant 'STATUS_INTERRUPTED'."""
        return Metaclass_GetChoreographyStatus_Request.__constants['STATUS_INTERRUPTED']

    @property
    def STATUS_FALLEN(self):
        """Message constant 'STATUS_FALLEN'."""
        return Metaclass_GetChoreographyStatus_Request.__constants['STATUS_FALLEN']

    @property
    def STATUS_POWERED_OFF(self):
        """Message constant 'STATUS_POWERED_OFF'."""
        return Metaclass_GetChoreographyStatus_Request.__constants['STATUS_POWERED_OFF']

    @property
    def STATUS_OTHER(self):
        """Message constant 'STATUS_OTHER'."""
        return Metaclass_GetChoreographyStatus_Request.__constants['STATUS_OTHER']


class GetChoreographyStatus_Request(metaclass=Metaclass_GetChoreographyStatus_Request):
    """
    Message class 'GetChoreographyStatus_Request'.

    Constants:
      STATUS_UNKNOWN
      STATUS_DANCING
      STATUS_COMPLETED_SEQUENCE
      STATUS_PREPPING
      STATUS_WAITING_FOR_START_TIME
      STATUS_VALIDATING
      STATUS_INTERRUPTED
      STATUS_FALLEN
      STATUS_POWERED_OFF
      STATUS_OTHER
    """

    __slots__ = [
    ]

    _fields_and_field_types = {
    }

    SLOT_TYPES = (
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))

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
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)


# Import statements for member types

import builtins  # noqa: E402, I100

# already imported above
# import rosidl_parser.definition


class Metaclass_GetChoreographyStatus_Response(type):
    """Metaclass of message 'GetChoreographyStatus_Response'."""

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
                'spot_msgs.srv.GetChoreographyStatus_Response')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__srv__get_choreography_status__response
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__srv__get_choreography_status__response
            cls._CONVERT_TO_PY = module.convert_to_py_msg__srv__get_choreography_status__response
            cls._TYPE_SUPPORT = module.type_support_msg__srv__get_choreography_status__response
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__srv__get_choreography_status__response

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class GetChoreographyStatus_Response(metaclass=Metaclass_GetChoreographyStatus_Response):
    """Message class 'GetChoreographyStatus_Response'."""

    __slots__ = [
        '_success',
        '_message',
        '_status',
        '_execution_id',
    ]

    _fields_and_field_types = {
        'success': 'boolean',
        'message': 'string',
        'status': 'uint8',
        'execution_id': 'int32',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.BasicType('boolean'),  # noqa: E501
        rosidl_parser.definition.UnboundedString(),  # noqa: E501
        rosidl_parser.definition.BasicType('uint8'),  # noqa: E501
        rosidl_parser.definition.BasicType('int32'),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        self.success = kwargs.get('success', bool())
        self.message = kwargs.get('message', str())
        self.status = kwargs.get('status', int())
        self.execution_id = kwargs.get('execution_id', int())

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
        if self.status != other.status:
            return False
        if self.execution_id != other.execution_id:
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
    def status(self):
        """Message field 'status'."""
        return self._status

    @status.setter
    def status(self, value):
        if __debug__:
            assert \
                isinstance(value, int), \
                "The 'status' field must be of type 'int'"
            assert value >= 0 and value < 256, \
                "The 'status' field must be an unsigned integer in [0, 255]"
        self._status = value

    @builtins.property
    def execution_id(self):
        """Message field 'execution_id'."""
        return self._execution_id

    @execution_id.setter
    def execution_id(self, value):
        if __debug__:
            assert \
                isinstance(value, int), \
                "The 'execution_id' field must be of type 'int'"
            assert value >= -2147483648 and value < 2147483648, \
                "The 'execution_id' field must be an integer in [-2147483648, 2147483647]"
        self._execution_id = value


class Metaclass_GetChoreographyStatus(type):
    """Metaclass of service 'GetChoreographyStatus'."""

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
                'spot_msgs.srv.GetChoreographyStatus')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._TYPE_SUPPORT = module.type_support_srv__srv__get_choreography_status

            from spot_msgs.srv import _get_choreography_status
            if _get_choreography_status.Metaclass_GetChoreographyStatus_Request._TYPE_SUPPORT is None:
                _get_choreography_status.Metaclass_GetChoreographyStatus_Request.__import_type_support__()
            if _get_choreography_status.Metaclass_GetChoreographyStatus_Response._TYPE_SUPPORT is None:
                _get_choreography_status.Metaclass_GetChoreographyStatus_Response.__import_type_support__()


class GetChoreographyStatus(metaclass=Metaclass_GetChoreographyStatus):
    from spot_msgs.srv._get_choreography_status import GetChoreographyStatus_Request as Request
    from spot_msgs.srv._get_choreography_status import GetChoreographyStatus_Response as Response

    def __init__(self):
        raise NotImplementedError('Service classes can not be instantiated')
