import { RegisterSoundMessage } from './messages'

export namespace TtsTypes {

    /**
     * The name and type of message that the Tts subset can publish.
     */
    export type publishMessagesList = {
        register_sound: RegisterSoundMessage
    }
}