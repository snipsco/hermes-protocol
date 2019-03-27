import {
    StartSessionMessage,
    ContinueSessionMessage,
    IntentNotRecognizedMessage,
    IntentMessage,
    EndSessionMessage,
    SessionEndedMessage,
    SessionQueuedMessage,
    SessionStartedMessage,
    DialogueConfigureMessage
} from './messages'

export namespace DialogTypes {

    /**
     * The name and type of message that the Dialog subset can publish.
     */
    export type publishMessagesList = {
        start_session: StartSessionMessage,
        continue_session: ContinueSessionMessage,
        end_session: EndSessionMessage,
        configure: DialogueConfigureMessage
    }

    /**
     * The name and type of message to which the Dialog subset can subscribe.
     */
    export type subscribeMessagesList = {
        intents: IntentMessage,
        intent_not_recognized: IntentNotRecognizedMessage,
        session_ended: SessionEndedMessage,
        session_queued: SessionQueuedMessage,
        session_started: SessionStartedMessage
    } & {
        // Workaround for intents that have a dynamic key
        [key: string]: IntentMessage
    }
}
