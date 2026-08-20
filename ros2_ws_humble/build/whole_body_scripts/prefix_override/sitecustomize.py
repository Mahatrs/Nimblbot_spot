import sys
if sys.prefix == '/usr':
    sys.real_prefix = sys.prefix
    sys.prefix = sys.exec_prefix = '/home/mtouri/ros2_ws_humble/install/whole_body_scripts'
