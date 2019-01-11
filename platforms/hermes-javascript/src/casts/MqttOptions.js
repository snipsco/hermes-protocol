const ref = require('ref')
const Casteable = require('./Casteable')
const StringArray = require('./StringArray')
const { CMqttOptions } = require('../ffi/typedefs')

class IntentMessage extends Casteable {
    constructor(args) {
        super(args)
        this.type = CMqttOptions
    }

    forge() {
        return super.forge(this.type, {
            tls_ca_file: tls_ca_file => tls_ca_file && new StringArray(tls_ca_file).forge() || ref.NULL,
            tls_ca_path: tls_ca_path => tls_ca_path && new StringArray(tls_ca_path).forge() || ref.NULL
        })
    }
}

module.exports = IntentMessage
