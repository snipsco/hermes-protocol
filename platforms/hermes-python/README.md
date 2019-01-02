# Hermes Python 
[![Build Status](https://travis-ci.org/snipsco/hermes-protocol.svg)](https://travis-ci.org/snipsco/hermes-protocol)
[![PyPI version](https://badge.fury.io/py/hermes-python.svg)](https://badge.fury.io/py/hermes-python)

The `hermes-python` library provides python bindings for the Hermes protocol that snips components use to communicate together.
`hermes-python` allows you to interface seamlessly with the Snips platform and quick start development of Voice applications. 

`hermes-python` abstracts away the connection to the MQTT bus and the parsing of incoming and outcoming messages from and to the components of the snips platform. 

## Installation 
The library is packaged as a pre-compiled platform wheel, available on [PyPi](https://pypi.org/project/hermes-python/).
It can be installed with : 
```
pip install hermes-python
```

Or you can add it to your `requirements.txt` file. 

### Requirements 
The wheel is available for Python 2.7+ and Python 3.5

The wheel supports the following platform tags : 
- `manylinux1_x86_64`
- `armv7l`, `armv6`
- `macos`

## Usage 

The lifecycle of a script using `hermes-python` has the following steps : 
- Initiating a connection to the MQTT broker
- Registering callback functions to handle incoming intent parsed by the snips platform
- Listening to incoming intents
- Closing the connection  

Let's quickly dive into an example : Let's write an app for a Weather Assistant ! 
This code implies that you created a weather assistant using the [snips Console](https://console.snips.ai), and that it has a *`searchWeatherForecast`* intent. 

Here is a code example for `python2.7` : 

```
from hermes_python.hermes import Hermes

MQTT_ADDR = "localhost:1883"	# Specify host and port for the MQTT broker 

def subscribe_weather_forecast_callback(hermes, intentMessage):	# Defining callback functions to handle an intent that asks for the weather. 
	print("Parsed intent : {}".format(intentMessage.intent.intent_name))


with Hermes(MQTT_ADDR) as h: # Initialization of a connection to the MQTT broker
	h.subscribe_intent("searchWeatherForecast", subscribe_weather_forecast_callback) \  # Registering callback functions to handle the searchWeatherForecast intent
         .start() 

# We get out of the with block, which closes and releases the connection. 

```

### Examples 
#### Handling the `intent_message` object
*Coming soon.*

#### Providing TTS feedback for a voice application 
*Coming soon.*

## Documentation
### Hermes protocol documentation 
If you want to dive deeper into how Snips components communicate together, check out the specification of the `hermes-protocol` [here](https://docs.snips.ai/ressources/hermes-protocol). 
You can also check this repository for other language bindings. 

### API
*Coming soon.*

## Release Checklist 

Everytime you need to perform a release, do the following steps : 
- [ ] Commit all changes to the project for said release
- [ ] Write all the changes introduced in the Changelog (HISTORY.rst file) and commit it
- [ ] Run tests
- [ ] Bump the version and commit it
- [ ] Upload to PyPI
 
