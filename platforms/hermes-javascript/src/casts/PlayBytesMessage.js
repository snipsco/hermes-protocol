const ref = require('ref')
const Casteable = require('./Casteable')
const { CPlayBytesMessage } = require('../ffi/typedefs')

class PlayBytesMessage extends Casteable {
    constructor(args) {
        super(args)
        this.type = CPlayBytesMessage
    }

    fromBuffer(buffer) {
        return super.fromBuffer(buffer, {
            wav_bytes: (bytes, message) => ref.reinterpret(bytes, message.wav_bytes_len)
        })
    }

    forge() {
        return super.forge(this.type, {
            wav_bytes: bytes => bytes
        })
    }
}

module.exports = PlayBytesMessage
