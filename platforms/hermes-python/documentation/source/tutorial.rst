Tutorial
========

The lifecycle of a script using ``hermes-python`` has the following steps :

* Initiating a connection to the MQTT broker
* Registering callback functions to handle incoming intent parsed by the snips platform
* Listening to incoming intents
* Closing the connection

Let's quickly dive into an example :

Let's write an app for a Weather Assistant !
This code implies that you created a weather assistant using the `Snips Console <https://console.snips.ai/>`_, and that it has a ``searchWeatherForecast`` intent.
Or you could download `this weather Assistant <https://resources.snips.ai/assistants/assistant-weather-EN-0.19.0-dyn-heysnipsv4.zip>`_ .

::

    from hermes_python.hermes import Hermes

    MQTT_ADDR = "localhost:1883"	# Specify host and port for the MQTT broker

    def subscribe_weather_forecast_callback(hermes, intent_message):	# Defining callback functions to handle an intent that asks for the weather.
        print("Parsed intent : {}".format(intent_message.intent.intent_name))

    with Hermes(MQTT_ADDR) as h: # Initialization of a connection to the MQTT broker
        h.subscribe_intent("searchWeatherForecast", subscribe_weather_forecast_callback) \  # Registering callback functions to handle the searchWeatherForecast intent
             .start()
        # We get out of the with block, which closes and releases the connection.

This app is a bit limited as it only prints out which intent was detected by our assistant.
Let's add more features.

Handling the ``IntentMessage`` object
-------------------------------------

In the previous example, we registered a callback that had this signature.  ::

    subscribe_intent_callback(hermes, intent_message)

The ``intent_message`` object contains information that was extracted from the spoken sentence.

For instance, in the previous code snippet, we extracted the name of the recognized intent with ::

    intent_message.intent.intent_name

We could also retrieve the associated confidence score the NLU engine had when classifying this intent with ::

    intent_message.intent.confidence_score


Extracting slots
^^^^^^^^^^^^^^^^
Here are some best practices when dealing with slots.
The ``IntentMessage`` object has a ``slots`` attribute.

This ``slots`` attributes is a **container** that is empty when the intent message doesn't have slots : ::

    assert len(intent_message.slots) == 0

This container is a dictionary where the key is the name of the slot, and the value is a list of all the slot values for
this slot name.

You can access these values in two ways : ::

    assert len(intent_message.slots.slot1) == 0
    assert len(intent_message.slots["slot1"]) == 0

The slot values are of type ``NluSlot`` which is a deeply nested object, we offer convenience methods to rapidly access
the `slot_value` attribute of the `NluSlot`.

To access the first ``slot_value`` of a slot called ``myslot``, you can use :
::

    intent_message.slots.myslot.first()

You can also access all the ``slot_value`` of a slot called ``myslot`` :
::

    intent_message.slots.myslot.all()


Let's add to our Weather assistant example.

We assume that the ``searchWeatherForecast`` has one slot called ``forecast_location``,
that indicates which location the user would like to know the weather at.

Let's print all the ``forecast_location`` slots :

::

    for slot in intent_message.slots.forecast_location:
        name = slot.slot_name
        confidence = slot.confidence_score
        print("For slot : {}, the confidence is : {}".format(name, confidence))


The *dot* notation was used, but we can also use the dictionary notation :

::

    for slot in intent_message.slots.forecast_location:
        name = slot["slot_name"]
        print(name)

Some convenience methods are available to easily retrieve slot values :

*Retrieving the first slot value for a given slot name*

::

    slot_value = intent_message.slots.forecast_location.first()


*Retrieving all slot values for a given slot name*

::

    slot_values = intent_message.slots.forecast_location.all()

Coming back to our example, we can now have the app print the ``forecast_location`` slot value back to the user :

::

    def subscribe_weather_forecast_callback(hermes, intent_message):
        slot_value = intent_message.slots.forecast_location.first().value
        print("The slot was : {}".format(slot_value)


Managing sessions
-----------------

The Snips platform includes support for conversations with back and forth communication between the Dialogue Manager and
the client code. Within the Snips platform, a conversation happening between a user and her assistant is called a session.

In this document, we will go through the details of how to start, continue and end a session.

In its default setup, you initiate a conversation with your assistant by pronouncing the defined wake-word.
You say your request out-loud, an intent is extracted from your request, and triggers the portion of the action code you
registered to react to this intent.
Under the hood, the Dialogue Manager starts a new **session** when the wake-word is detected. The session is then ended
by the action code.

Starting a session
^^^^^^^^^^^^^^^^^^

A session can be also be started programmatically. When you initiate a new session, the Dialogue Manager will start the
session by asking the TTS to say the text (if any) and wait for the answer of the end user.

You can start a session in two manners :

* with an action
* with a notification


When initiating a new session with an action, it means the action code will expect a response from the end user.

For instance: You could have an assistant that books concerts tickets for you. The action code would start a session
with an action, having the assistant asking for what band you would like to see live.

When initiating a new session with a notification, it means the action code only inform the user of something without
expecting a response.

For instance: Instead of pronouncing your defined wake-word, you could program a button to initiate a new session.

Let's build up on our previous example of an assistant that book concerts tickets for you.
Here, we are going to initiate a new session with an **action**, filtering on the intent the end-user can respond with.

::

    from hermes_python.hermes import Hermes, MqttOptions

    with Hermes(mqtt_options=MqttOptions()) as h:
        h.publish_start_session_action(None,
            "What band would you like to see live ?",
            ["findLiveBands"],
            True, False, None)


Let's say that we added a physical button to initiate a conversation with our concert tickets booking assistant.
We could use this button to initiate a new session and start talking immediately after pressing the button instead of
relying on triggering a wake-word.

When the button is pressed, the following code could be ran :
::

    hermes.publish_start_session_notification("office", None, None)


This would initiate a new session on the ``office`` site id.


Ending a session
^^^^^^^^^^^^^^^^

To put an end to the current interaction the action code can terminate a started session. You can optionally terminate a
session with a session with a message that should be said out loud by the TTS.

Let's get back to our concert tickets booking assistant, we would end a session like this :

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
^^^^^^^^^^^^^^^^^^^^

You can programmatically extend the lifespan of a dialogue session, expecting interactions from the end users.
The typical use of continuing a session is for your assistant to ask additional information to the end user.

Let's continue with our concert tickets booking assistant, after starting a session, we will continue a session,
expecting the user to tell us how many tickets the assistant should buy.

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
^^^^^^^^^^^^

You can programmatically continue a session, and asking for a specific slot.
If we build on our previous example, we could continue a dialog session by specifying which slot the assistant expects
from the end-user.

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


Dynamic Vocabulary using Entities Injection
-------------------------------------------

Please refer to the `official documentation <https://docs.snips.ai/articles/platform/nlu/dynamic-vocabulary>`_ for further information.

Sometimes, you want to extend your voice assistant with new vocabulary it hasn't seen when it was trained.
For instance, let's say that you have a bookstore voice assistant, that you update every week with new book titles that came out.

The snips platform comes with the **Entities Injection** feature, which allows you to update both the ASR and the NLU models
directly on the device to understand new vocabulary.

Each intent within an assistant may contain some slots, and each slot has a specific type that we call an entity.
If you have a book_title entity that contains a list of book titles in the inventory of your book store,
Entities Injection lets you add new titles to this list.

To inject new entity values, you have multiple operations at your disposal :

* ``add`` adds the list of values that you provide to the existing entity values.
* ``addFromVanilla`` removes all the previously injected values to the entity, and then, adds the list of values provided. Note that the entity values coming from the console will be kept.

Let's see how an injection would be performed by the action code :

::

    from hermes_python.hermes import Hermes
    from hermes_python.ontology.injection import InjectionRequestMessage, AddInjectionRequest, AddFromVanillaInjectionRequest

    def retrieve_new_book_releases():
        return ["The Half-Blood Prince", "The Deathly Hallows"]


    def retrieve_book_inventory():
        return ["The Philosopher's Stone", "The Chamber of Secrets", "The Prisoner of Azkaban", "The Goblet of Fire",
                "The Order of the Phoenix", "The Half-Blood Prince", "The Deathly Hallows"]


    # First example : We just add weekly releases

    operations =  [
        AddInjectionRequest({"book_titles" : retrieve_new_book_releases() }),
    ]

    request1 = InjectionRequestMessage(operations)

    with Hermes("localhost:1883") as h:
        h.request_injection(request1)


    # Second example : We reset all the previously injected values of the book_title entity, and then, adds the list of values provided

    operations =  [
        AddInjectionRequest({"book_titles" : retrieve_book_inventory() }),
    ]

    request2 = InjectionRequestMessage(operations)

    with Hermes("localhost:1883") as h:
        h.request_injection(request2)



**Careful**, performing an entity injection is a CPU and memory intensive task. You should not trigger multiple injection
tasks at the same time on devices with limited computing power.

You can register a callback so that your code knows when an injection process is completed :

::

    def injection_completed(hermes, injection_complete_message):
        print("The injection operation with id {} completed !".format(injection_complete_message.request_id))

    with Hermes("localhost:1883") as h:
        h.subscribe_injection_complete(injection_completed).request_injection(injection_request)


You can monitor the progress of your injection request with ``snips-watch -vvv``.

You can also reset the injected vocabulary of your assistant to its factory settings using the ``request_injection_reset` method of ``hermes``.
Since the operation of resetting the injection is asynchronous, you can register a callback to know when the injection reset process is completed :

::

    def injection_reset_completed(hermes, injection_reset_complete_message):
        print("The injection reset operation with id {} completed !".format(injection_reset_complete_message.request_id))

    with Hermes("localhost:1883") as h:
        h.subscribe_injection_reset_complete(injection_reset_completed).request_injection_reset(request)



Configuring MQTT options
------------------------

The connection to your MQTT broker can be configured with the ``hermes_python.ffi.utils.MqttOptions`` class.

The ``Hermes`` client uses the options specified in the ``MqttOptions`` class when establishing the connection to the MQTT broker.

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

* ``broker_address``: The address of the MQTT broker. It should be formatted as ``ip:port``.
* ``username``: Username to use on the broker. Nullable
* ``password``: Password to use on the broker. Nullable
* ``tls_hostname``: Hostname to use for the TLS configuration. Nullable, setting a value enables TLS
* ``tls_ca_file``: CA files to use if TLS is enabled. Nullable
* ``tls_ca_path``: CA path to use if TLS is enabled. Nullable
* ``tls_client_key``: Client key to use if TLS is enabled. Nullable
* ``tls_client_cert``: Client cert to use if TLS is enabled. Nullable
* ``tls_disable_root_store``: Boolean indicating if the root store should be disabled if TLS is enabled.

Let's connect to an external MQTT broker that requires a username and a password :

::

    from hermes_python.hermes import Hermes
    from hermes_python.ffi.utils import MqttOptions

    mqtt_opts = MqttOptions(username="user1", password="password", broker_address="my-mqtt-broker.com:18852")

    def simple_intent_callback(hermes, intent_message):
        print("I received an intent !")

    with Hermes(mqtt_options=mqtt_opts) as h:
        h.subscribe_intents().loop_forever()


Configuring Dialogue
--------------------

``hermes-python`` offers the possibility to configure different aspects of the Dialogue system.

Enabling and disabling intents on the fly
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

It is possible to enable and disable intents of your assistant on the fly.
Once an intent is disabled, it will not be recognized by the NLU.

Note that intents in the intent filters of started or continued session will take precedence over intents that are enabled/disabled in the configuration of the Dialogue.

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
--------------------------

Enabling and disabling sound feedback
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

By default, the snips platform notify the user of different events of its lifecycle with sound.
It emits a sound when the wakeword is detected, or when the NLU engine (natural understanding engine) has successfuly
extracted an intent from a spoken sentence.

``hermes-python`` allows to disable this sound feedback programmatically, by sending a message to the snips platform,
specifying the ``siteId`` where the sound feedback should be disabled.

::

    from hermes_python.hermes import Hermes
    from hermes_python.ontology.feedback import SiteMessage

    with Hermes("localhost:1883") as h:
        h.disable_sound_feedback(SiteMessage("kitchen"))
        h.start()


Making the TTS play custom sounds
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

The snips-platform allows you to register custom sounds which can be played later by the TTS engine.

``hermes-python`` allows you to register sounds on the fly, by specifying a string identifier for the sound, and providing
a ``wav`` file.

For instance, let's say that your assistant tells a bad joke and that you want to play a *ba dum tss* sound at the end
of the punchline.

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


In the TTS string, when you specify the sound you want to play, you need to follow the syntax : ``[[sound:<your_sound_id>]]``

Enabling Debugging
------------------

You can debug ``hermes-python`` if you encounter an issue and get a better stacktrace that you can send us.

To do so, you have to set the ``rust_logs_enabled`` flag to True when you create an instance of the ``Hermes`` class :

::

    from hermes_python.hermes import Hermes

    def callback(hermes, intent_message):
        pass

    with Hermes("localhost:1883", rust_logs_enabled=True) as h:
        h.subscribe_intent("...", callback)
        h.start()

You should then execute your script with the ``RUST_LOG`` environment variable : ``RUST_LOG=TRACE python your_script.py``.

