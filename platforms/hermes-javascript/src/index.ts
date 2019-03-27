/**
 * @module api
 */

import { Hermes, HermesOptions } from './api'
import * as tools from './tools'

export * from './tools'
export * from './api'

/**
 * Will stop Hermes gracefully when called.
 */
export type Done = () => void

/**
 * Sets an event loop up and initializes the Hermes class.
 *
 * @param context - The wrapped context function.
 * @param opts - Options used to create the Hermes instance.
 */
export const withHermes = function(
    context: (hermes: Hermes, done: Done) => void,
    opts?: HermesOptions
) {
    const hermes = new Hermes(opts)
    const keepAliveRef = tools.keepAlive(60000)
    const done: Done = () => {
        hermes.destroy()
        tools.killKeepAlive(keepAliveRef)
    }
    context(hermes, done)
}