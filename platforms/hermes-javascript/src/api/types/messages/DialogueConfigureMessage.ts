export type DialogueConfigureMessage = {
    siteId?: string,
    intents?: {
        intentId: string,
        enable: boolean
    }[]
}
