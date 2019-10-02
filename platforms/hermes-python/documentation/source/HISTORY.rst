History
==========
0.8.0 (2019-09-10)
------------------
* Adds subscription to injection lifecycle events : subscribe_injection_complete, subscribe_injection_reset_complete
* Adds a component field to the SessionTerminationType class
* Introduces alternatives intent resolutions
* Fixes folder creation issue when building the wheel from sources

0.7.0 (2019-05-14)
------------------
* Introduces Entities Injection API.

0.6.1 (2019-05-10)
------------------
* Introduces `register_sound` API

0.5.2 (2019-05-07)
------------------
* Fixes nullable fields in Dialogue ontology and brings more type annotations

0.5.1 (2019-05-06)
------------------
* introduces new (cli) API to build python wheel that include pre-compiled hermes-mqtt-ffi extension.

0.5.0 (2019-04-19)
-------------------
* Adds APIs to enable and disable sound feedback. 

0.4.1 (2019-03-29)
------------------
* Re-enables debugging of hermes-python with the `rust_logs_enabled` flag
* AmountOfMoneyValue, InstantTimeValue and DurationValue slot values now use Precision and Grain enumerations

0.4.0 (2019-03-20)
------------------
* Adds support to configure the Dialogue Mananger : enabling and disabling intents on the fly.
* Adds slot filling API : You can ask for a specific slot when continuing a session
* adding support for `OrdinalSlot`

0.3.3 (2019-03-06)
------------------
* Fixes a bug with `publish_start_session_notification` that didn't take the `text` parameter into account.

0.3.2 (2019-02-25)
------------------
* Fixes an important bug that gave the argument `hermes` the wrong type for every registered callback. 
* Fixes an important bug that caused the program to crash when parsing intentMessage that had no slots. 

0.3.1 (2019-02-25)
------------------
* Fixes import bug with templates, the `hermes_python.ffi.utils` module now re-exports `MqttOptions`

0.3.0 (2019-02-25)
------------------
* `IntentClassifierResult`'s `probability` field has been renamed to `confidence_score`.
* Introduces support for snips-platform `1.1.0 - 0.61.1`.

0.2.0 (2019-02-04)
------------------
* Introduces options to connect to the MQTT broker (auth + TLS are now supported).

0.1.29 (2019-01-29)
-------------------
* Fixes bug when deserializing `TimeIntervalValue` that used wrong `encode` method instead of `decode`.

0.1.28 (2019-01-14)
-------------------
* Fixes bug when the `__exit__` method was called twice on the `Hermes` class.
* Introduces two methods to the public api : `connect` and `disconnect` that should bring more flexibility

0.1.27 (2019-01-07)
-------------------
* Fixed broken API introduced in `0.1.26` with the publish_continue_session method of the Hermes class. 
* Cast any string that goes in the mqtt_server_adress parameter in the constructor of the Hermes class to be a 8-bit string.

0.1.26 (2019-01-02)
---------------------
* LICENSING : This wheel now has the same licenses as the parent project : APACHE-MIT. 
* Subscription to not recognized intent messages is added to the API. You can now write your own callbacks to handle unrecognized intents.  
* Adds send_intent_not_recognized flag to continue session : indicate whether the dialogue manager should handle non recognized intents by itself or sent them as an `IntentNotRecognizedMessage` for the client to handle.

0.1.25 (2018-12-13)
---------------------
* Better error handling : Errors from wrapped C library throw a LibException with detailled errors. 


