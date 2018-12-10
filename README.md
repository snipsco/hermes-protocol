# `hermes-protocol`
[![Build Status](https://travis-ci.org/snipsco/hermes-protocol.svg?branch=develop)](https://travis-ci.org/snipsco/hermes-protocol)

This repository contains the definition of the `hermes` protocol used by
the Snips platform.

## What is `hermes`?

`hermes` is a protocol used by all the components of the Snips platform
to communicate with each other. For example an user app for the platform
can use `hermes` to be notified that a new intent was detected. An other
example could when the dialogue component asks the Automatic Speech
Recognition (ASR) component to start capturing user speech, this will also
be done through `hermes`.

### Ontology

The various messages that can be sent using `hermes` are defined in what
we call an ontology. It is an ensemble of rust `struct`s definitions.
Relevant parts of the ontology are available in the guest languages that
hermes supports.

### Sites

An important notion when using the Snips platform is the notion of
"site". A site can be seen as an "interaction locus". It is throught a
site than an user will interact with the platform. A site consists at
least of a voice input (a microphone) and also probably of speakers and
maybe other devices used to give information to the user (leds,
screens...).

A site is identified in `hermes` by a `site_id`. When there is a single
site, it is usually `default`.


### Components and facades

The Snips platform consists of multiple components (for example the ASR,
the dialogue, the TTS, the NLU...) and `hermes` has been designed around
theses components.

Each component is represented in hermes by two "facades":
 - The "main" facade that is intended to by used by the one other
 components to communicate with the component.
 - The "backend" facade that is intended to be used only by the
 component as it represents the communication side that should be
 implemented by it.

Facades are retrieved through a `ProtocolHandler` more on that later.

Let's take the example of the detection of an hotword:
 1. The code that wants to react to an hotword first need to "subscribe"
 to the event:
   - It needs to use the main hotword facade, that can be obtained using
   the `hotword` method on the `ProtocolHandler`
   - It can then register a callback using the `subscribeDetected` or
   `subscribeAllDetected` methods on the facade (the first one provides
   and handy way to filter the hotword detected only on a specific site)
 2. The code that actually detects the hotword (aka `snips-hotword`)
 needs to publish the event when an hotword is detected:
   - It needs to use the backend hotword facade, that can be obtained
   using the `hotwork_backend` method on the `ProtocolHandler`
   - It can then publish the event using the `publishDetected` method on
   the facade
 3. The event is delivered to the callback registered a `1.` and the
 code can react how it wants to the the hotword.

### Dialogue

When writing an app for the Snips platform, the facade you'll want to
use is the main dialogue facade. The dialogue component orchestrates the
interaction with the user and your app should interact with the platform
through it. You can find an overview of the available apis on the [Snips
documentation](https://docs.snips.ai/ressources/messages-reference).

### Communication layer and `ProtocolHandler`s

`hermes` has two implementations: one over MQTT and one using an
inprocess bus. A standard installation on a RaspberryPi uses the MQTT
version whereas the Android and iOs sdks use the inprocess one.

The rust crates `hermes-mqtt` and `hermes-inprocess` provide
`ProtocolHandler`s implementation for the two communication layers.
The guest language bindings for `hermes` wrap `hermes-mqtt`.

## Quick description of the different dirs

- `hermes` ontology and facades (ie protocol) definitions
- `hermes-ffi` ffi bindings for ontology and facades
- `hermes-ffi-test` echo lib that can be used to test guest language
bindings
- `hermes-inprocess` protocol implementation using an in-process bus
(ripb) for communication
- `hermes-mqtt` protocol implementation using MQTT for communication
- `hermes-mqtt-ffi` lib exposing the MQTT impl to guest languages
- `hermes-test-suite` test suite used to verify implementation
correctness
- `platforms` guest language bindings
    - `c` C header
    - `hermes-kotlin` jvm impl
    - `hermes-python` python impl
    - `hermes-javascript` js impl

## License
### Apache 2.0/MIT

Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
http://opensource.org/licenses/MIT)

     at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms
or conditions.
