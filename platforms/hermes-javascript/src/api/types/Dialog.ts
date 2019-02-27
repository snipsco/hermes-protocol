import {
    StartSessionMessage,
    ContinueSessionMessage,
    IntentNotRecognizedMessage,
    IntentMessage,
    EndSessionMessage,
    SessionEndedMessage,
    SessionQueuedMessage,
    SessionStartedMessage
} from './messages'

export namespace DialogTypes {
    export type publishMessagesList = {
        start_session: StartSessionMessage,
        continue_session: ContinueSessionMessage,
        end_session: EndSessionMessage
    }
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
