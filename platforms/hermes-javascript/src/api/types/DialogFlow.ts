import {
    IntentMessage,
    IntentMessageLegacy,
    IntentNotRecognizedMessage,
    IntentNotRecognizedMessageLegacy,
    SessionStartedMessage,
    SessionStartedMessageLegacy
} from './messages'

export type FlowContinuation<API> = {
    continue: (intentName: string, action: FlowIntentAction<API>) => FlowActionReturn<API>,
    notRecognized: (action: FlowNotRecognizedAction<API>) => FlowActionReturn<API>,
    end: () => FlowActionReturn<API>
}
export type FlowActionReturnData<API> =
    API extends 'json' ? {
        text?: string,
        customData?: string
    } : {
        text?: string,
        custom_data?: string
    }
export type FlowActionReturn<API> =
    FlowActionReturnData<API> |
    Promise<FlowActionReturnData<API> | void | string> |
    void

export type FlowIntentAction<API> = (
    message: API extends 'json' ? IntentMessage : IntentMessageLegacy,
    flow: FlowContinuation<API>
) => FlowActionReturn<API>

export type FlowNotRecognizedAction<API> = (
    message: API extends 'json' ? IntentNotRecognizedMessage : IntentNotRecognizedMessageLegacy,
    flow: FlowContinuation<API>
) => FlowActionReturn<API>

export type FlowSessionAction<API> = (
    message: API extends 'json' ? SessionStartedMessage : SessionStartedMessageLegacy,
    flow: FlowContinuation<API>
) => FlowActionReturn<API>
