export interface PlayBytesMessage {
    /** Id of the playback. */
    id: string
    /** Site id to target. */
    siteId: string
    /** Sound buffer (Wav PCM16) stringified in base64. */
    wavBytes: string
    /** Length of the sound buffer. */
    wavBytesLen: number
}
