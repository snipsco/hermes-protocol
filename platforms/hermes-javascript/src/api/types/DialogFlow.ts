export type FlowContinuation = {
    continue: (intentName: string, action: FlowAction) => void,
    notRecognized: (action: FlowAction) => void,
    end: () => void
}
export type FlowActionReturn = string | {
    text?: string,
    custom_data?: string
}
export type FlowAction = (
    message: { [key: string]: any },
    flow: FlowContinuation
) => FlowActionReturn | Promise<FlowActionReturn | void> | void