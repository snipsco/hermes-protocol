export interface DialogueConfigureMessage {
    /** Id of the site to configure. */
    siteId?: string
    /** An array of intents to enable / disable. */
    intents?: {
        /** Id of the intent. */
        intentId: string,
        /** Enable or diable the intent. */
        enable: boolean
    }[]
}
