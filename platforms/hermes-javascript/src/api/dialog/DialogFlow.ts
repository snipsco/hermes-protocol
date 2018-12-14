// eslint-disable-next-line
import Dialog from './index'

export type FlowContinuation = {
    continue: (intentName: string, action: FlowAction) => void,
    notRecognized: (action: FlowAction) => void,
    end: () => void
}
export type FlowActionReturn = string | {
    text?: string,
    custom_data?: string
}
export type FlowAction = (
    message: { [key: string]: any },
    flow: FlowContinuation
) => FlowActionReturn | Promise<FlowActionReturn | void> | void

export default class DialogFlow {

    private continuations = new Map()
    private continuationsListeners = new Map()
    private notRecognizedAction = null
    private notRecognizedListener = null
    private ended = false

    // eslint-disable-next-line
    constructor(private dialog: Dialog, public sessionId: string, done: () => void) {
        // Sets up a subscriber to clean up in case the session is ended programatically.
        const onSessionEnded = msg => {
            if(msg.session_id === this.sessionId) {
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
    }

    private cleanUpListeners() {
        this.continuationsListeners.forEach((listener, intentName) => {
            this.dialog.off(`intent/${intentName}`, listener)
        })
        if(this.notRecognizedListener) {
            this.dialog.off('intent_not_recognized', this.notRecognizedListener)
        }
    }

    // Starts a dialog flow.
    start(action: FlowAction, message: { [key: string]: any }) {
        const flow : FlowContinuation = {
            continue: this.continue.bind(this),
            notRecognized: this.notRecognized.bind(this),
            end: this.end.bind(this)
        }
        return Promise.resolve(action(message, flow))
            .then(this.continuation.bind(this))
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
                session_id: this.sessionId
            })
        }
        let intent_filter = []
        if(this.continuations.size > 0) {
            // If continue calls have been registered.
            this.continuations.forEach((action, intentName) => {
                intent_filter.push(intentName)
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
            options.send_intent_not_recognized = 1
        }
        // Publish a continue session message
        this.dialog.publish('continue_session', {
            text: '',
            ...options,
            session_id: this.sessionId,
            intent_filter
        })
    }

    private createListener(action) {
        return message => {
            // Checks the session id
            if(message.session_id !== this.sessionId)
                return
            // Cleans up other listeners that could have been registered using .continue
            this.cleanUpListeners()
            // Resets the state
            this.reset()
            // Exposes .continue / .end
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

    /* Exposed methods */

    // Registers an intent filter and continue the current dialog session.
    continue(intentName: string, action: FlowAction) {
        this.continuations.set(intentName, action)
    }

    // Registers a listener that will be called if no intents have been recognized.
    notRecognized(action: FlowAction) {
        this.notRecognizedAction = action
    }

    // Terminates the dialog session.
    end() {
        this.ended = true
    }
}

module.exports = DialogFlow