from setuptools import find_packages
from setuptools import setup

setup(
    name='ros_babel_fish_test_msgs',
    version='1.25.2',
    packages=find_packages(
        include=('ros_babel_fish_test_msgs', 'ros_babel_fish_test_msgs.*')),
)
