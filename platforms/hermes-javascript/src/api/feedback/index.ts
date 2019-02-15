import ApiSubset from '../ApiSubset'
import { CSiteMessage } from '../../ffi/typedefs'
import { FeedbackTypes } from '../types'
export default class Feedback<API> extends ApiSubset<API> {

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
    publishMessagesList: FeedbackTypes.publishMessagesList<API>

    destroy () {
        this.call('hermes_drop_sound_feedback_facade', this.facade)
    }
}