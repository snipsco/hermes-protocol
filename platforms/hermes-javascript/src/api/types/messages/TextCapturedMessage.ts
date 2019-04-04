export interface TextCapturedMessage {
    text: string
    siteId?: string
    sessionId?: string
    tokens?: [
        {
            /** The value of the token. */
            value: string,
            /** Confidence of the token, between 0 and 1, 1 being confident. */
            confidence: number,
            /** The start range in which the token is in the original input. */
            rangeStart: number,
            /** The end range in which the token is in the original input. */
            rangeEnd: number,
            /** Time when this token was detected. */
            time: {
                /** Start time. */
                start: number,
                /** End time. */
                end: number
            }
        }
    ]
    likelihood?: number
    seconds?: number
}
