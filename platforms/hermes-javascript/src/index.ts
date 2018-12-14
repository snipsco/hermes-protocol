// eslint-disable-next-line
import { Hermes, HermesOptions } from './api'
import * as tools from './tools'

export * from './tools'
export * from './api'

export type Done = () => void

/**
 * Sets up an event loop and initializes the Hermes class.
 *
 * @param {*} context The wrapped context function.
 * @param {*} opts Hermes options.
 */
export const withHermes = function(context: (hermes: Hermes, done: Done) => void, opts?: HermesOptions) {
    const hermes = new Hermes(opts)
    const keepAliveRef = tools.keepAlive(20)
    const done: Done = () => {
        hermes.destroy()
        tools.killKeepAlive(keepAliveRef)
    }
    context(hermes, done)
}