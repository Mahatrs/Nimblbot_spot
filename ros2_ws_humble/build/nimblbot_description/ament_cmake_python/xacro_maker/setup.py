from setuptools import find_packages
from setuptools import setup

setup(
    name='xacro_maker',
    version='0.1.0',
    packages=find_packages(
        include=('xacro_maker', 'xacro_maker.*')),
)
