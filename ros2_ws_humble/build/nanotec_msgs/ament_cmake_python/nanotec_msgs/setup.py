from setuptools import find_packages
from setuptools import setup

setup(
    name='nanotec_msgs',
    version='0.0.1',
    packages=find_packages(
        include=('nanotec_msgs', 'nanotec_msgs.*')),
)
