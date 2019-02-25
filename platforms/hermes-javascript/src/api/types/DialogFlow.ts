import {
    IntentMessage,
    IntentMessageLegacy,
    IntentNotRecognizedMessage,
    IntentNotRecognizedMessageLegacy,
    SessionStartedMessage,
    SessionStartedMessageLegacy
} from './messages'

export type FlowContinuation<API = 'json'> = {
    continue: (intentName: string, action: FlowIntentAction<API>) => FlowActionReturn<API>,
    notRecognized: (action: FlowNotRecognizedAction<API>) => FlowActionReturn<API>,
    end: () => FlowActionReturn<API>
}
export type FlowActionReturnData<API = 'json'> =
    API extends 'json' ? {
        text?: string,
        customData?: string
    } : {
        text?: string,
        custom_data?: string
    }
export type FlowActionReturn<API = 'json'> =
    FlowActionReturnData<API> | string | void |
    Promise<FlowActionReturnData<API> | void | string>

export type FlowIntentAction<API = 'json'> = (
    message: API extends 'json' ? IntentMessage : IntentMessageLegacy,
    flow: FlowContinuation<API>
) => FlowActionReturn<API>

export type FlowNotRecognizedAction<API = 'json'> = (
    message: API extends 'json' ? IntentNotRecognizedMessage : IntentNotRecognizedMessageLegacy,
    flow: FlowContinuation<API>
) => FlowActionReturn<API>

export type FlowSessionAction<API = 'json'> = (
    message: API extends 'json' ? SessionStartedMessage : SessionStartedMessageLegacy,
    flow: FlowContinuation<API>
) => FlowActionReturn<API>
