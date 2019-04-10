import Dialog from './index'
import {
    FlowContinuation,
    FlowIntentAction,
    FlowNotRecognizedAction,
    FlowSessionAction,
    MessageListener,
} from '../types'
import {
    IntentMessage,
    SessionStartedMessage,
    SessionEndedMessage,
    IntentNotRecognizedMessage,
    EndSessionMessage,
    ContinueSessionMessage,
} from '../types/messages'

/**
 * Dialog flow session manager.
 */
export default class DialogFlow {

    private continuations = new Map()
    private continuationsListeners = new Map()
    private notRecognizedAction: FlowNotRecognizedAction | null = null
    private notRecognizedListener?: MessageListener<IntentNotRecognizedMessage> | null = null
    private ended: boolean = false
    private slotFiller: string | null = null

    /**
     * @internal **For internal use only**
     * @param dialog - Dialog instance.
     * @param sessionId - The session id to manage.
     * @param done - A callback to perform when the session ends.
     */
    constructor(private dialog: Dialog, public sessionId: string | null, done: () => void) {
        // Sets up a subscriber to clean up in case the session is ended programatically.
        const onSessionEnded = (msg: SessionEndedMessage | undefined) => {
            if(msg && msg.sessionId === this.sessionId) {
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
    private continuation(
        options: string | void | Partial<EndSessionMessage & ContinueSessionMessage> = {},
        { sessionStart = false } = {}
    ) {
        let messageOptions: Partial<EndSessionMessage & ContinueSessionMessage> = {}
        if(typeof options === 'string') {
            messageOptions = { text: options }
        }
        if(this.ended) {
            // End the session.
            return this.dialog.publish('end_session', {
                text: '',
                ...messageOptions,
                sessionId: this.sessionId
            } as EndSessionMessage)
        }
        let intentFilter: string[] = []
        if(this.continuations.size > 0) {
            // If continue calls have been registered.
            this.continuations.forEach((action, intentName) => {
                intentFilter.push(intentName)
                const listener = this.createListener(action)
                const wrappedListener = this.dialog.once(`intent/${intentName}`, listener as any)
                this.continuationsListeners.set(intentName, wrappedListener)
            })
        }
        if(this.notRecognizedAction) {
            // If a listener has been set in case the intent has not been properly detected
            const listener = this.createListener(this.notRecognizedAction)
            const wrappedListener = this.dialog.once('intent_not_recognized', listener as any)
            this.notRecognizedListener = wrappedListener
            messageOptions.sendIntentNotRecognized = true
        }
        if(!sessionStart) {
            // Publish a continue session message
            this.dialog.publish('continue_session', {
                text: '',
                ...messageOptions,
                slot: this.slotFiller,
                sessionId: this.sessionId,
                intentFilter
            } as ContinueSessionMessage)
        }
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

    /**
     * @internal **For internal use only.**
     *
     * Starts a dialog flow.
     *
     * @param action - Action to perform on flow creation.
     * @param message - The message received on flow creation.
     * @param options - Internal options.
     */
    start(action: FlowIntentAction | FlowSessionAction, message: IntentMessage | SessionStartedMessage, { sessionStart = false } = {}) {
        const flow : FlowContinuation = {
            continue: this.continue.bind(this),
            notRecognized: this.notRecognized.bind(this),
            end: this.end.bind(this)
        }
        return Promise.resolve(action(message as any, flow))
            .then((...args) => this.continuation.bind(this)(...args, { sessionStart }))
    }

    // Registers an intent filter and continue the current dialog session.
    private continue(intentName: string, action: FlowIntentAction, continueOptions : { slotFiller: string | null } = { slotFiller: null }) {
        this.slotFiller = continueOptions.slotFiller
        this.continuations.set(intentName, action)
    }

    // Registers a listener that will be called if no intents have been recognized.
    private notRecognized(action: FlowNotRecognizedAction) {
        this.notRecognizedAction = action
    }

    // Terminates the dialog session.
    private end() {
        this.ended = true
    }
}

module.exports = DialogFlow