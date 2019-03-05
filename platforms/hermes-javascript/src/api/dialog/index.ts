import ref from 'ref'
import ApiSubset from '../ApiSubset'
import DialogFlow from './DialogFlow'
import {
    FlowIntentAction,
    FlowSessionAction,
    DialogTypes,
    IntentMessage,
    SessionStartedMessage,
    FFIFunctionCall,
    HermesOptions
} from '../types'
import * as enums from '../types/enums'

export default class Dialog extends ApiSubset {

    constructor(protocolHandler: Buffer, call: FFIFunctionCall, options: HermesOptions) {
        super(protocolHandler, call, options, 'hermes_protocol_handler_dialogue_facade')
    }

    private activeSessions = new Set()
    publishEvents = {
        start_session: {
            fullEventName: 'hermes_dialogue_publish_start_session_json'
        },
        continue_session: {
            fullEventName: 'hermes_dialogue_publish_continue_session_json'
        },
        end_session: {
            fullEventName: 'hermes_dialogue_publish_end_session_json'
        }
    }
    publishMessagesList: DialogTypes.publishMessagesList = undefined as any

    subscribeEvents = {
        'intent/': {
            fullEventName: 'hermes_dialogue_subscribe_intent_json',
            additionalArguments: eventName => [
                ref.allocCString(eventName.substring(7))
            ]
        },
        intents: {
            fullEventName: 'hermes_dialogue_subscribe_intents_json'
        },
        intent_not_recognized: {
            fullEventName: 'hermes_dialogue_subscribe_intent_not_recognized_json'
        },
        session_ended: {
            fullEventName: 'hermes_dialogue_subscribe_session_ended_json'
        },
        session_queued: {
            fullEventName: 'hermes_dialogue_subscribe_session_queued_json'
        },
        session_started: {
            fullEventName: 'hermes_dialogue_subscribe_session_started_json'
        }
    }
    subscribeMessagesList: DialogTypes.subscribeMessagesList = undefined as any

    destroy() {
        this.call('hermes_drop_dialogue_facade', this.facade)
    }

    /**
     * Sets up a dialog flow.
     * @param {*} intent Starting intent name.
     * @param {*} action Action to perform when the starting intent is triggered.
     */
    flow(intent: string, action: FlowIntentAction) {
        return this.flows([{ intent, action }])
    }

    /**
     * Sets up a dialog flow with multiple starting intents.
     * @param {*} intents An array of { intent, action } objects.
     */
    flows(intents: { intent: string, action: FlowIntentAction }[]) {
        intents.forEach(({ intent, action }) => {
            this.on(`intent/${intent}`, (message: IntentMessage) => {
                const sessionId = message.sessionId
                // If this particular session is already in progress - prevent
                if(this.activeSessions.has(sessionId))
                    return
                const flow = new DialogFlow(this, sessionId, () => {
                    this.activeSessions.delete(sessionId)
                })
                this.activeSessions.add(sessionId)
                return flow.start(action, message)
            })
        })
    }

    /**
     * Creates a dialog flow that will trigger when the target session starts.
     * Useful when initiating a session programmatically.
     *
     * @param id : An id that should match the customData field of the started session.
     * @param action : The action to execute on session startup.
     */
    sessionFlow(id: string, action: FlowSessionAction) {
        const listener = (message: SessionStartedMessage) => {
            const customData = message.customData
            const sessionId = message.sessionId

            if(customData !== id)
                return
            this.off('session_started', listener)
            const flow = new DialogFlow(this, sessionId, () => {
                this.activeSessions.delete(sessionId)
            })
            this.activeSessions.add(sessionId)
            return flow.start(action, message, { sessionStart: true })
        }
        this.on('session_started', listener)
    }

    static enums = {
        grain: enums.grain,
        precision: enums.precision,
        initType: enums.initType,
        terminationType: enums.terminationType,
        slotType: enums.slotType
    }
}
