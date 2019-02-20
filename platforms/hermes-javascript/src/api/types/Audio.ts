import {
    PlayAudioMessageLegacy,
    PlayAudioMessage,
    PlayFinishedMessage,
    PlayFinishedMessageLegacy
} from './messages'
import { HermesAPI } from '.'

export namespace AudioTypes {
    export type publishMessagesList<API extends HermesAPI = 'json'> = {
        play_audio: API extends 'json' ? PlayAudioMessage : PlayAudioMessageLegacy
    }
    export type subscribeMessagesList<API extends HermesAPI = 'json'> = {
        play_finished_all: API extends 'json' ? PlayFinishedMessage : PlayFinishedMessageLegacy
    } & {
        // Workaround for dynamic key
        [key: string]: API extends 'json' ? PlayFinishedMessage : PlayFinishedMessageLegacy
    }
}
