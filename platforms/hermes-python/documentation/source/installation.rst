Installation
============

The library is packaged as a pre-compiled platform wheel, available on `PyPi <https://pypi.org/project/hermes-python/>`_.

It can be installed with :
``pip install hermes-python``.

Or you can add it to your `requirements.txt` file.

Building from source
====================

If you want to use ``hermes-python`` on platforms that are not supported, you have to manually compile the wheel.

You need to have ``rust`` and ``cargo`` installed :

``curl https://sh.rustup.rs -sSf``

Clone, the ``hermes-protocol`` repository : ::

    git clone git@github.com:snipsco/hermes-protocol.git
    cd hermes-protocol/platforms/hermes-python

You can then build the wheel : ::

    virtualenv env
    source env/bin/activate
    python setup.py bdist_wheel

The built wheels should be in ``platforms/hermes-python/dist``

You can install those with pip : ``pip install platforms/hermes-python/dist/<your_wheel>.whl``

Advanced wheel building
=======================

We define a new API for including pre-compiled shared objects when building a platform wheel. ::

    python setup.py bdist_wheel

This command will compile the ``hermes-mqtt-ffi`` Rust extension, copy them to an appropriate location, and include them in the wheel.

We introduce a new command-line argument : ``include-extension`` which is a way to include an already compiled (in previous steps) ``hermes-mqtt-ffi`` extension in the wheel.

Its usage is the following : ``include-extension=<default | the/path/to/your/extension.[so|dylib]>``

For instance : ::

    python setup.py bdist_wheel --include-extension=default

The default value for ``include-extension`` will look up for pre-compiled extension in the default paths (in ``hermes-protocol/target/release/libhermes_mqtt_ffi.[dylib|so]`` and ``hermes-protocol/platforms/hermes-python/hermes_python/dylib``). ::

    python setup.py bdist_wheel --include-extension=<the/path/to/your/extension.[so|dylib]>

When doing x-compilation, you can also specify the target platform : ::

    python setup.py bdist_wheel --include-extension=<the/path/to/your/extension.[so|dylib]> --plat-name=<the_platform_tag>


