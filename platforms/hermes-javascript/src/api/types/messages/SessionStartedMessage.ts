export type SessionStartedMessage = {
    sessionId: string,
    siteId: string
    customData?: string
}

export type SessionStartedMessageLegacy =  {
    session_id: string,
    site_id: string
    custom_data?: string
}
