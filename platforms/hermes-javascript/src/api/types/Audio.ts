import {
    PlayAudioMessageLegacy,
    PlayAudioMessage,
    PlayFinishedMessage,
    PlayFinishedMessageLegacy
} from './messages'

export namespace AudioTypes {
    export type publishMessagesList<API> = {
        play_audio: API extends 'json' ? PlayAudioMessage : PlayAudioMessageLegacy
    }
    export type subscribeMessagesList<API> = {
        play_finished_all: API extends 'json' ? PlayFinishedMessage : PlayFinishedMessageLegacy
    } & {
        // Workaround for dynamic key
        [key: string]: API extends 'json' ? PlayFinishedMessage : PlayFinishedMessageLegacy
    }
}
