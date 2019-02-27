export type ContinueSessionMessage = {
    sessionId: string,
    text?: string,
    intentFilter?: string[],
    customData?: string,
    slot?: string,
    sendIntentNotRecognized?: boolean
}

export type ContinueSessionMessageLegacy = {
    session_id: string,
    text?: string,
    intent_filter?: string[],
    custom_data?: string,
    slot?: string,
    send_intent_not_recognized?: boolean
}
