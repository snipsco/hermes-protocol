export type IntentNotRecognizedMessage = {
    sessionId: string,
    siteId: string,
    input: string,
    confidence_score: number
    customData?: string,
}

export type IntentNotRecognizedMessageLegacy = {
    session_id: string,
    site_id: string,
    input: string,
    confidence_score: number
    custom_data?: string,
}
