# generated from rosidl_generator_py/resource/_idl.py.em
# with input from nanotec_msgs:msg/DeviceStatus.idl
# generated code does not contain a copyright notice


# Import statements for member types

import builtins  # noqa: E402, I100

import math  # noqa: E402, I100

import rosidl_parser.definition  # noqa: E402, I100


class Metaclass_DeviceStatus(type):
    """Metaclass of message 'DeviceStatus'."""

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
            module = import_type_support('nanotec_msgs')
        except ImportError:
            import logging
            import traceback
            logger = logging.getLogger(
                'nanotec_msgs.msg.DeviceStatus')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__msg__device_status
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__msg__device_status
            cls._CONVERT_TO_PY = module.convert_to_py_msg__msg__device_status
            cls._TYPE_SUPPORT = module.type_support_msg__msg__device_status
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__msg__device_status

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class DeviceStatus(metaclass=Metaclass_DeviceStatus):
    """Message class 'DeviceStatus'."""

    __slots__ = [
        '_name',
        '_voltage_power',
        '_voltage_logic',
        '_temperature_motor',
        '_temperature_micro_chip',
        '_ready_to_switch_on',
        '_switched_on',
        '_operation_enabled',
        '_fault',
        '_voltage_enabled',
        '_quick_stop',
        '_switch_on_disabled',
        '_warning',
        '_target_reached',
        '_internal_limit_active',
        '_operation_mode_specific',
        '_homing_status',
    ]

    _fields_and_field_types = {
        'name': 'string',
        'voltage_power': 'float',
        'voltage_logic': 'float',
        'temperature_motor': 'float',
        'temperature_micro_chip': 'float',
        'ready_to_switch_on': 'boolean',
        'switched_on': 'boolean',
        'operation_enabled': 'boolean',
        'fault': 'boolean',
        'voltage_enabled': 'boolean',
        'quick_stop': 'boolean',
        'switch_on_disabled': 'boolean',
        'warning': 'boolean',
        'target_reached': 'boolean',
        'internal_limit_active': 'boolean',
        'operation_mode_specific': 'int32',
        'homing_status': 'int32',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.UnboundedString(),  # noqa: E501
        rosidl_parser.definition.BasicType('float'),  # noqa: E501
        rosidl_parser.definition.BasicType('float'),  # noqa: E501
        rosidl_parser.definition.BasicType('float'),  # noqa: E501
        rosidl_parser.definition.BasicType('float'),  # noqa: E501
        rosidl_parser.definition.BasicType('boolean'),  # noqa: E501
        rosidl_parser.definition.BasicType('boolean'),  # noqa: E501
        rosidl_parser.definition.BasicType('boolean'),  # noqa: E501
        rosidl_parser.definition.BasicType('boolean'),  # noqa: E501
        rosidl_parser.definition.BasicType('boolean'),  # noqa: E501
        rosidl_parser.definition.BasicType('boolean'),  # noqa: E501
        rosidl_parser.definition.BasicType('boolean'),  # noqa: E501
        rosidl_parser.definition.BasicType('boolean'),  # noqa: E501
        rosidl_parser.definition.BasicType('boolean'),  # noqa: E501
        rosidl_parser.definition.BasicType('boolean'),  # noqa: E501
        rosidl_parser.definition.BasicType('int32'),  # noqa: E501
        rosidl_parser.definition.BasicType('int32'),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        self.name = kwargs.get('name', str())
        self.voltage_power = kwargs.get('voltage_power', float())
        self.voltage_logic = kwargs.get('voltage_logic', float())
        self.temperature_motor = kwargs.get('temperature_motor', float())
        self.temperature_micro_chip = kwargs.get('temperature_micro_chip', float())
        self.ready_to_switch_on = kwargs.get('ready_to_switch_on', bool())
        self.switched_on = kwargs.get('switched_on', bool())
        self.operation_enabled = kwargs.get('operation_enabled', bool())
        self.fault = kwargs.get('fault', bool())
        self.voltage_enabled = kwargs.get('voltage_enabled', bool())
        self.quick_stop = kwargs.get('quick_stop', bool())
        self.switch_on_disabled = kwargs.get('switch_on_disabled', bool())
        self.warning = kwargs.get('warning', bool())
        self.target_reached = kwargs.get('target_reached', bool())
        self.internal_limit_active = kwargs.get('internal_limit_active', bool())
        self.operation_mode_specific = kwargs.get('operation_mode_specific', int())
        self.homing_status = kwargs.get('homing_status', int())

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
        if self.name != other.name:
            return False
        if self.voltage_power != other.voltage_power:
            return False
        if self.voltage_logic != other.voltage_logic:
            return False
        if self.temperature_motor != other.temperature_motor:
            return False
        if self.temperature_micro_chip != other.temperature_micro_chip:
            return False
        if self.ready_to_switch_on != other.ready_to_switch_on:
            return False
        if self.switched_on != other.switched_on:
            return False
        if self.operation_enabled != other.operation_enabled:
            return False
        if self.fault != other.fault:
            return False
        if self.voltage_enabled != other.voltage_enabled:
            return False
        if self.quick_stop != other.quick_stop:
            return False
        if self.switch_on_disabled != other.switch_on_disabled:
            return False
        if self.warning != other.warning:
            return False
        if self.target_reached != other.target_reached:
            return False
        if self.internal_limit_active != other.internal_limit_active:
            return False
        if self.operation_mode_specific != other.operation_mode_specific:
            return False
        if self.homing_status != other.homing_status:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def name(self):
        """Message field 'name'."""
        return self._name

    @name.setter
    def name(self, value):
        if __debug__:
            assert \
                isinstance(value, str), \
                "The 'name' field must be of type 'str'"
        self._name = value

    @builtins.property
    def voltage_power(self):
        """Message field 'voltage_power'."""
        return self._voltage_power

    @voltage_power.setter
    def voltage_power(self, value):
        if __debug__:
            assert \
                isinstance(value, float), \
                "The 'voltage_power' field must be of type 'float'"
            assert not (value < -3.402823466e+38 or value > 3.402823466e+38) or math.isinf(value), \
                "The 'voltage_power' field must be a float in [-3.402823466e+38, 3.402823466e+38]"
        self._voltage_power = value

    @builtins.property
    def voltage_logic(self):
        """Message field 'voltage_logic'."""
        return self._voltage_logic

    @voltage_logic.setter
    def voltage_logic(self, value):
        if __debug__:
            assert \
                isinstance(value, float), \
                "The 'voltage_logic' field must be of type 'float'"
            assert not (value < -3.402823466e+38 or value > 3.402823466e+38) or math.isinf(value), \
                "The 'voltage_logic' field must be a float in [-3.402823466e+38, 3.402823466e+38]"
        self._voltage_logic = value

    @builtins.property
    def temperature_motor(self):
        """Message field 'temperature_motor'."""
        return self._temperature_motor

    @temperature_motor.setter
    def temperature_motor(self, value):
        if __debug__:
            assert \
                isinstance(value, float), \
                "The 'temperature_motor' field must be of type 'float'"
            assert not (value < -3.402823466e+38 or value > 3.402823466e+38) or math.isinf(value), \
                "The 'temperature_motor' field must be a float in [-3.402823466e+38, 3.402823466e+38]"
        self._temperature_motor = value

    @builtins.property
    def temperature_micro_chip(self):
        """Message field 'temperature_micro_chip'."""
        return self._temperature_micro_chip

    @temperature_micro_chip.setter
    def temperature_micro_chip(self, value):
        if __debug__:
            assert \
                isinstance(value, float), \
                "The 'temperature_micro_chip' field must be of type 'float'"
            assert not (value < -3.402823466e+38 or value > 3.402823466e+38) or math.isinf(value), \
                "The 'temperature_micro_chip' field must be a float in [-3.402823466e+38, 3.402823466e+38]"
        self._temperature_micro_chip = value

    @builtins.property
    def ready_to_switch_on(self):
        """Message field 'ready_to_switch_on'."""
        return self._ready_to_switch_on

    @ready_to_switch_on.setter
    def ready_to_switch_on(self, value):
        if __debug__:
            assert \
                isinstance(value, bool), \
                "The 'ready_to_switch_on' field must be of type 'bool'"
        self._ready_to_switch_on = value

    @builtins.property
    def switched_on(self):
        """Message field 'switched_on'."""
        return self._switched_on

    @switched_on.setter
    def switched_on(self, value):
        if __debug__:
            assert \
                isinstance(value, bool), \
                "The 'switched_on' field must be of type 'bool'"
        self._switched_on = value

    @builtins.property
    def operation_enabled(self):
        """Message field 'operation_enabled'."""
        return self._operation_enabled

    @operation_enabled.setter
    def operation_enabled(self, value):
        if __debug__:
            assert \
                isinstance(value, bool), \
                "The 'operation_enabled' field must be of type 'bool'"
        self._operation_enabled = value

    @builtins.property
    def fault(self):
        """Message field 'fault'."""
        return self._fault

    @fault.setter
    def fault(self, value):
        if __debug__:
            assert \
                isinstance(value, bool), \
                "The 'fault' field must be of type 'bool'"
        self._fault = value

    @builtins.property
    def voltage_enabled(self):
        """Message field 'voltage_enabled'."""
        return self._voltage_enabled

    @voltage_enabled.setter
    def voltage_enabled(self, value):
        if __debug__:
            assert \
                isinstance(value, bool), \
                "The 'voltage_enabled' field must be of type 'bool'"
        self._voltage_enabled = value

    @builtins.property
    def quick_stop(self):
        """Message field 'quick_stop'."""
        return self._quick_stop

    @quick_stop.setter
    def quick_stop(self, value):
        if __debug__:
            assert \
                isinstance(value, bool), \
                "The 'quick_stop' field must be of type 'bool'"
        self._quick_stop = value

    @builtins.property
    def switch_on_disabled(self):
        """Message field 'switch_on_disabled'."""
        return self._switch_on_disabled

    @switch_on_disabled.setter
    def switch_on_disabled(self, value):
        if __debug__:
            assert \
                isinstance(value, bool), \
                "The 'switch_on_disabled' field must be of type 'bool'"
        self._switch_on_disabled = value

    @builtins.property
    def warning(self):
        """Message field 'warning'."""
        return self._warning

    @warning.setter
    def warning(self, value):
        if __debug__:
            assert \
                isinstance(value, bool), \
                "The 'warning' field must be of type 'bool'"
        self._warning = value

    @builtins.property
    def target_reached(self):
        """Message field 'target_reached'."""
        return self._target_reached

    @target_reached.setter
    def target_reached(self, value):
        if __debug__:
            assert \
                isinstance(value, bool), \
                "The 'target_reached' field must be of type 'bool'"
        self._target_reached = value

    @builtins.property
    def internal_limit_active(self):
        """Message field 'internal_limit_active'."""
        return self._internal_limit_active

    @internal_limit_active.setter
    def internal_limit_active(self, value):
        if __debug__:
            assert \
                isinstance(value, bool), \
                "The 'internal_limit_active' field must be of type 'bool'"
        self._internal_limit_active = value

    @builtins.property
    def operation_mode_specific(self):
        """Message field 'operation_mode_specific'."""
        return self._operation_mode_specific

    @operation_mode_specific.setter
    def operation_mode_specific(self, value):
        if __debug__:
            assert \
                isinstance(value, int), \
                "The 'operation_mode_specific' field must be of type 'int'"
            assert value >= -2147483648 and value < 2147483648, \
                "The 'operation_mode_specific' field must be an integer in [-2147483648, 2147483647]"
        self._operation_mode_specific = value

    @builtins.property
    def homing_status(self):
        """Message field 'homing_status'."""
        return self._homing_status

    @homing_status.setter
    def homing_status(self, value):
        if __debug__:
            assert \
                isinstance(value, int), \
                "The 'homing_status' field must be of type 'int'"
            assert value >= -2147483648 and value < 2147483648, \
                "The 'homing_status' field must be an integer in [-2147483648, 2147483647]"
        self._homing_status = value
