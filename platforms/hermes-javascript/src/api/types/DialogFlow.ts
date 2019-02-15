import {
    IntentMessage,
    IntentMessageLegacy,
    IntentNotRecognizedMessage,
    IntentNotRecognizedMessageLegacy
} from './messages'

export type FlowContinuation<API> = {
    continue: (intentName: string, action: FlowIntentAction<API>) => void,
    notRecognized: (action: FlowNotRecognizedAction<API>) => void,
    end: () => void
}
export type FlowActionReturn<API> =
    string |
    API extends 'json' ? {
        text?: string,
        customData?: string
    } : {
        text?: string,
        custom_data?: string
    }

export type FlowIntentAction<API> = (
    message: API extends 'json' ? IntentMessage : IntentMessageLegacy,
    flow: FlowContinuation<API>
) => FlowActionReturn<API> | Promise<FlowActionReturn<API> | void> | void

export type FlowNotRecognizedAction<API> = (
    message: API extends 'json' ? IntentNotRecognizedMessage : IntentNotRecognizedMessageLegacy,
    flow: FlowContinuation<API>
) => FlowActionReturn<API> | Promise<FlowActionReturn<API> | void> | void
