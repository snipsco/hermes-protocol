# hermes-javascript

#### A javascript wrapper around around the hermes protocol

## Setup

```sh
npm install hermes-javascript
```

## Usage

```js
const { withHermes } = require('hermes-javascript')

/*
    A small js context manager that sets up an infinite loop
    to prevent the process from exiting, and will stop and
    clean up the mess when the done() function gets called.
*/
withHermes((hermes, done) => {
    const dialog = hermes.dialog()

    /* Basic example of an intent subscriber. */

    dialog.on('intent/someIntent', message => {
        console.log('received intent '+ message.intent.intent_name)

        dialog.publish('continue_session', {
            session_id: message.session_id,
            text: 'Session continued',
            intent_filter: ['nextIntent']
        })

        // OR ~~~~~~~~~~~~~~~~~~~~~~~~~~~

        dialog.publish('end_session', {
            session_id: message.session_id,
            text: 'Session ended'
        })
    })

    /* You can also unsubscribe easily. */

    const handler = message => {
        console.log(message)
        // Unsubscribe the first time this message is received.
        dialog.off('intent/someIntent', handler)
        // publish continue / end …
    }
    dialog.on('intent/someIntent', handler)

    /* Or process a message only once. */

    dialog.once('intent/someIntent', message => {
        console.log(message)
        // publish continue / end …
    })

    /*
        There is also a small wrapper on top of the dialog API.
        It allows you to setup complex dialog flows easily.
    */

    dialog.flow('A', (msg, flow) => {

        console.log('Intent A received. Session started.')

        // You can grab a slot and its value like this
        const mySlot = msg.slots.find(slot => slot.slot_name === 'slotName')
        const slotValue = mySlot.value.value

        // We then subscribe to both intent B or C so that the dialog
        // flow will continue with either one or the other next.

        // A -> B
        flow.continue('B', (msg, flow) => {
            console.log('Intent B received. Session continued.')

            // A -> B -> D
            flow.continue('D', (msg, flow) => {
                console.log('Intent D received. Session is ended.')
                flow.end()
                return 'Finished the session with intent D.'
            })

            return 'Continue with D.'
        })

        // A -> C
        flow.continue('C', (msg, flow) => {
            const slotValue = msg.slots[0].value.value
            console.log('Intent C received. Session is ended.')
            flow.end()
            return 'Finished the session with intent C having value ' + slotValue + ' .'
        })

        // The continue / end message options (basically TTS)
        // If the return value is a string, then it is equivalent to { text: '...' }
        return 'Continue with B or C.'
    })

}, { logs: true })
```

## API

### Context loop

An hermes client should implement a context loop that will prevent the program from exiting.

#### Using withHermes

```js
const { withHermes } = require('hermes-javascript')

// See the Hermes class documentation for available options.
const hermesOptions = { /* ... */ }

/*
The withHermes function automatically sets up the context loop.

Arguments:
   - hermes is a freshly created instance of the Hermes class
   - call done() to exit the loop and destroy() the hermes instance
*/
withHermes((hermes, done) => {
    /* ... */
}, hermesOptions)
```

#### Using the keepAlive tools

```js
const { Hermes, tools: { keepAlive, killKeepAlive }} = require('hermes-javascript')

const hermes = new Hermes(/* options */)

// Sleeps for 20 miliseconds between each loop cycle to prevent heavy CPU usage
tools.keepAlive(20)

// Call done to free resources and stop the loop
function done () {
    hermes.destroy()
    killKeepAlive()
}

/* ... */
```

### Hermes class

```js
new Hermes({
    // The hermes bus address (default localhost:1883)
    address: 'localhost:1883',
    // Enables or disables stdout logs (default true).
    // Use it in conjunction with the RUST_LOG environment variable (set -x RUST_LOG debug)
    logs: true,
    // A custom path to the hermes FFI dynamic library file.
    libraryPath: // default: at the hermes-js package root folder location
})
```

#### dialog()

Use the Dialog Api Subset.

```js
const hermes = new Hermes()
const dialog = hermes.dialog()
```

#### destroy()

Release the resources associated with this Hermes instance.

```js
const hermes = new Hermes()
hermes.destroy()
```

### ApiSubset common methods

**Check out [the hermes protocol documentation](https://snips.gitbook.io/documentation/ressources/hermes-protocol) for more details on the event names.**

#### on(eventName, listener)

Subscribes to an event on the bus, then unsubscribes after the first event is received.

```js
// Example for the dialog subset.

const hermes = new Hermes()
const dialog = hermes.dialog()

dialog.on('session_started', message => {
    /* ... */
})
```

#### once(eventName, listener)

Subscribes to an event on the bus, then unsubscribes after the first event is received.

```js
// Example for the dialog subset.

const hermes = new Hermes()
const dialog = hermes.dialog()

dialog.once('intent/myIntent', message => {
    /* ... */
})
```

#### off(eventName, listener)

Unsubscribe an already existing event.

```js
// Example for the dialog subset.

const hermes = new Hermes()
const dialog = hermes.dialog()

const handler = message => {
    /* ... */
}

// Subscribes
dialog.on('intent/myIntent', handler)

// Unsubscribes
dialog.off('intent/myIntent', handler)
```

#### publish(eventName, message)

Publish an event programatically.

```js
// Example for the dialog subset.

const hermes = new Hermes()
const dialog = hermes.dialog()

dialog.publish('start_session', {
    custom_data: 'some data',
    site_id: 'site Id',
    session_init: {
        init_type: 2,
        value: 'notification'
    }
})
```

### Dialog Api Subset

The dialog manager.



#### Events available for publishing

- **start_session**

Start a new dialog session.

```js
dialog.publish('start_session', {
    custom_data: /* string */,
    site_id: /* string */,
    session_init: {
        init_type: /* 1 or 2 */,
        value:
        /* If init_type is 1 */
            /* string */
        /* If init_type is 2 */
            {
                text: /* string */,
                intent_filter: /* string[] */,
                can_be_enqueued: /* char */
            }
    }
})

```

- **continue_session**

Continue a dialog session.


```js
dialog.publish('continue_session', {
    session_id: /* string */,
    text: /* string */,
    intent_filter: /* string[] */,
    custom_data: /* string */,
    send_intent_not_recognized: /* uchar */
})
```

- **end_session**

Finish a dialog session.

```js
dialog.publish('end_session', {
    session_id: /* string */,
    text: /* string */
})
```

#### Events available for subscribing

- **intent/[intentName]**

An intent was recognized.

- **session_ended**

A dialog session has ended.

- **session_queued**

A dialog session has been put in the queue.

- **session_started**

A dialog session has started.

#### flow(intentName, action)

Starts a new dialog flow.

```js
const hermes = new Hermes()
const dialog = hermes.dialog()

dialog.flow('intentName', (message, flow) => {

    // Chain flow actions (continue / end)…

    // Return the text to speech if needed.
    return 'intentName recognized!'
})

// You can also return an object that will be used for
// the 'continue_session' or 'end_session' parameters.

dialog.flow('intentName', (message, flow) => {

    // Chain flow actions (continue / end)…

    return {
        text: 'intentName recognized!'
    }
})

// If you need to perform asynchronous calculations
// Just return a promise and the flow actions will
// be performed afterwards.

dialog.flow('intentName', async (message, flow) => {
    const json = await fetch('something').then(res => res.json())

    // Chain flow actions (continue / end)…

    return 'Fetched some stuff!'
})
```

#### flow->continue(intentName, action)

Subscribes to an intent for the next dialog step.

```js
dialog.flow('intentName', async (message, flow) => {

    flow.continue('otherIntent', (message, flow) => {
        /* ... */
    })

    flow.continue('andAnotherIntent', (message, flow) => {
        /* ... */
    })

    return 'Continue with either one of these 2 intents.'
})
```

#### flow->notRecognized(action)

Add a callback that is going to be executed if the intents failed to be recognized.

```js
dialog.flow('intentName', async (message, flow) => {

    /* Add continuations here ... */

    flow.notRecognized((message, flow) => {
        /* ... */
    })

    return 'If the dialog failed to understand the intents, notRecognized callback will be called.'
})
```

#### flow->end()

Ends the dialog flow.

```js
dialog.flow('intentName', async (message, flow) => {
    flow.end()
    return 'Dialog ended.'
})
```

## License

MIT