import {
    InjectionRequestMessage,
    InjectionRequestMessageLegacy,
    InjectionStatusMessage,
    InjectionStatusMessageLegacy
} from './messages'

export namespace InjectionTypes {
    export type publishMessagesList = {
        injection_request: InjectionRequestMessage | InjectionRequestMessageLegacy,
        injection_status_request: null
    }

    export type subscribeMessagesList = {
        injection_status: InjectionStatusMessage & InjectionStatusMessageLegacy
    }
}
