export interface EndSessionMessage {
    /** Id of the session to end. */
    sessionId: string
    /** Text that will be spoken by the TTS. */
    text?: string
}
