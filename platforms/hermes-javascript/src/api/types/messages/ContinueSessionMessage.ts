export interface ContinueSessionMessage {
    /** Id of the dialogue session to continue. */
    sessionId: string
    /** Text that will be spoken by the TTS. */
    text?: string
    /**
     * A list of intents that will be expected in the next dialogue round.
     * If specified, the speech recognition will try to match **only** these intents.
     */
    intentFilter?: string[]
    /** A custom string stored and passed to the next dialogue round. */
    customData?: string
    /** Enables the slot filler that will expect this specific slot to be spoken. */
    slot?: string
    /**
     * Indicates whether the dialogue manager should handle non recognized
     * intents by itself or sent them as an Intent Not Recognized for the client to handle.
     * This setting applies only to the next conversation turn.
     */
    sendIntentNotRecognized?: boolean
}
