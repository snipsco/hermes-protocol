export type PlayAudioMessage = {
    id: string,
    siteId: string,
    wavBytes: string,
    wavBytesLen: number
}

export type PlayAudioMessageLegacy = {
    id: string,
    site_id: string,
    wav_bytes: Buffer,
    wav_bytes_len: number
}
