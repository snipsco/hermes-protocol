export type IntentNotRecognizedMessage = {
    sessionId: string,
    siteId: string,
    input: string,
    confidence_score: number
    customData?: string,
}
