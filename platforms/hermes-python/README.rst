Hermes Python
*************

.. image:: https://travis-ci.org/snipsco/hermes-protocol.svg
   :target: https://travis-ci.org/snipsco/hermes-protocol

.. image:: https://badge.fury.io/py/hermes-python.svg
   :target: https://badge.fury.io/py/hermes-python


About
*****

The ``hermes-python`` library provides python bindings for the Hermes
protocol that snips components use to communicate together over MQTT.
``hermes-python`` allows you to interface seamlessly with the Snips
platform and kickstart development of Voice applications.

``hermes-python`` abstracts away the connection to the MQTT bus and
the parsing of incoming and outcoming messages from and to the
components of the snips platform.


Requirements
************

Pre-compiled wheels are available for Python 2.7+ and Python 3.5

The pre-compiled wheels supports the following platform tags :

* ``manylinux1_x86_64``

* ``armv7l``, ``armv6``

* ``macos``

If you want to install ``hermes-python`` on another platform, you have
to build it from source.


Installation
************

The library is packaged as a pre-compiled platform wheel, available on
`PyPi <https://pypi.org/project/hermes-python/>`_.

It can be installed with : ``pip install hermes-python``.

Or you can add it to your *requirements.txt* file.


Building from source
********************

If you want to use ``hermes-python`` on platforms that are not
supported, you have to manually compile the wheel.

You need to have ``rust`` and ``cargo`` installed :

``curl https://sh.rustup.rs -sSf``

Clone, the ``hermes-protocol`` repository :

::

   git clone git@github.com:snipsco/hermes-protocol.git
   cd hermes-protocol

You can then build the wheel :

::

   virtualenv env
   source env/bin/activate
   python setup.py bdist_wheel

The built wheels should be in ``platforms/hermes-python/dist``

You can install those with pip : ``pip install
platforms/hermes-python/dist/<your_wheel>.whl``


Advanced wheel building
***********************

We define a new API for including pre-compiled shared objects when
building a platform wheel.

::

   python setup.py bdist_wheel

This command will compile the ``hermes-mqtt-ffi`` Rust extension, copy
them to an appropriate location, and include them in the wheel.

We introduce a new command-line argument : ``include-extension`` which
is a way to include an already compiled (in previous steps)
``hermes-mqtt-ffi`` extension in the wheel.

Its usage is the following : ``include-extension=<default |
the/path/to/your/extension.[so|dylib]>``

For instance :

::

   python setup.py bdist_wheel --include-extension=default

The default value for ``include-extension`` will look up for
pre-compiled extension in the default paths (in
``hermes-protocol/target/release/libhermes_mqtt_ffi.[dylib|so]`` and
``hermes-protocol/platforms/hermes-python/hermes_python/dylib``).

::

   python setup.py bdist_wheel --include-extension=<the/path/to/your/extension.[so|dylib]>

When doing x-compilation, you can also specify the target platform :

::

   python setup.py bdist_wheel --include-extension=<the/path/to/your/extension.[so|dylib]> --plat-name=<the_platform_tag>


Tutorial
********

The lifecycle of a script using ``hermes-python`` has the following
steps :

* Initiating a connection to the MQTT broker

* Registering callback functions to handle incoming intent parsed by
   the snips platform

* Listening to incoming intents

* Closing the connection

Let’s quickly dive into an example :

Let’s write an app for a Weather Assistant ! This code implies that
you created a weather assistant using the `Snips Console
<https://console.snips.ai/>`_, and that it has a
``searchWeatherForecast`` intent. Or you could download `this weather
Assistant
<https://resources.snips.ai/assistants/assistant-weather-EN-0.19.0-dyn-heysnipsv4.zip>`_
.

::

   from hermes_python.hermes import Hermes

   MQTT_ADDR = "localhost:1883"        # Specify host and port for the MQTT broker

   def subscribe_weather_forecast_callback(hermes, intent_message):    # Defining callback functions to handle an intent that asks for the weather.
       print("Parsed intent : {}".format(intent_message.intent.intent_name))

   with Hermes(MQTT_ADDR) as h: # Initialization of a connection to the MQTT broker
       h.subscribe_intent("searchWeatherForecast", subscribe_weather_forecast_callback) \  # Registering callback functions to handle the searchWeatherForecast intent
            .start()
       # We get out of the with block, which closes and releases the connection.

This app is a bit limited as it only prints out which intent was
detected by our assistant. Let’s add more features.


Handling the ``IntentMessage`` object
=====================================

In the previous example, we registered a callback that had this
signature.

::

   subscribe_intent_callback(hermes, intent_message)

The ``intent_message`` object contains information that was extracted
from the spoken sentence.

For instance, in the previous code snippet, we extracted the name of
the recognized intent with

::

   intent_message.intent.intent_name

We could also retrieve the associated confidence score the NLU engine
had when classifying this intent with

::

   intent_message.intent.confidence_score


Extracting slots
----------------

Here are some best practices when dealing with slots. The
``IntentMessage`` object has a ``slots`` attribute.

This ``slots`` attributes is a **container** that is empty when the
intent message doesn’t have slots :

::

   assert len(intent_message.slots) == 0

This container is a dictionary where the key is the name of the slot,
and the value is a list of all the slot values for this slot name.

You can access these values in two ways :

::

   assert len(intent_message.slots.slot1) == 0
   assert len(intent_message.slots["slot1"]) == 0

The slot values are of type ``NluSlot`` which is a deeply nested
object, we offer convenience methods to rapidly access the
*slot_value* attribute of the *NluSlot*.

To access the first ``slot_value`` of a slot called ``myslot``, you
can use :

::

   intent_message.slots.myslot.first()

You can also access all the ``slot_value`` of a slot called ``myslot``
:

::

   intent_message.slots.myslot.all()

Let’s add to our Weather assistant example.

We assume that the ``searchWeatherForecast`` has one slot called
``forecast_location``, that indicates which location the user would
like to know the weather at.

Let’s print all the ``forecast_location`` slots :

::

   for slot in intent_message.slots.forecast_location:
       name = slot.slot_name
       confidence = slot.confidence_score
       print("For slot : {}, the confidence is : {}".format(name, confidence))

The *dot* notation was used, but we can also use the dictionary
notation :

::

   for slot in intent_message.slots.forecast_location:
       name = slot["slot_name"]
       print(name)

Some convenience methods are available to easily retrieve slot values
:

*Retrieving the first slot value for a given slot name*

::

   slot_value = intent_message.slots.forecast_location.first()

*Retrieving all slot values for a given slot name*

::

   slot_values = intent_message.slots.forecast_location.all()

Coming back to our example, we can now have the app print the
``forecast_location`` slot value back to the user :

::

   def subscribe_weather_forecast_callback(hermes, intent_message):
       slot_value = intent_message.slots.forecast_location.first().value
       print("The slot was : {}".format(slot_value)


Managing sessions
=================

The Snips platform includes support for conversations with back and
forth communication between the Dialogue Manager and the client code.
Within the Snips platform, a conversation happening between a user and
her assistant is called a session.

In this document, we will go through the details of how to start,
continue and end a session.

In its default setup, you initiate a conversation with your assistant
by pronouncing the defined wake-word. You say your request out-loud,
an intent is extracted from your request, and triggers the portion of
the action code you registered to react to this intent. Under the
hood, the Dialogue Manager starts a new **session** when the wake-word
is detected. The session is then ended by the action code.


Starting a session
------------------

A session can be also be started programmatically. When you initiate a
new session, the Dialogue Manager will start the session by asking the
TTS to say the text (if any) and wait for the answer of the end user.

You can start a session in two manners :

* with an action

* with a notification

When initiating a new session with an action, it means the action code
will expect a response from the end user.

For instance: You could have an assistant that books concerts tickets
for you. The action code would start a session with an action, having
the assistant asking for what band you would like to see live.

When initiating a new session with a notification, it means the action
code only inform the user of something without expecting a response.

For instance: Instead of pronouncing your defined wake-word, you could
program a button to initiate a new session.

Let’s build up on our previous example of an assistant that book
concerts tickets for you. Here, we are going to initiate a new session
with an **action**, filtering on the intent the end-user can respond
with.

::

   from hermes_python.hermes import Hermes, MqttOptions

   with Hermes(mqtt_options=MqttOptions()) as h:
       h.publish_start_session_action(None,
           "What band would you like to see live ?",
           ["findLiveBands"],
           True, False, None)

Let’s say that we added a physical button to initiate a conversation
with our concert tickets booking assistant. We could use this button
to initiate a new session and start talking immediately after pressing
the button instead of relying on triggering a wake-word.

When the button is pressed, the following code could be ran :

::

   hermes.publish_start_session_notification("office", None, None)

This would initiate a new session on the ``office`` site id.


Ending a session
----------------

To put an end to the current interaction the action code can terminate
a started session. You can optionally terminate a session with a
session with a message that should be said out loud by the TTS.

Let’s get back to our concert tickets booking assistant, we would end
a session like this :

::

   from hermes_python.hermes import Hermes, MqttOptions


   def find_shows(band):
       pass


   def findLiveBandHandler(hermes, intent_message):
       band = intent_message.slots.band.first().value
       shows = find_shows(band)
       hermes.publish_end_session(intent_message.session_id, "I found {} shows for this band !".format(len(shows)))


   with Hermes(mqtt_options=MqttOptions()) as h:
       h\
           .subscribe_intent("findLiveBand", findLiveBandHandler)\
           .start()


Continuing a session
--------------------

You can programmatically extend the lifespan of a dialogue session,
expecting interactions from the end users. The typical use of
continuing a session is for your assistant to ask additional
information to the end user.

Let’s continue with our concert tickets booking assistant, after
starting a session, we will continue a session, expecting the user to
tell us how many tickets the assistant should buy.

::

   import json
   from hermes_python.hermes import Hermes, MqttOptions

   required_slots = {  # We are expecting these slots.
       "band": None,
       "number_of_tickets": None
   }

   def ticketShoppingHandler(hermes, intent_message):
       available_slots = json.loads(intent_message.custom_data)

       band_slot = intent_message.slots.band.first().value or available_slots["band"]
       number_of_tickets = intent_message.slots.number_of_tickets.first().value or available_slots["number_of_tickets"]

       available_slots["band"] = band_slot
       available_slots["number_of_tickets"] = number_of_tickets

       if not band_slot:
           return hermes.publish_continue_session(intent_message.session_id,
                                                  "What band would you like to see live ?",
                                                  ["ticketShopping"],
                                                  custom_data=json.dumps(available_slots))

       if not number_of_tickets:
           return hermes.publish_continue_session(intent_message.session_id,
                                                  "How many tickets should I buy ?",
                                                  ["ticketShopping"],
                                                  custom_data=json.dumps(available_slots))

       return hermes.publish_end_session(intent_message.session_id, "Ok ! Consider it booked !")


   with Hermes(mqtt_options=MqttOptions("raspi-anthal-support.local")) as h:
       h\
           .subscribe_intent("ticketShopping", ticketShoppingHandler)\
           .start()


Slot filling
------------

You can programmatically continue a session, and asking for a specific
slot. If we build on our previous example, we could continue a dialog
session by specifying which slot the assistant expects from the
end-user.

::

   import json
   from hermes_python.hermes import Hermes, MqttOptions

   required_slots_questions = {
       "band": "What band would you like to see live ?",
       "number_of_tickets": "How many tickets should I buy ?"
   }

   def ticketShoppingHandler(hermes, intent_message):
       available_slots = json.loads(intent_message.custom_data)

       band_slot = intent_message.slots.band.first().value or available_slots["band"]
       number_of_tickets = intent_message.slots.number_of_tickets.first().value or available_slots["number_of_tickets"]

       available_slots["band"] = band_slot
       available_slots["number_of_tickets"] = number_of_tickets

       missing_slots = filter(lambda slot: slot is None, [band_slot, number_of_tickets])

       if len(missing_slots):
           missing_slot = missing_slots.pop()
           return hermes.publish_continue_session(intent_message.session_id,
                                                  required_slots_questions[missing_slot],
                                                  custom_data=json.dumps(available_slots),
                                                  slot_to_fill=missing_slot)
       else:
           return hermes.publish_end_session(intent_message.session_id, "Ok ! Consider it booked !")


   with Hermes(mqtt_options=MqttOptions("raspi-anthal-support.local")) as h:
       h\
           .subscribe_intent("ticketShopping", ticketShoppingHandler)\
           .start()


Configuring MQTT options
========================

The connection to your MQTT broker can be configured with the
``hermes_python.ffi.utils.MqttOptions`` class.

The ``Hermes`` client uses the options specified in the
``MqttOptions`` class when establishing the connection to the MQTT
broker.

Here is a code example :

::

   from hermes_python.hermes import Hermes
   from hermes_python.ffi.utils import MqttOptions

   mqtt_opts = MqttOptions()

   def simple_intent_callback(hermes, intent_message):
       print("I received an intent !")

   with Hermes(mqtt_options=mqtt_opts) as h:
       h.subscribe_intents().loop_forever()

Here are the options you can specify in the MqttOptions class :

* ``broker_address``: The address of the MQTT broker. It should be
   formatted as ``ip:port``.

* ``username``: Username to use on the broker. Nullable

* ``password``: Password to use on the broker. Nullable

* ``tls_hostname``: Hostname to use for the TLS configuration.
   Nullable, setting a value enables TLS

* ``tls_ca_file``: CA files to use if TLS is enabled. Nullable

* ``tls_ca_path``: CA path to use if TLS is enabled. Nullable

* ``tls_client_key``: Client key to use if TLS is enabled. Nullable

* ``tls_client_cert``: Client cert to use if TLS is enabled. Nullable

* ``tls_disable_root_store``: Boolean indicating if the root store
   should be disabled if TLS is enabled.

Let’s connect to an external MQTT broker that requires a username and
a password :

::

   from hermes_python.hermes import Hermes
   from hermes_python.ffi.utils import MqttOptions

   mqtt_opts = MqttOptions(username="user1", password="password", broker_address="my-mqtt-broker.com:18852")

   def simple_intent_callback(hermes, intent_message):
       print("I received an intent !")

   with Hermes(mqtt_options=mqtt_opts) as h:
       h.subscribe_intents().loop_forever()


Configuring Dialogue
====================

``hermes-python`` offers the possibility to configure different
aspects of the Dialogue system.


Enabling and disabling intents on the fly
-----------------------------------------

It is possible to enable and disable intents of your assistant on the
fly. Once an intent is disabled, it will not be recognized by the NLU.

Note that intents in the intent filters of started or continued
session will take precedence over intents that are enabled/disabled in
the configuration of the Dialogue.

You can disable/enable intents with the following methods :

::

   from hermes_python.ontology.dialogue import DialogueConfiguration

   dialogue_conf = DialogueConfiguration()                          \
                           .disable_intent("intent1")               \
                           .enable_intent("intent2")                \
                           .enable_intents(["intent1", "intent2"])  \
                           .disable_intents(["intent2", "intent1"])

   hermes.configure_dialogue(dialogue_conf)


Configuring Sound Feedback
==========================


Enabling and disabling sound feedback
-------------------------------------

By default, the snips platform notify the user of different events of
its lifecycle with sound. It emits a sound when the wakeword is
detected, or when the NLU engine (natural understanding engine) has
successfuly extracted an intent from a spoken sentence.

``hermes-python`` allows to disable this sound feedback
programmatically, by sending a message to the snips platform,
specifying the ``siteId`` where the sound feedback should be disabled.

::

   from hermes_python.hermes import Hermes
   from hermes_python.ontology.feedback import SiteMessage

   with Hermes("localhost:1883") as h:
       h.disable_sound_feedback(SiteMessage("kitchen"))
       h.start()


Making the TTS play custom sounds
---------------------------------

The snips-platform allows you to register custom sounds which can be
played later by the TTS engine.

``hermes-python`` allows you to register sounds on the fly, by
specifying a string identifier for the sound, and providing a ``wav``
file.

For instance, let’s say that your assistant tells a bad joke and that
you want to play a *ba dum tss* sound at the end of the punchline.

::

   from builtins import bytearray
   from hermes_python.hermes import Hermes
   from hermes_python.ontology.tts import RegisterSoundMessage

   # Step 1 : We read a wav file
   def read_wav_data():
       with open('ba_dum_tss.wav', 'rb') as f:
           read_data = f.read()
       return bytearray(read_data)


   # Step 2 : We register a sound that will be named "bad_joke"
   sound = RegisterSoundMessage("bad_joke", read_wav_data())

   def callback(hermes, intent_message):
       hermes.publish_end_session(intent_message.session_id, "A very bad joke ... [[sound:bad_joke]]")  # Step 4 : You play your registered sound

   with Hermes("localhost:1883") as h:
       h.connect()\
           .register_sound(sound)\    # Step 3 : You register your custom sound
           .subscribe_intents(callback)\
           .start()

In the TTS string, when you specify the sound you want to play, you
need to follow the syntax : ``[[sound:<your_sound_id>]]``


Enabling Debugging
==================

You can debug ``hermes-python`` if you encounter an issue and get a
better stacktrace that you can send us.

To do so, you have to set the ``rust_logs_enabled`` flag to True when
you create an instance of the ``Hermes`` class :

::

   from hermes_python.hermes import Hermes

   def callback(hermes, intent_message):
       pass

   with Hermes("localhost:1883", rust_logs_enabled=True) as h:
       h.subscribe_intent("...", callback)
       h.start()

You should then execute your script with the ``RUST_LOG`` environment
variable : ``RUST_LOG=TRACE python your_script.py``.


Release Checklist
*****************

Everytime you need to perform a release, do the following steps : - [
] Commit all changes to the project for said release - [ ] Write all
the changes introduced in the Changelog (source/HISTORY.rst file) and
commit it - [ ] Run tests - [ ] Build the documentation and commit the
README.rst - [ ] Bump the version and commit it - [ ] Upload to PyPI


Build details
*************


Creating macOS wheels
=====================

The build script : ``build_scripts/build_macos_wheels.sh`` uses
``pyenv`` to generate ``hermes-python`` wheels for different versions
of python.

To be able to run it, you need to :

* install pyenv : brew install pyenv. Then follow the additional
   steps detailled

* you then have to install python at different versions:  ``pyenv
   install --list`` to list the available version to install

* Before installing and building the different python version from
   sources, install the required dependencies : `Link here
   <https://github.com/pyenv/pyenv/wiki/>`_

That’s it !
