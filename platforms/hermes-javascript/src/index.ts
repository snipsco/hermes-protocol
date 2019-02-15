import { Hermes, HermesOptions, HermesAPI } from './api'
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
export const withHermes = function<API extends HermesAPI = 'json'>(
    context: (hermes: Hermes<API>,
    done: Done
) => void, opts?: HermesOptions) {
    const hermes = new Hermes<API>(opts)
    const keepAliveRef = tools.keepAlive(60000)
    const done: Done = () => {
        hermes.destroy()
        tools.killKeepAlive(keepAliveRef)
    }
    context(hermes, done)
}