import ApiSubset from '../ApiSubset'
import {
    InjectionRequestMessage
} from '../../casts'
import {
    CInjectionRequestMessage,
    CInjectionStatusMessage
} from '../../ffi/typedefs'

export default class Injection extends ApiSubset {

    constructor(protocolHandler, call) {
        super(protocolHandler, call, 'hermes_protocol_handler_injection_facade')
    }

    publishEvents = {
        injection_request: {
            fullEventName: 'hermes_injection_publish_injection_request',
            messageClass: InjectionRequestMessage,
            forgedStruct: CInjectionRequestMessage
        },
        injection_status_request: {
            fullEventName: 'hermes_injection_publish_injection_status_request',
            forgedStruct: null
        }
    }

    subscribeEvents = {
        injection_status: {
            fullEventName: 'hermes_injection_subscribe_injection_status',
            dropEventName: 'hermes_drop_injection_status_message',
            messageStruct: CInjectionStatusMessage
        }
    }

    destroy () {
        this.call('hermes_drop_injection_facade', this.facade)
    }
}