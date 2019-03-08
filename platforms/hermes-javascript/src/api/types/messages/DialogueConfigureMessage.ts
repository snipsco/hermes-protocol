export type DialogueConfigureMessage = {
    siteId?: string,
    intents?: {
        intentName: string,
        enable: boolean
    }[]
}
