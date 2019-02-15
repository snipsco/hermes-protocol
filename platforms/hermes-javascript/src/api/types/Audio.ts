import {
    PlayAudioMessageLegacy,
    PlayAudioMessage,
    PlayFinishedMessage,
    PlayFinishedMessageLegacy
} from './messages'

export namespace AudioTypes {
    export type publishMessagesList = {
        play_audio: PlayAudioMessage | PlayAudioMessageLegacy
    }
    export type subscribeMessagesList = {
        play_finished_all: PlayFinishedMessage & PlayFinishedMessageLegacy
    } & {
        // Workaround for dynamic key
        [key: string]: PlayFinishedMessage & PlayFinishedMessageLegacy
    }
}
