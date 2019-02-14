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

    export type PlayFinishedMessage = {
        id: string,
        siteId: string
    } & {
        id: string,
        site_id: string
    }

    export type publishMessagesList = {
        play_audio: PlayAudioMessage
    }
    export type subscribeMessagesList = {
        play_finished_all: PlayFinishedMessage
    } & {
        // Workaround for dynamic key
        [key: string]: PlayFinishedMessage
    }
}