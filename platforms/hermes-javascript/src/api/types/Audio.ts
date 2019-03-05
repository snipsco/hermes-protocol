import {
    PlayBytesMessage,
    PlayFinishedMessage,
} from './messages'

export namespace AudioTypes {
    export type publishMessagesList = {
        play_audio: PlayBytesMessage
    }
    export type subscribeMessagesList = {
        play_finished_all: PlayFinishedMessage
    } & {
        // Workaround for dynamic key
        [key: string]: PlayFinishedMessage
    }
}
