import {
    InjectionRequestMessage,
    InjectionRequestMessageLegacy,
    InjectionStatusMessage,
    InjectionStatusMessageLegacy
} from './messages'

export namespace InjectionTypes {
    export type publishMessagesList<API> = {
        injection_request: API extends 'json' ? InjectionRequestMessage : InjectionRequestMessageLegacy,
        injection_status_request: null
    }

    export type subscribeMessagesList<API> = {
        injection_status: API extends 'json' ? InjectionStatusMessage : InjectionStatusMessageLegacy
    }
}
