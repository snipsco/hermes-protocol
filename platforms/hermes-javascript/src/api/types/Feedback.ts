export namespace FeedbackTypes {

    export type NotificationMessage = {
        siteId: string,
        sessionId?: string
    } | {
        site_id: string,
        session_id?: string
    }

    export type publishMessagesList = {
        notification_on: NotificationMessage,
        notification_off: NotificationMessage
    }

}