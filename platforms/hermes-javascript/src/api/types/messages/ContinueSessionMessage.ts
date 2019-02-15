export type ContinueSessionMessage = {
    sessionId: string,
    text?: string,
    intentFilter?: string[],
    customData?: string,
    sendIntentNotRecognized?: boolean
}

export type ContinueSessionMessageLegacy = {
    session_id: string,
    text?: string,
    intent_filter?: string[],
    custom_data?: string,
    send_intent_not_recognized?: boolean
}
