export namespace InjectionTypes {

    /* Enums */
    export enum injectionKind {
        add = 'add',
        addFromVanilla = 'addFromVanilla'
    }
    export enum injectionKind_legacy {
        add = 1,
        addFromVanilla
    }

    /* Messages */

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
    } | {
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
    export type InjectionStatusRequestMessage = {
        lastInjectionDate: string
    } | {
        last_injection_date: string
    }

    export type publishMessagesList = {
        lastInjectionDate: InjectionRequestMessage,
        injection_status_request: InjectionStatusRequestMessage
    }

}