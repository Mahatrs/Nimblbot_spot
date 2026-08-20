# generated from rosidl_generator_py/resource/_idl.py.em
# with input from rviz_manager_msgs:msg/ManagerLaunch.idl
# generated code does not contain a copyright notice


# Import statements for member types

import builtins  # noqa: E402, I100

import rosidl_parser.definition  # noqa: E402, I100


class Metaclass_ManagerLaunch(type):
    """Metaclass of message 'ManagerLaunch'."""

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
            module = import_type_support('rviz_manager_msgs')
        except ImportError:
            import logging
            import traceback
            logger = logging.getLogger(
                'rviz_manager_msgs.msg.ManagerLaunch')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__msg__manager_launch
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__msg__manager_launch
            cls._CONVERT_TO_PY = module.convert_to_py_msg__msg__manager_launch
            cls._TYPE_SUPPORT = module.type_support_msg__msg__manager_launch
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__msg__manager_launch

            from std_msgs.msg import Header
            if Header.__class__._TYPE_SUPPORT is None:
                Header.__class__.__import_type_support__()

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class ManagerLaunch(metaclass=Metaclass_ManagerLaunch):
    """Message class 'ManagerLaunch'."""

    __slots__ = [
        '_header',
        '_id',
        '_action',
        '_ns',
        '_bash_session',
        '_is_launch_file',
        '_package',
        '_executable',
        '_arguments',
        '_ros_arguments',
        '_working_dir',
        '_session_name',
        '_use_sim_time',
        '_timeout',
    ]

    _fields_and_field_types = {
        'header': 'std_msgs/Header',
        'id': 'int32',
        'action': 'string',
        'ns': 'string',
        'bash_session': 'boolean',
        'is_launch_file': 'boolean',
        'package': 'string',
        'executable': 'string',
        'arguments': 'sequence<string>',
        'ros_arguments': 'sequence<string>',
        'working_dir': 'string',
        'session_name': 'string',
        'use_sim_time': 'boolean',
        'timeout': 'int32',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.NamespacedType(['std_msgs', 'msg'], 'Header'),  # noqa: E501
        rosidl_parser.definition.BasicType('int32'),  # noqa: E501
        rosidl_parser.definition.UnboundedString(),  # noqa: E501
        rosidl_parser.definition.UnboundedString(),  # noqa: E501
        rosidl_parser.definition.BasicType('boolean'),  # noqa: E501
        rosidl_parser.definition.BasicType('boolean'),  # noqa: E501
        rosidl_parser.definition.UnboundedString(),  # noqa: E501
        rosidl_parser.definition.UnboundedString(),  # noqa: E501
        rosidl_parser.definition.UnboundedSequence(rosidl_parser.definition.UnboundedString()),  # noqa: E501
        rosidl_parser.definition.UnboundedSequence(rosidl_parser.definition.UnboundedString()),  # noqa: E501
        rosidl_parser.definition.UnboundedString(),  # noqa: E501
        rosidl_parser.definition.UnboundedString(),  # noqa: E501
        rosidl_parser.definition.BasicType('boolean'),  # noqa: E501
        rosidl_parser.definition.BasicType('int32'),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        from std_msgs.msg import Header
        self.header = kwargs.get('header', Header())
        self.id = kwargs.get('id', int())
        self.action = kwargs.get('action', str())
        self.ns = kwargs.get('ns', str())
        self.bash_session = kwargs.get('bash_session', bool())
        self.is_launch_file = kwargs.get('is_launch_file', bool())
        self.package = kwargs.get('package', str())
        self.executable = kwargs.get('executable', str())
        self.arguments = kwargs.get('arguments', [])
        self.ros_arguments = kwargs.get('ros_arguments', [])
        self.working_dir = kwargs.get('working_dir', str())
        self.session_name = kwargs.get('session_name', str())
        self.use_sim_time = kwargs.get('use_sim_time', bool())
        self.timeout = kwargs.get('timeout', int())

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
        if self.header != other.header:
            return False
        if self.id != other.id:
            return False
        if self.action != other.action:
            return False
        if self.ns != other.ns:
            return False
        if self.bash_session != other.bash_session:
            return False
        if self.is_launch_file != other.is_launch_file:
            return False
        if self.package != other.package:
            return False
        if self.executable != other.executable:
            return False
        if self.arguments != other.arguments:
            return False
        if self.ros_arguments != other.ros_arguments:
            return False
        if self.working_dir != other.working_dir:
            return False
        if self.session_name != other.session_name:
            return False
        if self.use_sim_time != other.use_sim_time:
            return False
        if self.timeout != other.timeout:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def header(self):
        """Message field 'header'."""
        return self._header

    @header.setter
    def header(self, value):
        if __debug__:
            from std_msgs.msg import Header
            assert \
                isinstance(value, Header), \
                "The 'header' field must be a sub message of type 'Header'"
        self._header = value

    @builtins.property  # noqa: A003
    def id(self):  # noqa: A003
        """Message field 'id'."""
        return self._id

    @id.setter  # noqa: A003
    def id(self, value):  # noqa: A003
        if __debug__:
            assert \
                isinstance(value, int), \
                "The 'id' field must be of type 'int'"
            assert value >= -2147483648 and value < 2147483648, \
                "The 'id' field must be an integer in [-2147483648, 2147483647]"
        self._id = value

    @builtins.property
    def action(self):
        """Message field 'action'."""
        return self._action

    @action.setter
    def action(self, value):
        if __debug__:
            assert \
                isinstance(value, str), \
                "The 'action' field must be of type 'str'"
        self._action = value

    @builtins.property
    def ns(self):
        """Message field 'ns'."""
        return self._ns

    @ns.setter
    def ns(self, value):
        if __debug__:
            assert \
                isinstance(value, str), \
                "The 'ns' field must be of type 'str'"
        self._ns = value

    @builtins.property
    def bash_session(self):
        """Message field 'bash_session'."""
        return self._bash_session

    @bash_session.setter
    def bash_session(self, value):
        if __debug__:
            assert \
                isinstance(value, bool), \
                "The 'bash_session' field must be of type 'bool'"
        self._bash_session = value

    @builtins.property
    def is_launch_file(self):
        """Message field 'is_launch_file'."""
        return self._is_launch_file

    @is_launch_file.setter
    def is_launch_file(self, value):
        if __debug__:
            assert \
                isinstance(value, bool), \
                "The 'is_launch_file' field must be of type 'bool'"
        self._is_launch_file = value

    @builtins.property
    def package(self):
        """Message field 'package'."""
        return self._package

    @package.setter
    def package(self, value):
        if __debug__:
            assert \
                isinstance(value, str), \
                "The 'package' field must be of type 'str'"
        self._package = value

    @builtins.property
    def executable(self):
        """Message field 'executable'."""
        return self._executable

    @executable.setter
    def executable(self, value):
        if __debug__:
            assert \
                isinstance(value, str), \
                "The 'executable' field must be of type 'str'"
        self._executable = value

    @builtins.property
    def arguments(self):
        """Message field 'arguments'."""
        return self._arguments

    @arguments.setter
    def arguments(self, value):
        if __debug__:
            from collections.abc import Sequence
            from collections.abc import Set
            from collections import UserList
            from collections import UserString
            assert \
                ((isinstance(value, Sequence) or
                  isinstance(value, Set) or
                  isinstance(value, UserList)) and
                 not isinstance(value, str) and
                 not isinstance(value, UserString) and
                 all(isinstance(v, str) for v in value) and
                 True), \
                "The 'arguments' field must be a set or sequence and each value of type 'str'"
        self._arguments = value

    @builtins.property
    def ros_arguments(self):
        """Message field 'ros_arguments'."""
        return self._ros_arguments

    @ros_arguments.setter
    def ros_arguments(self, value):
        if __debug__:
            from collections.abc import Sequence
            from collections.abc import Set
            from collections import UserList
            from collections import UserString
            assert \
                ((isinstance(value, Sequence) or
                  isinstance(value, Set) or
                  isinstance(value, UserList)) and
                 not isinstance(value, str) and
                 not isinstance(value, UserString) and
                 all(isinstance(v, str) for v in value) and
                 True), \
                "The 'ros_arguments' field must be a set or sequence and each value of type 'str'"
        self._ros_arguments = value

    @builtins.property
    def working_dir(self):
        """Message field 'working_dir'."""
        return self._working_dir

    @working_dir.setter
    def working_dir(self, value):
        if __debug__:
            assert \
                isinstance(value, str), \
                "The 'working_dir' field must be of type 'str'"
        self._working_dir = value

    @builtins.property
    def session_name(self):
        """Message field 'session_name'."""
        return self._session_name

    @session_name.setter
    def session_name(self, value):
        if __debug__:
            assert \
                isinstance(value, str), \
                "The 'session_name' field must be of type 'str'"
        self._session_name = value

    @builtins.property
    def use_sim_time(self):
        """Message field 'use_sim_time'."""
        return self._use_sim_time

    @use_sim_time.setter
    def use_sim_time(self, value):
        if __debug__:
            assert \
                isinstance(value, bool), \
                "The 'use_sim_time' field must be of type 'bool'"
        self._use_sim_time = value

    @builtins.property
    def timeout(self):
        """Message field 'timeout'."""
        return self._timeout

    @timeout.setter
    def timeout(self, value):
        if __debug__:
            assert \
                isinstance(value, int), \
                "The 'timeout' field must be of type 'int'"
            assert value >= -2147483648 and value < 2147483648, \
                "The 'timeout' field must be an integer in [-2147483648, 2147483647]"
        self._timeout = value
