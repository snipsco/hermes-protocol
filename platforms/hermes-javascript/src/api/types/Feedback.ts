import { NotificationMessage } from './messages'

export namespace FeedbackTypes {

    /**
     * The name and type of message that the Feedback subset can publish.
     */
    export type publishMessagesList = {
        notification_on: NotificationMessage,
        notification_off: NotificationMessage
    }
}
