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
        super(options, 'hermes_protocol_handler_dialogue_facade', protocolHandler)
        this.activeSessions = new Set()
    }

    destroy() {
        this.call('hermes_drop_dialogue_facade', this.facade)
    }

    /**
     * Sets up a dialog flow.
     * @param {*} intent Starting intent name.
     * @param {*} action Action to perform when the starting intent is triggered.
     */
    flow(intent, action) {
        return this.flows([{ intent, action }])
    }

    /**
     * Sets up a dialog flow with multiple starting intents.
     * @param {*} intents An array of { intent, action } objects.
     */
    flows(intents) {
        intents.forEach(({ intent, action }) => {
            this.on(`intent/${intent}`, message => {
                const sessionId = message.session_id
                // If this particular session is already in progress - prevent
                if(this.activeSessions.has(sessionId))
                    return
                const flow = new DialogFlow(this, sessionId, () => {
                    this.activeSessions.delete(sessionId)
                })
                this.activeSessions.add(sessionId)
                return flow.start(intent, action, message)
            })
        })
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