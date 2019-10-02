API Reference
=============

The Hermes Client
-----------------

.. module:: hermes_python.hermes

.. autoclass:: Hermes
   :members:


Ontology
--------

Session
^^^^^^^

.. module:: hermes_python.ontology.dialogue.session

.. autoclass:: StartSessionMessage
    :members: __init__

.. autoclass:: EndSessionMessage
    :members: __init__

.. autoclass:: ContinueSessionMessage
    :members: __init__

.. autoclass:: SessionStartedMessage
    :members: __init__

.. autoclass:: SessionEndedMessage
    :members: __init__

.. autoclass:: SessionQueuedMessage
    :members: __init__

.. autoclass:: SessionInitAction
    :show-inheritance:
    :members: __init__

.. autoclass:: SessionInitNotification
    :show-inheritance:
    :members: __init__

.. autoclass:: SessionInit



Intent
^^^^^^

.. module:: hermes_python.ontology.dialogue.intent

.. autoclass:: IntentMessage
    :members: __init__

.. autoclass:: IntentClassifierResult
    :members: __init__

.. autoclass:: IntentNotRecognizedMessage
    :members: __init__

    :members:

.. module:: hermes_python.ontology.nlu

.. autoclass:: NluIntentAlternative
    :members: __init__


Slots
^^^^^

.. autoclass:: NluSlot
    :members: __init__

.. autoclass:: SlotMap
    :members: __init__

.. autoclass:: SlotsList
    :members:

.. autoclass:: NluSlot
    :members:


Slot Values
^^^^^^^^^^^

.. module:: hermes_python.ontology.slot

.. autoclass:: SlotValue
    :members: __init__

.. autoclass:: CustomValue
    :members: __init__

.. autoclass:: NumberValue
    :members: __init__

.. autoclass:: OrdinalValue
    :members: __init__

.. autoclass:: AmountOfMoneyValue
    :members: __init__

.. autoclass:: TemperatureValue
    :members: __init__

.. autoclass:: InstantTimeValue
    :members: __init__

.. autoclass:: TimeIntervalValue
    :members: __init__

.. autoclass:: DurationValue
    :members: __init__

.. autoclass:: PercentageValue
    :members: __init__

.. autoclass:: MusicArtistValue
    :members: __init__

.. autoclass:: MusicAlbumValue
    :members: __init__

.. autoclass:: MusicTrackValue
    :members: __init__

.. autoclass:: CityValue
    :members: __init__

.. autoclass:: CountryValue
    :members: __init__

.. autoclass:: RegionValue
    :members: __init__

Injection
^^^^^^^^^

.. module:: hermes_python.ontology.injection

.. autoclass:: InjectionRequestMessage
    :members: __init__

.. autoclass:: InjectionCompleteMessage
    :members: __init__

.. autoclass:: InjectionResetRequestMessage
    :members: __init__

.. autoclass:: InjectionResetCompleteMessage
    :members: __init__

.. autoclass:: InjectionRequestOperation
    :members: __init__

.. autoclass:: AddInjectionRequest
    :show-inheritance:
    :members: __init__

.. autoclass:: AddFromVanillaInjectionRequest
    :show-inheritance:
    :members: __init__


