import { injectionKind } from '../enums'

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
