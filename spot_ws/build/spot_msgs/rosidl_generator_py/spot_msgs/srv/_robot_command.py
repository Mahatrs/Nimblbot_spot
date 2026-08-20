# generated from rosidl_generator_py/resource/_idl.py.em
# with input from spot_msgs:srv/RobotCommand.idl
# generated code does not contain a copyright notice


# Import statements for member types

import builtins  # noqa: E402, I100

import rosidl_parser.definition  # noqa: E402, I100


class Metaclass_RobotCommand_Request(type):
    """Metaclass of message 'RobotCommand_Request'."""

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
                'spot_msgs.srv.RobotCommand_Request')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__srv__robot_command__request
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__srv__robot_command__request
            cls._CONVERT_TO_PY = module.convert_to_py_msg__srv__robot_command__request
            cls._TYPE_SUPPORT = module.type_support_msg__srv__robot_command__request
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__srv__robot_command__request

            from bosdyn_api_msgs.msg import RobotCommand
            if RobotCommand.__class__._TYPE_SUPPORT is None:
                RobotCommand.__class__.__import_type_support__()

            from builtin_interfaces.msg import Duration
            if Duration.__class__._TYPE_SUPPORT is None:
                Duration.__class__.__import_type_support__()

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class RobotCommand_Request(metaclass=Metaclass_RobotCommand_Request):
    """Message class 'RobotCommand_Request'."""

    __slots__ = [
        '_command',
        '_duration',
    ]

    _fields_and_field_types = {
        'command': 'bosdyn_api_msgs/RobotCommand',
        'duration': 'builtin_interfaces/Duration',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.NamespacedType(['bosdyn_api_msgs', 'msg'], 'RobotCommand'),  # noqa: E501
        rosidl_parser.definition.NamespacedType(['builtin_interfaces', 'msg'], 'Duration'),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        from bosdyn_api_msgs.msg import RobotCommand
        self.command = kwargs.get('command', RobotCommand())
        from builtin_interfaces.msg import Duration
        self.duration = kwargs.get('duration', Duration())

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
        if self.command != other.command:
            return False
        if self.duration != other.duration:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def command(self):
        """Message field 'command'."""
        return self._command

    @command.setter
    def command(self, value):
        if __debug__:
            from bosdyn_api_msgs.msg import RobotCommand
            assert \
                isinstance(value, RobotCommand), \
                "The 'command' field must be a sub message of type 'RobotCommand'"
        self._command = value

    @builtins.property
    def duration(self):
        """Message field 'duration'."""
        return self._duration

    @duration.setter
    def duration(self, value):
        if __debug__:
            from builtin_interfaces.msg import Duration
            assert \
                isinstance(value, Duration), \
                "The 'duration' field must be a sub message of type 'Duration'"
        self._duration = value


# Import statements for member types

# already imported above
# import builtins

# already imported above
# import rosidl_parser.definition


class Metaclass_RobotCommand_Response(type):
    """Metaclass of message 'RobotCommand_Response'."""

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
                'spot_msgs.srv.RobotCommand_Response')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__srv__robot_command__response
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__srv__robot_command__response
            cls._CONVERT_TO_PY = module.convert_to_py_msg__srv__robot_command__response
            cls._TYPE_SUPPORT = module.type_support_msg__srv__robot_command__response
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__srv__robot_command__response

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class RobotCommand_Response(metaclass=Metaclass_RobotCommand_Response):
    """Message class 'RobotCommand_Response'."""

    __slots__ = [
        '_success',
        '_message',
        '_robot_command_id',
    ]

    _fields_and_field_types = {
        'success': 'boolean',
        'message': 'string',
        'robot_command_id': 'uint32',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.BasicType('boolean'),  # noqa: E501
        rosidl_parser.definition.UnboundedString(),  # noqa: E501
        rosidl_parser.definition.BasicType('uint32'),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        self.success = kwargs.get('success', bool())
        self.message = kwargs.get('message', str())
        self.robot_command_id = kwargs.get('robot_command_id', int())

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
        if self.robot_command_id != other.robot_command_id:
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
    def robot_command_id(self):
        """Message field 'robot_command_id'."""
        return self._robot_command_id

    @robot_command_id.setter
    def robot_command_id(self, value):
        if __debug__:
            assert \
                isinstance(value, int), \
                "The 'robot_command_id' field must be of type 'int'"
            assert value >= 0 and value < 4294967296, \
                "The 'robot_command_id' field must be an unsigned integer in [0, 4294967295]"
        self._robot_command_id = value


class Metaclass_RobotCommand(type):
    """Metaclass of service 'RobotCommand'."""

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
                'spot_msgs.srv.RobotCommand')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._TYPE_SUPPORT = module.type_support_srv__srv__robot_command

            from spot_msgs.srv import _robot_command
            if _robot_command.Metaclass_RobotCommand_Request._TYPE_SUPPORT is None:
                _robot_command.Metaclass_RobotCommand_Request.__import_type_support__()
            if _robot_command.Metaclass_RobotCommand_Response._TYPE_SUPPORT is None:
                _robot_command.Metaclass_RobotCommand_Response.__import_type_support__()


class RobotCommand(metaclass=Metaclass_RobotCommand):
    from spot_msgs.srv._robot_command import RobotCommand_Request as Request
    from spot_msgs.srv._robot_command import RobotCommand_Response as Response

    def __init__(self):
        raise NotImplementedError('Service classes can not be instantiated')
