# hermes-protocol

This repository contains the definition of the hermes protocol used by
the Snips platform

- `hermes` ontology and facades (ie protocol) definitions
- `hermes-ffi` ffi bindings for ontology and facade
- `hermes-ffi-test` echo lib that can be used to test guest language bindings
- `hermes-inprocess` protocol implementation using an in-process bus (ripb)
- `hermes-mqtt` protocol implementation using MQTT for communication
- `hermes-mqtt-ffi` lib exposing the MQTT impl to guest languages
- `hermes-test-suite` test suite used to verify implementation correctness
- `platforms` guest language bindings
    - `c` C header
    - `hermes-kotlin` jvm impl

