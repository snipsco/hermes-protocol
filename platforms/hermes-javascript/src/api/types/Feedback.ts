import { NotificationMessage } from './messages'

export namespace FeedbackTypes {
    export type publishMessagesList = {
        notification_on: NotificationMessage,
        notification_off: NotificationMessage
    }
}
