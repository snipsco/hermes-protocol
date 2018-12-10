const Hermes = require('./api')
const tools = require('./tools')

/**
 * Sets up an event loop and initializes the Hermes class.
 *
 * @param {*} context The wrapped context function.
 * @param {*} opts Hermes options.
 */
const withHermes = function(context, opts) {
    const hermes = new Hermes(opts)
    const keepAliveRef = tools.keepAlive(20)
    const done = () => {
        hermes.destroy()
        tools.killKeepAlive(keepAliveRef)
    }
    context(hermes, done)
}

module.exports = {
    Hermes,
    withHermes,
    tools
}