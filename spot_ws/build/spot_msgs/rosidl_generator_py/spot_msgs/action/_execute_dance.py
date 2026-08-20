# generated from rosidl_generator_py/resource/_idl.py.em
# with input from spot_msgs:action/ExecuteDance.idl
# generated code does not contain a copyright notice


# Import statements for member types

# Member 'choreo_sequence_serialized'
import array  # noqa: E402, I100

import builtins  # noqa: E402, I100

import rosidl_parser.definition  # noqa: E402, I100


class Metaclass_ExecuteDance_Goal(type):
    """Metaclass of message 'ExecuteDance_Goal'."""

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
                'spot_msgs.action.ExecuteDance_Goal')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__action__execute_dance__goal
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__action__execute_dance__goal
            cls._CONVERT_TO_PY = module.convert_to_py_msg__action__execute_dance__goal
            cls._TYPE_SUPPORT = module.type_support_msg__action__execute_dance__goal
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__action__execute_dance__goal

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class ExecuteDance_Goal(metaclass=Metaclass_ExecuteDance_Goal):
    """Message class 'ExecuteDance_Goal'."""

    __slots__ = [
        '_choreo_name',
        '_choreo_file_content',
        '_choreo_sequence_serialized',
        '_start_slice',
    ]

    _fields_and_field_types = {
        'choreo_name': 'string',
        'choreo_file_content': 'string',
        'choreo_sequence_serialized': 'sequence<uint8>',
        'start_slice': 'uint32',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.UnboundedString(),  # noqa: E501
        rosidl_parser.definition.UnboundedString(),  # noqa: E501
        rosidl_parser.definition.UnboundedSequence(rosidl_parser.definition.BasicType('uint8')),  # noqa: E501
        rosidl_parser.definition.BasicType('uint32'),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        self.choreo_name = kwargs.get('choreo_name', str())
        self.choreo_file_content = kwargs.get('choreo_file_content', str())
        self.choreo_sequence_serialized = array.array('B', kwargs.get('choreo_sequence_serialized', []))
        self.start_slice = kwargs.get('start_slice', int())

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
        if self.choreo_name != other.choreo_name:
            return False
        if self.choreo_file_content != other.choreo_file_content:
            return False
        if self.choreo_sequence_serialized != other.choreo_sequence_serialized:
            return False
        if self.start_slice != other.start_slice:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def choreo_name(self):
        """Message field 'choreo_name'."""
        return self._choreo_name

    @choreo_name.setter
    def choreo_name(self, value):
        if __debug__:
            assert \
                isinstance(value, str), \
                "The 'choreo_name' field must be of type 'str'"
        self._choreo_name = value

    @builtins.property
    def choreo_file_content(self):
        """Message field 'choreo_file_content'."""
        return self._choreo_file_content

    @choreo_file_content.setter
    def choreo_file_content(self, value):
        if __debug__:
            assert \
                isinstance(value, str), \
                "The 'choreo_file_content' field must be of type 'str'"
        self._choreo_file_content = value

    @builtins.property
    def choreo_sequence_serialized(self):
        """Message field 'choreo_sequence_serialized'."""
        return self._choreo_sequence_serialized

    @choreo_sequence_serialized.setter
    def choreo_sequence_serialized(self, value):
        if isinstance(value, array.array):
            assert value.typecode == 'B', \
                "The 'choreo_sequence_serialized' array.array() must have the type code of 'B'"
            self._choreo_sequence_serialized = value
            return
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
                 all(isinstance(v, int) for v in value) and
                 all(val >= 0 and val < 256 for val in value)), \
                "The 'choreo_sequence_serialized' field must be a set or sequence and each value of type 'int' and each unsigned integer in [0, 255]"
        self._choreo_sequence_serialized = array.array('B', value)

    @builtins.property
    def start_slice(self):
        """Message field 'start_slice'."""
        return self._start_slice

    @start_slice.setter
    def start_slice(self, value):
        if __debug__:
            assert \
                isinstance(value, int), \
                "The 'start_slice' field must be of type 'int'"
            assert value >= 0 and value < 4294967296, \
                "The 'start_slice' field must be an unsigned integer in [0, 4294967295]"
        self._start_slice = value


# Import statements for member types

# already imported above
# import builtins

# already imported above
# import rosidl_parser.definition


class Metaclass_ExecuteDance_Result(type):
    """Metaclass of message 'ExecuteDance_Result'."""

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
                'spot_msgs.action.ExecuteDance_Result')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__action__execute_dance__result
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__action__execute_dance__result
            cls._CONVERT_TO_PY = module.convert_to_py_msg__action__execute_dance__result
            cls._TYPE_SUPPORT = module.type_support_msg__action__execute_dance__result
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__action__execute_dance__result

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class ExecuteDance_Result(metaclass=Metaclass_ExecuteDance_Result):
    """Message class 'ExecuteDance_Result'."""

    __slots__ = [
        '_success',
        '_message',
    ]

    _fields_and_field_types = {
        'success': 'boolean',
        'message': 'string',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.BasicType('boolean'),  # noqa: E501
        rosidl_parser.definition.UnboundedString(),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        self.success = kwargs.get('success', bool())
        self.message = kwargs.get('message', str())

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


# Import statements for member types

# already imported above
# import builtins

# already imported above
# import rosidl_parser.definition


class Metaclass_ExecuteDance_Feedback(type):
    """Metaclass of message 'ExecuteDance_Feedback'."""

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
                'spot_msgs.action.ExecuteDance_Feedback')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__action__execute_dance__feedback
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__action__execute_dance__feedback
            cls._CONVERT_TO_PY = module.convert_to_py_msg__action__execute_dance__feedback
            cls._TYPE_SUPPORT = module.type_support_msg__action__execute_dance__feedback
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__action__execute_dance__feedback

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class ExecuteDance_Feedback(metaclass=Metaclass_ExecuteDance_Feedback):
    """Message class 'ExecuteDance_Feedback'."""

    __slots__ = [
        '_is_dancing',
    ]

    _fields_and_field_types = {
        'is_dancing': 'boolean',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.BasicType('boolean'),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        self.is_dancing = kwargs.get('is_dancing', bool())

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
        if self.is_dancing != other.is_dancing:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def is_dancing(self):
        """Message field 'is_dancing'."""
        return self._is_dancing

    @is_dancing.setter
    def is_dancing(self, value):
        if __debug__:
            assert \
                isinstance(value, bool), \
                "The 'is_dancing' field must be of type 'bool'"
        self._is_dancing = value


# Import statements for member types

# already imported above
# import builtins

# already imported above
# import rosidl_parser.definition


class Metaclass_ExecuteDance_SendGoal_Request(type):
    """Metaclass of message 'ExecuteDance_SendGoal_Request'."""

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
                'spot_msgs.action.ExecuteDance_SendGoal_Request')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__action__execute_dance__send_goal__request
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__action__execute_dance__send_goal__request
            cls._CONVERT_TO_PY = module.convert_to_py_msg__action__execute_dance__send_goal__request
            cls._TYPE_SUPPORT = module.type_support_msg__action__execute_dance__send_goal__request
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__action__execute_dance__send_goal__request

            from spot_msgs.action import ExecuteDance
            if ExecuteDance.Goal.__class__._TYPE_SUPPORT is None:
                ExecuteDance.Goal.__class__.__import_type_support__()

            from unique_identifier_msgs.msg import UUID
            if UUID.__class__._TYPE_SUPPORT is None:
                UUID.__class__.__import_type_support__()

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class ExecuteDance_SendGoal_Request(metaclass=Metaclass_ExecuteDance_SendGoal_Request):
    """Message class 'ExecuteDance_SendGoal_Request'."""

    __slots__ = [
        '_goal_id',
        '_goal',
    ]

    _fields_and_field_types = {
        'goal_id': 'unique_identifier_msgs/UUID',
        'goal': 'spot_msgs/ExecuteDance_Goal',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.NamespacedType(['unique_identifier_msgs', 'msg'], 'UUID'),  # noqa: E501
        rosidl_parser.definition.NamespacedType(['spot_msgs', 'action'], 'ExecuteDance_Goal'),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        from unique_identifier_msgs.msg import UUID
        self.goal_id = kwargs.get('goal_id', UUID())
        from spot_msgs.action._execute_dance import ExecuteDance_Goal
        self.goal = kwargs.get('goal', ExecuteDance_Goal())

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
        if self.goal_id != other.goal_id:
            return False
        if self.goal != other.goal:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def goal_id(self):
        """Message field 'goal_id'."""
        return self._goal_id

    @goal_id.setter
    def goal_id(self, value):
        if __debug__:
            from unique_identifier_msgs.msg import UUID
            assert \
                isinstance(value, UUID), \
                "The 'goal_id' field must be a sub message of type 'UUID'"
        self._goal_id = value

    @builtins.property
    def goal(self):
        """Message field 'goal'."""
        return self._goal

    @goal.setter
    def goal(self, value):
        if __debug__:
            from spot_msgs.action._execute_dance import ExecuteDance_Goal
            assert \
                isinstance(value, ExecuteDance_Goal), \
                "The 'goal' field must be a sub message of type 'ExecuteDance_Goal'"
        self._goal = value


# Import statements for member types

# already imported above
# import builtins

# already imported above
# import rosidl_parser.definition


class Metaclass_ExecuteDance_SendGoal_Response(type):
    """Metaclass of message 'ExecuteDance_SendGoal_Response'."""

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
                'spot_msgs.action.ExecuteDance_SendGoal_Response')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__action__execute_dance__send_goal__response
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__action__execute_dance__send_goal__response
            cls._CONVERT_TO_PY = module.convert_to_py_msg__action__execute_dance__send_goal__response
            cls._TYPE_SUPPORT = module.type_support_msg__action__execute_dance__send_goal__response
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__action__execute_dance__send_goal__response

            from builtin_interfaces.msg import Time
            if Time.__class__._TYPE_SUPPORT is None:
                Time.__class__.__import_type_support__()

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class ExecuteDance_SendGoal_Response(metaclass=Metaclass_ExecuteDance_SendGoal_Response):
    """Message class 'ExecuteDance_SendGoal_Response'."""

    __slots__ = [
        '_accepted',
        '_stamp',
    ]

    _fields_and_field_types = {
        'accepted': 'boolean',
        'stamp': 'builtin_interfaces/Time',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.BasicType('boolean'),  # noqa: E501
        rosidl_parser.definition.NamespacedType(['builtin_interfaces', 'msg'], 'Time'),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        self.accepted = kwargs.get('accepted', bool())
        from builtin_interfaces.msg import Time
        self.stamp = kwargs.get('stamp', Time())

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
        if self.accepted != other.accepted:
            return False
        if self.stamp != other.stamp:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def accepted(self):
        """Message field 'accepted'."""
        return self._accepted

    @accepted.setter
    def accepted(self, value):
        if __debug__:
            assert \
                isinstance(value, bool), \
                "The 'accepted' field must be of type 'bool'"
        self._accepted = value

    @builtins.property
    def stamp(self):
        """Message field 'stamp'."""
        return self._stamp

    @stamp.setter
    def stamp(self, value):
        if __debug__:
            from builtin_interfaces.msg import Time
            assert \
                isinstance(value, Time), \
                "The 'stamp' field must be a sub message of type 'Time'"
        self._stamp = value


class Metaclass_ExecuteDance_SendGoal(type):
    """Metaclass of service 'ExecuteDance_SendGoal'."""

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
                'spot_msgs.action.ExecuteDance_SendGoal')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._TYPE_SUPPORT = module.type_support_srv__action__execute_dance__send_goal

            from spot_msgs.action import _execute_dance
            if _execute_dance.Metaclass_ExecuteDance_SendGoal_Request._TYPE_SUPPORT is None:
                _execute_dance.Metaclass_ExecuteDance_SendGoal_Request.__import_type_support__()
            if _execute_dance.Metaclass_ExecuteDance_SendGoal_Response._TYPE_SUPPORT is None:
                _execute_dance.Metaclass_ExecuteDance_SendGoal_Response.__import_type_support__()


class ExecuteDance_SendGoal(metaclass=Metaclass_ExecuteDance_SendGoal):
    from spot_msgs.action._execute_dance import ExecuteDance_SendGoal_Request as Request
    from spot_msgs.action._execute_dance import ExecuteDance_SendGoal_Response as Response

    def __init__(self):
        raise NotImplementedError('Service classes can not be instantiated')


# Import statements for member types

# already imported above
# import builtins

# already imported above
# import rosidl_parser.definition


class Metaclass_ExecuteDance_GetResult_Request(type):
    """Metaclass of message 'ExecuteDance_GetResult_Request'."""

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
                'spot_msgs.action.ExecuteDance_GetResult_Request')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__action__execute_dance__get_result__request
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__action__execute_dance__get_result__request
            cls._CONVERT_TO_PY = module.convert_to_py_msg__action__execute_dance__get_result__request
            cls._TYPE_SUPPORT = module.type_support_msg__action__execute_dance__get_result__request
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__action__execute_dance__get_result__request

            from unique_identifier_msgs.msg import UUID
            if UUID.__class__._TYPE_SUPPORT is None:
                UUID.__class__.__import_type_support__()

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class ExecuteDance_GetResult_Request(metaclass=Metaclass_ExecuteDance_GetResult_Request):
    """Message class 'ExecuteDance_GetResult_Request'."""

    __slots__ = [
        '_goal_id',
    ]

    _fields_and_field_types = {
        'goal_id': 'unique_identifier_msgs/UUID',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.NamespacedType(['unique_identifier_msgs', 'msg'], 'UUID'),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        from unique_identifier_msgs.msg import UUID
        self.goal_id = kwargs.get('goal_id', UUID())

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
        if self.goal_id != other.goal_id:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def goal_id(self):
        """Message field 'goal_id'."""
        return self._goal_id

    @goal_id.setter
    def goal_id(self, value):
        if __debug__:
            from unique_identifier_msgs.msg import UUID
            assert \
                isinstance(value, UUID), \
                "The 'goal_id' field must be a sub message of type 'UUID'"
        self._goal_id = value


# Import statements for member types

# already imported above
# import builtins

# already imported above
# import rosidl_parser.definition


class Metaclass_ExecuteDance_GetResult_Response(type):
    """Metaclass of message 'ExecuteDance_GetResult_Response'."""

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
                'spot_msgs.action.ExecuteDance_GetResult_Response')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__action__execute_dance__get_result__response
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__action__execute_dance__get_result__response
            cls._CONVERT_TO_PY = module.convert_to_py_msg__action__execute_dance__get_result__response
            cls._TYPE_SUPPORT = module.type_support_msg__action__execute_dance__get_result__response
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__action__execute_dance__get_result__response

            from spot_msgs.action import ExecuteDance
            if ExecuteDance.Result.__class__._TYPE_SUPPORT is None:
                ExecuteDance.Result.__class__.__import_type_support__()

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class ExecuteDance_GetResult_Response(metaclass=Metaclass_ExecuteDance_GetResult_Response):
    """Message class 'ExecuteDance_GetResult_Response'."""

    __slots__ = [
        '_status',
        '_result',
    ]

    _fields_and_field_types = {
        'status': 'int8',
        'result': 'spot_msgs/ExecuteDance_Result',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.BasicType('int8'),  # noqa: E501
        rosidl_parser.definition.NamespacedType(['spot_msgs', 'action'], 'ExecuteDance_Result'),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        self.status = kwargs.get('status', int())
        from spot_msgs.action._execute_dance import ExecuteDance_Result
        self.result = kwargs.get('result', ExecuteDance_Result())

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
        if self.status != other.status:
            return False
        if self.result != other.result:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)

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
            assert value >= -128 and value < 128, \
                "The 'status' field must be an integer in [-128, 127]"
        self._status = value

    @builtins.property
    def result(self):
        """Message field 'result'."""
        return self._result

    @result.setter
    def result(self, value):
        if __debug__:
            from spot_msgs.action._execute_dance import ExecuteDance_Result
            assert \
                isinstance(value, ExecuteDance_Result), \
                "The 'result' field must be a sub message of type 'ExecuteDance_Result'"
        self._result = value


class Metaclass_ExecuteDance_GetResult(type):
    """Metaclass of service 'ExecuteDance_GetResult'."""

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
                'spot_msgs.action.ExecuteDance_GetResult')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._TYPE_SUPPORT = module.type_support_srv__action__execute_dance__get_result

            from spot_msgs.action import _execute_dance
            if _execute_dance.Metaclass_ExecuteDance_GetResult_Request._TYPE_SUPPORT is None:
                _execute_dance.Metaclass_ExecuteDance_GetResult_Request.__import_type_support__()
            if _execute_dance.Metaclass_ExecuteDance_GetResult_Response._TYPE_SUPPORT is None:
                _execute_dance.Metaclass_ExecuteDance_GetResult_Response.__import_type_support__()


class ExecuteDance_GetResult(metaclass=Metaclass_ExecuteDance_GetResult):
    from spot_msgs.action._execute_dance import ExecuteDance_GetResult_Request as Request
    from spot_msgs.action._execute_dance import ExecuteDance_GetResult_Response as Response

    def __init__(self):
        raise NotImplementedError('Service classes can not be instantiated')


# Import statements for member types

# already imported above
# import builtins

# already imported above
# import rosidl_parser.definition


class Metaclass_ExecuteDance_FeedbackMessage(type):
    """Metaclass of message 'ExecuteDance_FeedbackMessage'."""

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
                'spot_msgs.action.ExecuteDance_FeedbackMessage')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__action__execute_dance__feedback_message
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__action__execute_dance__feedback_message
            cls._CONVERT_TO_PY = module.convert_to_py_msg__action__execute_dance__feedback_message
            cls._TYPE_SUPPORT = module.type_support_msg__action__execute_dance__feedback_message
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__action__execute_dance__feedback_message

            from spot_msgs.action import ExecuteDance
            if ExecuteDance.Feedback.__class__._TYPE_SUPPORT is None:
                ExecuteDance.Feedback.__class__.__import_type_support__()

            from unique_identifier_msgs.msg import UUID
            if UUID.__class__._TYPE_SUPPORT is None:
                UUID.__class__.__import_type_support__()

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class ExecuteDance_FeedbackMessage(metaclass=Metaclass_ExecuteDance_FeedbackMessage):
    """Message class 'ExecuteDance_FeedbackMessage'."""

    __slots__ = [
        '_goal_id',
        '_feedback',
    ]

    _fields_and_field_types = {
        'goal_id': 'unique_identifier_msgs/UUID',
        'feedback': 'spot_msgs/ExecuteDance_Feedback',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.NamespacedType(['unique_identifier_msgs', 'msg'], 'UUID'),  # noqa: E501
        rosidl_parser.definition.NamespacedType(['spot_msgs', 'action'], 'ExecuteDance_Feedback'),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        from unique_identifier_msgs.msg import UUID
        self.goal_id = kwargs.get('goal_id', UUID())
        from spot_msgs.action._execute_dance import ExecuteDance_Feedback
        self.feedback = kwargs.get('feedback', ExecuteDance_Feedback())

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
        if self.goal_id != other.goal_id:
            return False
        if self.feedback != other.feedback:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def goal_id(self):
        """Message field 'goal_id'."""
        return self._goal_id

    @goal_id.setter
    def goal_id(self, value):
        if __debug__:
            from unique_identifier_msgs.msg import UUID
            assert \
                isinstance(value, UUID), \
                "The 'goal_id' field must be a sub message of type 'UUID'"
        self._goal_id = value

    @builtins.property
    def feedback(self):
        """Message field 'feedback'."""
        return self._feedback

    @feedback.setter
    def feedback(self, value):
        if __debug__:
            from spot_msgs.action._execute_dance import ExecuteDance_Feedback
            assert \
                isinstance(value, ExecuteDance_Feedback), \
                "The 'feedback' field must be a sub message of type 'ExecuteDance_Feedback'"
        self._feedback = value


class Metaclass_ExecuteDance(type):
    """Metaclass of action 'ExecuteDance'."""

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
                'spot_msgs.action.ExecuteDance')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._TYPE_SUPPORT = module.type_support_action__action__execute_dance

            from action_msgs.msg import _goal_status_array
            if _goal_status_array.Metaclass_GoalStatusArray._TYPE_SUPPORT is None:
                _goal_status_array.Metaclass_GoalStatusArray.__import_type_support__()
            from action_msgs.srv import _cancel_goal
            if _cancel_goal.Metaclass_CancelGoal._TYPE_SUPPORT is None:
                _cancel_goal.Metaclass_CancelGoal.__import_type_support__()

            from spot_msgs.action import _execute_dance
            if _execute_dance.Metaclass_ExecuteDance_SendGoal._TYPE_SUPPORT is None:
                _execute_dance.Metaclass_ExecuteDance_SendGoal.__import_type_support__()
            if _execute_dance.Metaclass_ExecuteDance_GetResult._TYPE_SUPPORT is None:
                _execute_dance.Metaclass_ExecuteDance_GetResult.__import_type_support__()
            if _execute_dance.Metaclass_ExecuteDance_FeedbackMessage._TYPE_SUPPORT is None:
                _execute_dance.Metaclass_ExecuteDance_FeedbackMessage.__import_type_support__()


class ExecuteDance(metaclass=Metaclass_ExecuteDance):

    # The goal message defined in the action definition.
    from spot_msgs.action._execute_dance import ExecuteDance_Goal as Goal
    # The result message defined in the action definition.
    from spot_msgs.action._execute_dance import ExecuteDance_Result as Result
    # The feedback message defined in the action definition.
    from spot_msgs.action._execute_dance import ExecuteDance_Feedback as Feedback

    class Impl:

        # The send_goal service using a wrapped version of the goal message as a request.
        from spot_msgs.action._execute_dance import ExecuteDance_SendGoal as SendGoalService
        # The get_result service using a wrapped version of the result message as a response.
        from spot_msgs.action._execute_dance import ExecuteDance_GetResult as GetResultService
        # The feedback message with generic fields which wraps the feedback message.
        from spot_msgs.action._execute_dance import ExecuteDance_FeedbackMessage as FeedbackMessage

        # The generic service to cancel a goal.
        from action_msgs.srv._cancel_goal import CancelGoal as CancelGoalService
        # The generic message for get the status of a goal.
        from action_msgs.msg._goal_status_array import GoalStatusArray as GoalStatusMessage

    def __init__(self):
        raise NotImplementedError('Action classes can not be instantiated')
