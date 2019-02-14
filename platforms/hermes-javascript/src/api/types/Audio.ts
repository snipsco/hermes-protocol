export namespace AudioTypes {

    export type PlayAudioMessage = {
        id: string,
        siteId: string,
        wavBytes: string,
        wavBytesLen: number
    } | {
        id: string,
        site_id: string,
        wav_bytes: Buffer,
        wav_bytes_len: number
    }

    export type publishMessagesList = {
        play_audio: PlayAudioMessage
    }
}