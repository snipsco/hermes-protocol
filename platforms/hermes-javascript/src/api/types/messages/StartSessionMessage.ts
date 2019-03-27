import { initType } from '../enums'

export interface StartSessionMessage {
    /** Session initializer. */
    init: {
        /** The type of the session. */
        type: initType,
        /** Text that will be spoken by the TTS. */
        text?: string,
        /**
         * A list of intents that will be expected in the next dialogue round.
         * If specified, the speech recognition will try to match **only** these intents.
         */
        intentFilter?: string[],
        /** Specify if the session can be enqueued. */
        canBeEnqueued?: boolean,
        /**
         * Indicates whether the dialogue manager should handle non recognized
         * intents by itself or sent them as an Intent Not Recognized for the client to handle.
         * This setting applies only to the next conversation turn.
         */
        sendIntentNotRecognized?: boolean
    }
    /** A custom string stored and passed to the next dialogue round. */
    customData?: string
    /** The id of the site to start the session on. */
    siteId?: string
}
