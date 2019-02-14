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

    export type InjectionStatusMessage = {
        lastInjectionDate: string
    } & {
        last_injection_date: string
    }

    export type publishMessagesList = {
        injection_request: InjectionRequestMessage,
        injection_status_request: null
    }

    export type subscribeMessagesList = {
        injection_status: InjectionStatusMessage
    }
}