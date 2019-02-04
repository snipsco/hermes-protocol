# Hermes Python 
[![Build Status](https://travis-ci.org/snipsco/hermes-protocol.svg)](https://travis-ci.org/snipsco/hermes-protocol)
[![PyPI version](https://badge.fury.io/py/hermes-python.svg)](https://badge.fury.io/py/hermes-python)

The `hermes-python` library provides python bindings for the Hermes protocol that snips components use to communicate together over MQTT.
`hermes-python` allows you to interface seamlessly with the Snips platform and kickstart development of Voice applications. 

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

### Initialization
The connection to your MQTT broker can be configured with the `hermes_python.ffi.utils.MqttOptions` class.

The `Hermes` client uses the options specified in the `MqttOptions` class when establishing the connection to the MQTT broker. 

Here is a code example : 
```
from hermes_python.hermes import Hermes
from hermes_python.ffi.utils import MqttOptions

mqtt_opts = MqttOptions()

def simple_intent_callback(hermes, intent_message):
    print("I received an intent !")

with Hermes(mqtt_options=mqtt_opts) as h:
    h.subscribe_intents().loop_forever()

```

Here are the options you can specify in the MqttOptions class : 
- `broker_address`: The address of the MQTT broker. It should be formatted as `ip:port`. 
- `username`: Username to use on the broker. Nullable
- `password`: Password to use on the broker. Nullable
- `tls_hostname`: Hostname to use for the TLS configuration. Nullable, setting a value enables TLS
- `tls_ca_file`: CA files to use if TLS is enabled. Nullable
- `tls_ca_path`: CA path to use if TLS is enabled. Nullable
- `tls_client_key`: Client key to use if TLS is enabled. Nullable
- `tls_client_cert`: Client cert to use if TLS is enabled. Nullable
- `tls_disable_root_store`: Boolean indicating if the root store should be disabled if TLS is enabled.

Let's connect to an external MQTT broker that requires a username and a password :  

```
from hermes_python.hermes import Hermes
from hermes_python.ffi.utils import MqttOptions

mqtt_opts = MqttOptions(username="user1", password="password", broker_address="my-mqtt-broker.com:18852")

def simple_intent_callback(hermes, intent_message):
    print("I received an intent !")

with Hermes(mqtt_options=mqtt_opts) as h:
    h.subscribe_intents().loop_forever()

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
 
