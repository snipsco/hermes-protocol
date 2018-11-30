class DialogFlow {
    constructor(dialog) {
        this.dialog = dialog
        this.reset()

        // Sets up a subscriber to clean up in case the session is ended programatically.
        const onSessionEnded = msg => {
            if(msg.session_id === this.sessionId) {
                this.cleanUpListeners()
                this.dialog.off('session_ended', onSessionEnded)
            }
        }
        this.dialog.on('session_ended', onSessionEnded)
    }

    reset() {
        this.continuations = new Map()
        this.continuationsListeners = new Map()
        this.ended = false
    }

    cleanUpListeners() {
        this.continuationsListeners.forEach((listener, intentName) => {
            this.dialog.off(`intent/${intentName}`, listener)
        })
    }

    // Starts a dialog flow.
    start(intentName, action, message) {
        const flow = {
            continue: this.continue.bind(this),
            end: this.end.bind(this)
        }
        return Promise.resolve(action(message, flow))
            .then(this.continuation.bind(this))
    }

    // Executed after a message callback has been processed.
    continuation(options = {}) {
        if(typeof options === 'string') {
            options = { text: options }
        }
        if(this.continuations.size > 0) {
            // If continue calls have been registered.
            let intent_filter = []
            this.continuations.forEach((action, intentName) => {
                intent_filter.push(intentName)
                const listener = this.createListener(action)
                const wrappedListener = this.dialog.once(`intent/${intentName}`, listener)
                this.continuationsListeners.set(intentName, wrappedListener)
            })
            this.dialog.publish('continue_session', {
                ...options,
                session_id: this.sessionId,
                intent_filter
            })
        } else if(this.ended) {
            // End the session.
            this.dialog.publish('end_session', {
                ...options,
                session_id: this.sessionId
            })
        }
    }

    createListener(action) {
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
                end: this.end.bind(this)
            }
            // Perform the message callback, then continue the flow
            return Promise.resolve(action(message, flow))
                .then(this.continuation.bind(this))
        }
    }

    /* Exposed methods */

    // Registers an intent filter and continue the current dialog session.
    continue(intentName, action) {
        this.continuations.set(intentName, action)
    }

    // Terminates the dialog session.
    end() {
        this.ended = true
    }
}

module.exports = DialogFlow