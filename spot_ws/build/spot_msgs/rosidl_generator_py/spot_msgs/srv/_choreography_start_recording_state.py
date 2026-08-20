# generated from rosidl_generator_py/resource/_idl.py.em
# with input from spot_msgs:srv/ChoreographyStartRecordingState.idl
# generated code does not contain a copyright notice


# Import statements for member types

import builtins  # noqa: E402, I100

import math  # noqa: E402, I100

import rosidl_parser.definition  # noqa: E402, I100


class Metaclass_ChoreographyStartRecordingState_Request(type):
    """Metaclass of message 'ChoreographyStartRecordingState_Request'."""

    _CREATE_ROS_MESSAGE = None
    _CONVERT_FROM_PY = None
    _CONVERT_TO_PY = None
    _DESTROY_ROS_MESSAGE = None
    _TYPE_SUPPORT = None

    __constants = {
        'STATUS_UNKNOWN': 0,
        'STATUS_OK': 1,
        'STATUS_UNKNOWN_RECORDING_SESSION_ID': 2,
        'STATUS_RECORDING_BUFFER_FULL': 3,
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
                'spot_msgs.srv.ChoreographyStartRecordingState_Request')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__srv__choreography_start_recording_state__request
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__srv__choreography_start_recording_state__request
            cls._CONVERT_TO_PY = module.convert_to_py_msg__srv__choreography_start_recording_state__request
            cls._TYPE_SUPPORT = module.type_support_msg__srv__choreography_start_recording_state__request
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__srv__choreography_start_recording_state__request

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
            'STATUS_UNKNOWN': cls.__constants['STATUS_UNKNOWN'],
            'STATUS_OK': cls.__constants['STATUS_OK'],
            'STATUS_UNKNOWN_RECORDING_SESSION_ID': cls.__constants['STATUS_UNKNOWN_RECORDING_SESSION_ID'],
            'STATUS_RECORDING_BUFFER_FULL': cls.__constants['STATUS_RECORDING_BUFFER_FULL'],
        }

    @property
    def STATUS_UNKNOWN(self):
        """Message constant 'STATUS_UNKNOWN'."""
        return Metaclass_ChoreographyStartRecordingState_Request.__constants['STATUS_UNKNOWN']

    @property
    def STATUS_OK(self):
        """Message constant 'STATUS_OK'."""
        return Metaclass_ChoreographyStartRecordingState_Request.__constants['STATUS_OK']

    @property
    def STATUS_UNKNOWN_RECORDING_SESSION_ID(self):
        """Message constant 'STATUS_UNKNOWN_RECORDING_SESSION_ID'."""
        return Metaclass_ChoreographyStartRecordingState_Request.__constants['STATUS_UNKNOWN_RECORDING_SESSION_ID']

    @property
    def STATUS_RECORDING_BUFFER_FULL(self):
        """Message constant 'STATUS_RECORDING_BUFFER_FULL'."""
        return Metaclass_ChoreographyStartRecordingState_Request.__constants['STATUS_RECORDING_BUFFER_FULL']


class ChoreographyStartRecordingState_Request(metaclass=Metaclass_ChoreographyStartRecordingState_Request):
    """
    Message class 'ChoreographyStartRecordingState_Request'.

    Constants:
      STATUS_UNKNOWN
      STATUS_OK
      STATUS_UNKNOWN_RECORDING_SESSION_ID
      STATUS_RECORDING_BUFFER_FULL
    """

    __slots__ = [
        '_duration_seconds',
    ]

    _fields_and_field_types = {
        'duration_seconds': 'float',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.BasicType('float'),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        self.duration_seconds = kwargs.get('duration_seconds', float())

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
        if self.duration_seconds != other.duration_seconds:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def duration_seconds(self):
        """Message field 'duration_seconds'."""
        return self._duration_seconds

    @duration_seconds.setter
    def duration_seconds(self, value):
        if __debug__:
            assert \
                isinstance(value, float), \
                "The 'duration_seconds' field must be of type 'float'"
            assert not (value < -3.402823466e+38 or value > 3.402823466e+38) or math.isinf(value), \
                "The 'duration_seconds' field must be a float in [-3.402823466e+38, 3.402823466e+38]"
        self._duration_seconds = value


# Import statements for member types

# already imported above
# import builtins

# already imported above
# import rosidl_parser.definition


class Metaclass_ChoreographyStartRecordingState_Response(type):
    """Metaclass of message 'ChoreographyStartRecordingState_Response'."""

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
                'spot_msgs.srv.ChoreographyStartRecordingState_Response')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__srv__choreography_start_recording_state__response
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__srv__choreography_start_recording_state__response
            cls._CONVERT_TO_PY = module.convert_to_py_msg__srv__choreography_start_recording_state__response
            cls._TYPE_SUPPORT = module.type_support_msg__srv__choreography_start_recording_state__response
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__srv__choreography_start_recording_state__response

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class ChoreographyStartRecordingState_Response(metaclass=Metaclass_ChoreographyStartRecordingState_Response):
    """Message class 'ChoreographyStartRecordingState_Response'."""

    __slots__ = [
        '_success',
        '_message',
        '_status',
        '_recording_session_id',
    ]

    _fields_and_field_types = {
        'success': 'boolean',
        'message': 'string',
        'status': 'uint8',
        'recording_session_id': 'uint64',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.BasicType('boolean'),  # noqa: E501
        rosidl_parser.definition.UnboundedString(),  # noqa: E501
        rosidl_parser.definition.BasicType('uint8'),  # noqa: E501
        rosidl_parser.definition.BasicType('uint64'),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        self.success = kwargs.get('success', bool())
        self.message = kwargs.get('message', str())
        self.status = kwargs.get('status', int())
        self.recording_session_id = kwargs.get('recording_session_id', int())

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
        if self.recording_session_id != other.recording_session_id:
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
    def recording_session_id(self):
        """Message field 'recording_session_id'."""
        return self._recording_session_id

    @recording_session_id.setter
    def recording_session_id(self, value):
        if __debug__:
            assert \
                isinstance(value, int), \
                "The 'recording_session_id' field must be of type 'int'"
            assert value >= 0 and value < 18446744073709551616, \
                "The 'recording_session_id' field must be an unsigned integer in [0, 18446744073709551615]"
        self._recording_session_id = value


class Metaclass_ChoreographyStartRecordingState(type):
    """Metaclass of service 'ChoreographyStartRecordingState'."""

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
                'spot_msgs.srv.ChoreographyStartRecordingState')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._TYPE_SUPPORT = module.type_support_srv__srv__choreography_start_recording_state

            from spot_msgs.srv import _choreography_start_recording_state
            if _choreography_start_recording_state.Metaclass_ChoreographyStartRecordingState_Request._TYPE_SUPPORT is None:
                _choreography_start_recording_state.Metaclass_ChoreographyStartRecordingState_Request.__import_type_support__()
            if _choreography_start_recording_state.Metaclass_ChoreographyStartRecordingState_Response._TYPE_SUPPORT is None:
                _choreography_start_recording_state.Metaclass_ChoreographyStartRecordingState_Response.__import_type_support__()


class ChoreographyStartRecordingState(metaclass=Metaclass_ChoreographyStartRecordingState):
    from spot_msgs.srv._choreography_start_recording_state import ChoreographyStartRecordingState_Request as Request
    from spot_msgs.srv._choreography_start_recording_state import ChoreographyStartRecordingState_Response as Response

    def __init__(self):
        raise NotImplementedError('Service classes can not be instantiated')
