from setuptools import find_packages
from setuptools import setup

setup(
    name='rviz_manager_msgs',
    version='0.0.1',
    packages=find_packages(
        include=('rviz_manager_msgs', 'rviz_manager_msgs.*')),
)
