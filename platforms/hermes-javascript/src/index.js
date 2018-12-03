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
    const done = () => {
        hermes.destroy()
        tools.killKeepAlive()
    }
    context(hermes, done)
    tools.keepAlive(20)
}

module.exports = {
    Hermes,
    withHermes,
    tools
}