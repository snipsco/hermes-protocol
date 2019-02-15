import { NotificationMessage, NotificationMessageLegacy } from './messages'

export namespace FeedbackTypes {
    export type publishMessagesList<API> = {
        notification_on: API extends 'json' ? NotificationMessage : NotificationMessageLegacy,
        notification_off: API extends 'json' ? NotificationMessage : NotificationMessageLegacy
    }
}
