const ApiSubset = require('../ApiSubset')
const {
    CSiteMessage
} = require('../../ffi/typedefs')

class Feedback extends ApiSubset {

    constructor(protocolHandler, options) {
        super(options, 'hermes_protocol_handler_sound_feedback_facade', protocolHandler)
    }

    destroy () {
        this.call('hermes_drop_sound_feedback_facade', this.facade)
    }
}

Feedback.prototype.publishEvents = {
    notification_on: {
        fullEventName: 'hermes_sound_feedback_publish_toggle_on',
        forgedStruct: CSiteMessage
    },
    notification_off: {
        fullEventName: 'hermes_sound_feedback_publish_toggle_off',
        forgedStruct: CSiteMessage
    }
}

module.exports = Feedback