const ref = require('ref')
const ApiSubset = require('../ApiSubset')
const DialogFlow = require('./DialogFlow')
const {
    StringArray,
    StartSessionMessage,
    IntentMessage
} = require('../../casts')
const {
    CContinueSessionMessage,
    CEndSessionMessage,
    CIntentMessage,
    CIntentNotRecognizedMessage,
    CSessionEndedMessage,
    CSessionQueuedMessage,
    CSessionStartedMessage
} = require('../../ffi/typedefs')

class Dialog extends ApiSubset {

    constructor(protocolHandler, options) {
        super(options)
        const dialogueFacadeRef = ref.alloc('void **')
        this.call('hermes_protocol_handler_dialogue_facade', protocolHandler, dialogueFacadeRef)
        this.facade = dialogueFacadeRef.deref()
    }

    destroy () {
        this.call('hermes_drop_dialogue_facade', this.facade)
    }

    /**
     * Sets up a dialog flow.
     * @param {*} intentName Starting intent name.
     * @param {*} action Action to perform when the starting intent is triggered.
     * @param {*} options The continuation / end message options.
     */
    flow (intentName, action) {
        const flow = new DialogFlow(this)
        this.on(`intent/${intentName}`, message => {
            flow.sessionId = message.session_id
            return flow.start(intentName, action, message)
        })
        return flow
    }
}

Dialog.prototype.publishEvents = {
    start_session: {
        fullEventName: 'hermes_dialogue_publish_start_session',
        messageClass: StartSessionMessage
    },
    continue_session: {
        fullEventName: 'hermes_dialogue_publish_continue_session',
        forgedStruct: CContinueSessionMessage,
        forgeOptions: {
            intent_filter: intents => new StringArray(intents).forge()
        }
    },
    end_session: {
        fullEventName: 'hermes_dialogue_publish_end_session',
        forgedStruct: CEndSessionMessage
    }
}

Dialog.prototype.subscribeEvents = {
    'intent/': {
        fullEventName: 'hermes_dialogue_subscribe_intent',
        dropEventName: 'hermes_drop_intent_message',
        additionalArguments: eventName => [
            ref.allocCString(eventName.substring(7))
        ],
        messageStruct: CIntentMessage,
        messageClass: IntentMessage
    },
    intents: {
        fullEventName: 'hermes_dialogue_subscribe_intents',
        dropEventName: 'hermes_drop_intent_message',
        messageStruct: CIntentMessage,
        messageClass: IntentMessage
    },
    intent_not_recognized: {
        fullEventName: 'hermes_dialogue_subscribe_intent_not_recognized',
        dropEventName: 'hermes_drop_intent_not_recognized_message',
        messageStruct: CIntentNotRecognizedMessage
    },
    session_ended: {
        fullEventName: 'hermes_dialogue_subscribe_session_ended',
        dropEventName: 'hermes_drop_session_ended_message',
        messageStruct: CSessionEndedMessage
    },
    session_queued: {
        fullEventName: 'hermes_dialogue_subscribe_session_queued',
        dropEventName: 'hermes_drop_session_queued_message',
        messageStruct: CSessionQueuedMessage

    },
    session_started: {
        fullEventName: 'hermes_dialogue_subscribe_session_started',
        dropEventName: 'hermes_drop_session_started_message',
        messageStruct: CSessionStartedMessage
    }
}

module.exports = Dialog