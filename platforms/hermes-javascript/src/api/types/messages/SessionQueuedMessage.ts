export type SessionQueuedMessage = {
    sessionId: string,
    siteId: string,
    customData?: string
}

export type SessionQueuedMessageLegacy = {
    session_id: string,
    site_id: string,
    custom_data?: string
}
