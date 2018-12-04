const ApiSubset = require('../ApiSubset')
const {
    InjectionRequestMessage
} = require('../../casts')
const {
    CInjectionRequestMessage
} = require('../../ffi/typedefs')

class Injection extends ApiSubset {

    constructor(protocolHandler, options) {
        super(options, 'hermes_protocol_handler_injection_facade', protocolHandler)
    }

    destroy () {
        this.call('hermes_drop_injection_facade', this.facade)
    }
}

Injection.prototype.publishEvents = {
    injection_request: {
        fullEventName: 'hermes_injection_publish_injection_request',
        messageClass: InjectionRequestMessage,
        forgedStruct: CInjectionRequestMessage
    }
}

module.exports = Injection