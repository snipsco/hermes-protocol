import {
    IntentMessage,
    IntentNotRecognizedMessage,
    SessionStartedMessage,
} from './messages'

export type FlowContinuation = {
    continue: (intentName: string, action: FlowIntentAction, options?: { slotFiller: string | null }) => FlowActionReturn,
    notRecognized: (action: FlowNotRecognizedAction) => FlowActionReturn,
    end: () => void
}
export type FlowActionReturnData = {
    text?: string,
    customData?: string
}
export type FlowActionReturn =
    FlowActionReturnData | string | void |
    Promise<FlowActionReturnData | void | string>

export type FlowIntentAction = (
    message: IntentMessage,
    flow: FlowContinuation
) => FlowActionReturn

export type FlowNotRecognizedAction = (
    message: IntentNotRecognizedMessage,
    flow: FlowContinuation
) => FlowActionReturn

export type FlowSessionAction = (
    message: SessionStartedMessage,
    flow: FlowContinuation
) => FlowActionReturn
