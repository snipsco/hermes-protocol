import { NotificationMessage, NotificationMessageLegacy } from './messages'
import { HermesAPI } from '.'

export namespace FeedbackTypes {
    export type publishMessagesList<API extends HermesAPI = 'json'> = {
        notification_on: API extends 'json' ? NotificationMessage : NotificationMessageLegacy,
        notification_off: API extends 'json' ? NotificationMessage : NotificationMessageLegacy
    }
}
