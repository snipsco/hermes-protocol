import { injectionKind, injectionKind_legacy } from '../enums'

export type InjectionRequestMessage = {
    id: string,
    crossLanguage?: string,
    lexicon: {
        [key: string]: string[]
    },
    operations: [
        injectionKind,
        {
            [key: string]: string[]
        }
    ][]
}

export type InjectionRequestMessageLegacy = {
    id: string,
    cross_language?: string,
    lexicon: {
        [key: string]: string[]
    },
    operations: {
        kind: injectionKind_legacy,
        values: {
            [key: string]: string[]
        }
    }
}
