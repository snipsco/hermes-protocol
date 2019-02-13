import ref from 'ref'
import ApiSubset from '../ApiSubset'
import DialogFlow from './DialogFlow'
import { FlowAction } from '../types'
import {
    StringArray,
    StartSessionMessage,
    IntentMessage
} from '../../casts'
import {
    CContinueSessionMessage,
    CEndSessionMessage,
    CIntentMessage,
    CIntentNotRecognizedMessage,
    CSessionEndedMessage,
    CSessionQueuedMessage,
    CSessionStartedMessage
} from '../../ffi/typedefs'

export default class Dialog extends ApiSubset {

    constructor(protocolHandler, call, options) {
        super(protocolHandler, call, options, 'hermes_protocol_handler_dialogue_facade')
    }

    private activeSessions = new Set()
    publishEvents = {
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
    subscribeEvents = {
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

    destroy() {
        this.call('hermes_drop_dialogue_facade', this.facade)
    }

    /**
     * Sets up a dialog flow.
     * @param {*} intent Starting intent name.
     * @param {*} action Action to perform when the starting intent is triggered.
     */
    flow(intent: string, action: FlowAction) {
        return this.flows([{ intent, action }])
    }

    /**
     * Sets up a dialog flow with multiple starting intents.
     * @param {*} intents An array of { intent, action } objects.
     */
    flows(intents: { intent: string, action: FlowAction }[]) {
        intents.forEach(({ intent, action }) => {
            this.on(`intent/${intent}`, message => {
                const sessionId = this.options.useJsonApi ? message.sessionId : message.session_id
                // If this particular session is already in progress - prevent
                if(this.activeSessions.has(sessionId))
                    return
                const flow = new DialogFlow(this, sessionId, () => {
                    this.activeSessions.delete(sessionId)
                }, { useJsonApi: this.options.useJsonApi })
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
    sessionFlow(id: string, action: FlowAction) {
        const listener = message => {
            const { useJsonApi } = this.options
            const customData = useJsonApi ? message.customData : message.custom_data
            const sessionId = useJsonApi ? message.sessionId : message.session_id

            if(customData !== id)
                return
            this.off('session_started', listener)
            const flow = new DialogFlow(this, sessionId, () => {
                this.activeSessions.delete(sessionId)
            }, { useJsonApi })
            this.activeSessions.add(sessionId)
            return flow.start(action, message)
        }
        this.on('session_started', listener)
    }

    static enums = {
        grain: {
            year: 'Year',
            quarter: 'Quarter',
            month: 'Month',
            week: 'Week',
            day: 'Day',
            hour: 'Hour',
            minute: 'Minute',
            second: 'Second'
        },
        precision: {
            approximate: 'Approximate',
            exact: 'Exact'
        },
        initType: {
            action: 'action',
            notification: 'notification'
        },
        terminationType: {
            nominal: 'nominal',
            unavailable: 'siteUnavailable',
            abortedByUser: 'abortedByUser',
            intentNotRecognized: 'intentNotRecognized',
            timeout: 'timeout',
            error: 'error'
        },
        slotType: {
            custom: 'Custom',
            number: 'Number',
            ordinal: 'Ordinal',
            instantTime: 'InstantTime',
            timeInterval: 'TimeInterval',
            amountOfMoney: 'AmountOfMoney',
            temperature: 'Temperature',
            duration: 'Duration',
            percentage: 'Percentage',
            musicAlbum: 'MusicAlbum',
            musicArtist: 'MusicArtist',
            musicTrack: 'MusicTrack'
        },
        legacy: {
            grain: {
                year: 0,
                quarter: 1,
                month: 2,
                week: 3,
                day: 4,
                hour: 5,
                minute: 6,
                second: 7
            },
            precision: {
                approximate: 0,
                exact: 1
            },
            initType: {
                action: 1,
                notification: 2
            },
            terminationType: {
                nominal: 1,
                unavailable: 2,
                abortedByUser: 3,
                intentNotRecognized: 4,
                timeout: 5,
                error: 6
            },
            slotType: {
                custom: 1,
                number: 2,
                ordinal: 3,
                instantTime: 4,
                timeInterval: 5,
                amountOfMoney: 6,
                temperature: 7,
                duration: 8,
                percentage: 9,
                musicAlbum: 10,
                musicArtist: 11,
                musicTrack: 12
            }
        }
    }
}
