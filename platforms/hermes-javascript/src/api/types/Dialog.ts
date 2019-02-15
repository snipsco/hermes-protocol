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
    export type publishMessagesList<API> = {
        start_session: API extends 'json' ? StartSessionMessage : StartSessionMessageLegacy,
        continue_session: API extends 'json' ? ContinueSessionMessage : ContinueSessionMessageLegacy,
        end_session: API extends 'json' ? EndSessionMessage : EndSessionMessageLegacy
    }
    export type subscribeMessagesList<API> = {
        intents: API extends 'json' ? IntentMessage : IntentMessageLegacy,
        intent_not_recognized: API extends 'json' ? IntentNotRecognizedMessage : IntentNotRecognizedMessageLegacy,
        session_ended: API extends 'json' ? SessionEndedMessage : SessionEndedMessageLegacy,
        session_queued: API extends 'json' ? SessionQueuedMessage : SessionQueuedMessageLegacy,
        session_started: API extends 'json' ? SessionStartedMessage : SessionStartedMessageLegacy
    } & {
        // Workaround for intents that have a dynamic key
        [key: string]: API extends 'json' ? IntentMessage : IntentMessageLegacy
    }
}
