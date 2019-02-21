import { slotType, slotType_legacy, grain } from '../enums'

export type NluSlot<T extends slotType = slotType> = {
    confidenceScore: number,
    rawValue: string,
    range: {
        start: number,
        end: number
    },
    entity: string,
    slotName: string,
    value: {
        kind: T,
        value?:
            T extends (slotType.custom | slotType.instantTime | slotType.musicAlbum | slotType.musicArtist | slotType.musicTrack) ?
                string :
            T extends (slotType.number | slotType.ordinal | slotType.amountOfMoney | slotType.temperature | slotType.percentage) ?
                number :
            T extends (slotType.timeInterval | slotType.duration) ?
                never :
            any,
        grain?:
            T extends slotType.instantTime ?
                grain :
            never,
        precision?:
            T extends (slotType.instantTime | slotType.amountOfMoney | slotType.duration) ?
                'Exact' | 'Approximate' :
            never,
        from?:
            T extends slotType.timeInterval ?
                string :
            never,
        to?:
            T extends slotType.timeInterval ?
                string :
            never,
        unit?:
            T extends (slotType.amountOfMoney | slotType.temperature) ?
                string :
            never,
        years?:
            T extends slotType.duration ?
                number :
            never,
        quarters?:
            T extends slotType.duration ?
                number :
            never,
        months?:
            T extends slotType.duration ?
                number :
            never,
        weeks?:
            T extends slotType.duration ?
                number :
            never,
        days?:
            T extends slotType.duration ?
                number :
            never,
        hours?:
            T extends slotType.duration ?
                number :
            never,
        minutes?:
            T extends slotType.duration ?
                number :
            never,
        seconds?:
            T extends slotType.duration ?
                number :
            never
    }
}

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
        }[]?
    ],
    slots: NluSlot[]
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
        }[]?
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
