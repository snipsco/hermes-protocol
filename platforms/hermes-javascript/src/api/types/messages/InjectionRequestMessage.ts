import { injectionKind } from '../enums'

export interface InjectionRequestMessage {
    /** Id of the injection. */
    id: string
    /**
     * An extra language to compute the pronunciations for.
     * *Note: 'en' is the only options for now*
     * */
    crossLanguage?: string
    /** Custom pronunciations. Do not use if you don't know what this is about! */
    lexicon: {
        [key: string]: string[]
    }
    /** A list of entities mapping to a list of words to inject. */
    operations: [
        injectionKind,
        {
            [key: string]: (string | [string, number])[]
        }
    ][]
}
