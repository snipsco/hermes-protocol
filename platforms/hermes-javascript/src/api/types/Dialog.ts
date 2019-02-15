import {
    StartSessionMessage,
    ContinueSessionMessage,
    IntentNotRecognizedMessage,
    IntentMessage,
    EndSessionMessage,
    SessionEndedMessage,
    SessionQueuedMessage,
    SessionStartedMessage,
    StartSessionMessageLegacy,
    ContinueSessionMessageLegacy,
    EndSessionMessageLegacy,
    IntentMessageLegacy,
    IntentNotRecognizedMessageLegacy,
    SessionEndedMessageLegacy,
    SessionQueuedMessageLegacy,
    SessionStartedMessageLegacy
} from './messages'

export namespace DialogTypes {
    export type publishMessagesList = {
        start_session: StartSessionMessage | StartSessionMessageLegacy,
        continue_session: ContinueSessionMessage | ContinueSessionMessageLegacy,
        end_session: EndSessionMessage | EndSessionMessageLegacy
    }
    export type subscribeMessagesList = {
        intents: IntentMessage & IntentMessageLegacy,
        intent_not_recognized: IntentNotRecognizedMessage & IntentNotRecognizedMessageLegacy,
        session_ended: SessionEndedMessage & SessionEndedMessageLegacy,
        session_queued: SessionQueuedMessage & SessionQueuedMessageLegacy,
        session_started: SessionStartedMessage & SessionStartedMessageLegacy
    } & {
        // Workaround for intents that have a dynamic key
        [key: string]: IntentMessage & IntentMessageLegacy
    }
}
