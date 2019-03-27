import { slotType, grain } from '../enums'

export type CustomSlotValue<T extends slotType.custom> = {
    kind: T,
    value: string
}
export type NumberSlotValue<T extends slotType.number> = {
    kind: T,
    value: number
}
export type OrdinalSlotValue<T extends slotType.ordinal> = {
    kind: T,
    value: number
}
export type InstantTimeSlotValue<T extends slotType.instantTime> = {
    kind: T,
    value: string,
    grain: grain,
    precision: 'Exact' | 'Approximate'
}
export type TimeIntervalSlotValue<T extends slotType.timeInterval> = {
    kind: T,
    from: string,
    to: string
}
export type AmountOfMoneySlotValue<T extends slotType.amountOfMoney> = {
    kind: T,
    value: number,
    precision: 'Exact' | 'Approximate',
    unit: string
}
export type TemperatureSlotValue<T extends slotType.temperature> = {
    kind: T,
    value: number,
    unit: 'celsius' | 'fahrenheit'
}
export type DurationSlotValue<T extends slotType.duration> = {
    kind: T,
    years: number,
    quarters: number,
    months: number,
    weeks: number,
    days: number,
    hours: number,
    minutes: number,
    seconds: number,
    precision: 'Exact' | 'Approximate'
}
export type MusicAlbumSlotValue<T extends slotType.musicAlbum> = {
    kind: T,
    value: string
}
export type MusicArtistSlotValue<T extends slotType.musicArtist> = {
    kind: T,
    value: string
}
export type MusicTrackSlotValue<T extends slotType.musicTrack> = {
    kind: T,
    value: string
}
export type PercentageSlotValue<T extends slotType.percentage> = {
    kind: T,
    value: number
}
export type NluSlot<T extends slotType = slotType> = {
    /** Confidence of the slot, between 0 and 1, 1 being confident. */
    confidenceScore: number,
    /**  The raw value of the slot, as is was in the input. */
    rawValue: string,
    /** The range where the slot can be found in the input. */
    range: {
        /** Beginning of the range (inclusive). */
        start: number,
        /** End of the range (exclusive). */
        end: number
    },
    /** The entity of the slot. */
    entity: string,
    /** The name of the slot. */
    slotName: string,
    /** The resolved value of the slot. */
    value:
        T extends slotType.custom ? CustomSlotValue<T> :
        T extends slotType.number ? NumberSlotValue<T> :
        T extends slotType.ordinal ? OrdinalSlotValue<T> :
        T extends slotType.instantTime ? InstantTimeSlotValue<T> :
        T extends slotType.timeInterval ? TimeIntervalSlotValue<T> :
        T extends slotType.amountOfMoney ? AmountOfMoneySlotValue<T> :
        T extends slotType.temperature ? TemperatureSlotValue<T> :
        T extends slotType.duration ? DurationSlotValue<T> :
        T extends slotType.percentage ? PercentageSlotValue<T> :
        T extends slotType.musicAlbum ? MusicAlbumSlotValue<T> :
        T extends slotType.musicArtist ? MusicArtistSlotValue<T> :
        T extends slotType.musicTrack ? MusicTrackSlotValue<T> :
        never
}

export interface IntentMessage {
    /** The current session id. */
    sessionId: string
    /** The site where the user interaction took place. */
    siteId: string
    /** The user input that has generated this intent. */
    input: string
    /** Custom data provided in the StartSessionMessage or a ContinueSessionMessage. */
    customData?: string
    /** Structured description of the intent classification. */
    intent: {
        /** The name of the detected intent. */
        intentName: string,
        /** The probability of the detection, between 0 and 1, 1 being sure. */
        confidenceScore: number
    }
    /** The level of confidence in the ASR prediction. */
    asrConfidence: number
    /**
     * Structured description of the tokens the ASR captured on for this intent.
     * The first level of arrays represents each invocation of the ASR,
     * the second one are the tokens captured.
     */
    asrTokens: [
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
        }[]?
    ]
    /** Structured description of the detected slots for this intent if any. */
    slots: NluSlot[]
}
