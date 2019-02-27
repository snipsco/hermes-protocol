import Dialog from './index'
import {
    FlowContinuation,
    FlowIntentAction,
    FlowNotRecognizedAction,
    FlowSessionAction,
} from '../types'
import {
    IntentMessage,
    SessionStartedMessage,
    SessionEndedMessage,
    IntentNotRecognizedMessage,
} from '../types/messages'

export default class DialogFlow {

    private continuations = new Map()
    private continuationsListeners = new Map()
    private notRecognizedAction = null
    private notRecognizedListener = null
    private ended = false
    private slotFiller = null

    constructor(private dialog: Dialog, public sessionId: string, done: () => void) {
        // Sets up a subscriber to clean up in case the session is ended programatically.
        const onSessionEnded = (msg: SessionEndedMessage) => {
            if(msg.sessionId === this.sessionId) {
                this.cleanUpListeners()
                this.reset()
                this.sessionId = null
                done()
            }
        }
        this.dialog.once('session_ended', onSessionEnded)
    }

    private reset() {
        this.continuations = new Map()
        this.continuationsListeners = new Map()
        this.notRecognizedAction = null
        this.notRecognizedListener = null
        this.ended = false
        this.slotFiller = null
    }

    private cleanUpListeners() {
        this.continuationsListeners.forEach((listener, intentName) => {
            this.dialog.off(`intent/${intentName}`, listener)
        })
        if(this.notRecognizedListener) {
            this.dialog.off('intent_not_recognized', this.notRecognizedListener)
        }
    }

    // Executed after a message callback has been processed.
    private continuation(options: { [key: string]: any } = {}) {
        if(typeof options === 'string') {
            options = { text: options }
        }
        if(this.ended) {
            // End the session.
            return this.dialog.publish('end_session', {
                text: '',
                ...options,
                sessionId: this.sessionId
            })
        }
        let intentFilter = []
        if(this.continuations.size > 0) {
            // If continue calls have been registered.
            this.continuations.forEach((action, intentName) => {
                intentFilter.push(intentName)
                const listener = this.createListener(action)
                const wrappedListener = this.dialog.once(`intent/${intentName}`, listener)
                this.continuationsListeners.set(intentName, wrappedListener)
            })
        }
        if(this.notRecognizedAction) {
            // If a listener has been set in case the intent has not been properly detected
            const listener = this.createListener(this.notRecognizedAction)
            const wrappedListener = this.dialog.once('intent_not_recognized', listener)
            this.notRecognizedListener = wrappedListener
            options.sendIntentNotRecognized = true
        }
        // Publish a continue session message
        this.dialog.publish('continue_session', {
            text: '',
            ...options,
            slot: this.slotFiller,
            sessionId: this.sessionId,
            intentFilter
        })
    }

    private createListener(action: FlowIntentAction | FlowNotRecognizedAction) {
        return (message: IntentMessage & IntentNotRecognizedMessage) => {
            // Checks the session id
            if(message.sessionId !== this.sessionId)
                return
            // Cleans up other listeners that could have been registered using .continue
            this.cleanUpListeners()
            // Resets the state
            this.reset()
            // Exposes .continue / .end / .notRecognized
            const flow = {
                continue: this.continue.bind(this),
                notRecognized: this.notRecognized.bind(this),
                end: this.end.bind(this)
            }
            // Perform the message callback, then continue the flow
            return Promise.resolve(action(message, flow))
                .then(this.continuation.bind(this))
        }
    }

    // Starts a dialog flow.
    start(action: FlowIntentAction | FlowSessionAction, message: IntentMessage | SessionStartedMessage) {
        const flow : FlowContinuation = {
            continue: this.continue.bind(this),
            notRecognized: this.notRecognized.bind(this),
            end: this.end.bind(this)
        }
        return Promise.resolve(action(message as any, flow))
            .then(this.continuation.bind(this))
    }

    // Registers an intent filter and continue the current dialog session.
    continue(intentName: string, action: FlowIntentAction, { slotFiller } : { slotFiller?: string} = { slotFiller: null }) {
        this.slotFiller = slotFiller
        this.continuations.set(intentName, action)
    }

    // Registers a listener that will be called if no intents have been recognized.
    notRecognized(action: FlowNotRecognizedAction) {
        this.notRecognizedAction = action
    }

    // Terminates the dialog session.
    end() {
        this.ended = true
    }
}

module.exports = DialogFlow