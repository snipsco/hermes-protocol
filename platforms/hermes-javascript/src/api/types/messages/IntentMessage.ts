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

export type NluSlot<T extends slotType = slotType> = {
    confidenceScore: number,
    rawValue: string,
    range: {
        start: number,
        end: number
    },
    entity: string,
    slotName: string,
    value:
        T extends slotType.custom ? CustomSlotValue<T> :
        T extends slotType.number ? NumberSlotValue<T> :
        T extends slotType.ordinal ? OrdinalSlotValue<T> :
        T extends slotType.instantTime ? InstantTimeSlotValue<T> :
        T extends slotType.timeInterval ? TimeIntervalSlotValue<T> :
        T extends slotType.amountOfMoney ? AmountOfMoneySlotValue<T> :
        T extends slotType.temperature ? TemperatureSlotValue<T> :
        T extends slotType.duration ? DurationSlotValue<T> :
        T extends slotType.musicAlbum ? MusicAlbumSlotValue<T> :
        T extends slotType.musicArtist ? MusicArtistSlotValue<T> :
        T extends slotType.musicTrack ? MusicTrackSlotValue<T> :
        never
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
    asrConfidence: number,
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
