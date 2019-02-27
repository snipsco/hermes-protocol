export type ContinueSessionMessage = {
    sessionId: string,
    text?: string,
    intentFilter?: string[],
    customData?: string,
    slot?: string,
    sendIntentNotRecognized?: boolean
}
