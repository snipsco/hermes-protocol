import {
    InjectionRequestMessage,
    InjectionStatusMessage,
} from './messages'

export namespace InjectionTypes {
    export type publishMessagesList = {
        injection_request: InjectionRequestMessage,
        injection_status_request: null
    }

    export type subscribeMessagesList = {
        injection_status: InjectionStatusMessage
    }
}
