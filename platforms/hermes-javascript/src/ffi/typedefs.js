const ref = require('ref')
const Struct = require('ref-struct')

const coerce = ref.coerceType
const pointer = ref.refType

/* Misc. */

const CStringArray = Struct({
  data: coerce('char **'),
  size: coerce('int')
})

const CMqttOptions = Struct({
  broker_address: coerce('char *'),
  username: coerce('char *'),
  password: coerce('char *'),
  tls_hostname: coerce('char *'),
  tls_ca_file: pointer(CStringArray),
  tls_ca_path: pointer(CStringArray),
  tls_client_key: coerce('char *'),
  tls_client_cert: coerce('char *'),
  tls_disable_root_store: coerce('uchar'),
})

const misc = {
  CMqttOptions,
  CStringArray
}

/* Protocol Handler */

const CProtocolHandler = Struct({
  handler: coerce('void *'),
  user_data: coerce('void *')
})

/* Exports */

module.exports = {
  CProtocolHandler,
  ...misc
}
