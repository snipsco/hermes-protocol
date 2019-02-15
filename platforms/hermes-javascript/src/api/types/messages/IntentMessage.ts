import { slotType, slotType_legacy } from '../enums'

export type IntentMessage = {
    sessionId: string,
    siteId: string,
    input: string,
    customData?: string,
    intent: {
        intentName: string,
        confidenceScore: number
    },
    asrTokens: [
        {
            value: string,
            confidence: number,
            rangeStart: number,
            rangeEnd: number,
            time: {
                start: number,
                end: number
            }
        }[]
    ],
    slots: {
        confidenceScore: number,
        rawValue: string,
        range: {
            start: number,
            end: number
        },
        entity: string,
        slotName: string,
        value: {
            kind: slotType,
            // Wildcard
            value: any
        }
    }[]
}

export type IntentMessageLegacy = {
    session_id: string,
    custom_data?: string,
    site_id: string,
    input: string,
    intent: {
        intent_name: string,
        confidence_score: number
    },
    asr_tokens: [
        {
            value: string,
            confidence: number,
            range_start: number,
            range_end: number,
            time: {
                start: number,
                end: number
            }
        }[]
    ],
    slots: {
        confidence_score: number,
        raw_value: string,
        range_start: number,
        range_end: number
        entity: string,
        slot_name: string,
        value: {
            value_type: slotType_legacy,
            // Wildcard
            value: any
        }
    }[]
}
