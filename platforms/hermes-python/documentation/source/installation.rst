Installation
============

The library is packaged as a pre-compiled platform wheel, available on `PyPi <https://pypi.org/project/hermes-python/>`_.

It can be installed with :
``pip install hermes-python``.

Or you can add it to your `requirements.txt` file.

Building from source
====================

If you want to use ``hermes-python`` on platforms that are not supported, you have to manually compile the wheel.

You need to have `rust` installed :

``curl https://sh.rustup.rs -sSf``

Clone, the ``hermes-protocol`` repository : ::

    git clone git@github.com:snipsco/hermes-protocol.git
    cd hermes-protocol

You need to compile the dynamically linked shared object library : ::

    mkdir -p platforms/hermes-python/target
    CARGO_TARGET_DIR=platforms/hermes-python/target cargo rustc --lib --manifest-path hermes-mqtt-ffi/Cargo.toml --release -- --crate-type cdylib
    mv platforms/hermes-python/target/release/libhermes_mqtt_ffi.dylib platforms/hermes-python/hermes_python/dylib/


You can then build the wheel : ::

    virtualenv env
    source env/bin/activate
    python setup.py bdist_wheel

The built wheels should be in ``platforms/hermes-python/dist``

You can install those with pip : ``pip install platforms/hermes-python/dist/<your_wheel>.whl``

