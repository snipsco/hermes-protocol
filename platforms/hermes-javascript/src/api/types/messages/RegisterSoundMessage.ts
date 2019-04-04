export interface RegisterSoundMessage {
    /** Sound label. */
    soundId: string
    /** Sound buffer (Wav PCM16) stringified in base64. */
    wavSound: string
    /**
     * @deprecated
     * **Unused! Kept for avoiding breaking existing action code.**
     *
     * Length of the sound buffer.
     */
    wavSoundLen?: number
}
