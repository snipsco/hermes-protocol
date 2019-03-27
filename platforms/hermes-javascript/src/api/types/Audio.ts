import {
    PlayBytesMessage,
    PlayFinishedMessage,
} from './messages'

export namespace AudioTypes {

    /**
     * The name and type of message that the Audio subset can publish.
     */
    export type publishMessagesList = {
        play_audio: PlayBytesMessage
    }

    /**
     * The name and type of message to which the Audio subset can subscribe.
     */
    export type subscribeMessagesList = {
        play_finished_all: PlayFinishedMessage
    } & {
        // Workaround because of the dynamic key
        [key: string]: PlayFinishedMessage
    }
}
