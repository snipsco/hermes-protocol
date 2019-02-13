import ApiSubset from '../ApiSubset'
import { CSiteMessage } from '../../ffi/typedefs'

export default class Feedback extends ApiSubset {

    constructor(protocolHandler, call, options) {
        super(protocolHandler, call, options, 'hermes_protocol_handler_sound_feedback_facade')
    }

    publishEvents = {
        notification_on: {
            fullEventName: 'hermes_sound_feedback_publish_toggle_on',
            forgedStruct: CSiteMessage
        },
        notification_off: {
            fullEventName: 'hermes_sound_feedback_publish_toggle_off',
            forgedStruct: CSiteMessage
        }
    }

    destroy () {
        this.call('hermes_drop_sound_feedback_facade', this.facade)
    }
}