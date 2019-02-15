import { NotificationMessage, NotificationMessageLegacy } from './messages'

export namespace FeedbackTypes {
    export type publishMessagesList = {
        notification_on: NotificationMessage | NotificationMessageLegacy,
        notification_off: NotificationMessage | NotificationMessageLegacy
    }
}
