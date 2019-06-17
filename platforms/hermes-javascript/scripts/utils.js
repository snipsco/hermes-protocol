const path = require('path')
const chalk = require('chalk')
const fs = require('fs')

const REPO_URL = 'https://github.com/snipsco/hermes-protocol'
const REPO_NAME = 'hermes-protocol'
const hermesMqttVersion = require('../package.json')['hermes-mqtt-version']

const LIB_EXTENSION = {
    linux:  '.so',
    linux2: '.so',
    sunos:  '.so',
    solaris: '.so',
    freebsd: '.so',
    openbsd: '.so',
    darwin: '.dylib',
    mac:    '.dylib',
    win32:  '.dll'
}[process.platform]

const LIB_NAME = {
    linux:  'libhermes_mqtt_ffi',
    linux2: 'libhermes_mqtt_ffi',
    sunos:  'libhermes_mqtt_ffi',
    solaris: 'libhermes_mqtt_ffi',
    freebsd: 'libhermes_mqtt_ffi',
    openbsd: 'libhermes_mqtt_ffi',
    darwin: 'libhermes_mqtt_ffi',
    mac:    'libhermes_mqtt_ffi',
    win32:  'hermes_mqtt_ffi.dll'
}[process.platform]

const LIB_PATH = baseFolder =>
    path.join(baseFolder, 'target/release/' + LIB_NAME + LIB_EXTENSION)
const LIB_DIST = path.join(__dirname, '../libhermes_mqtt_ffi' + LIB_EXTENSION)

const errorStyle = chalk.bold.red
const warningStyle = chalk.bold.yellow
const successStyle = chalk.bold.green
const cmdStyle = chalk.bold
const logError = str => console.error(errorStyle(str))
const logWarning = str => console.log(warningStyle(str))
const logCmd = str => console.log(cmdStyle(str))
const logSuccess = str => console.log(successStyle(str))

function osIsRaspbian () {
    if(!fs.existsSync('/etc/os-release'))
        return false
    return (
        (
            fs.readFileSync('/etc/os-release', 'utf-8')
              .indexOf('NAME="Raspbian')
        ) >= 0
    )
}

module.exports = {
    LIB_PATH,
    LIB_DIST,
    LIB_EXTENSION,
    REPO_URL,
    REPO_NAME,
    hermesMqttVersion,
    osIsRaspbian,
    logger: {
        error: logError,
        warning: logWarning,
        success: logSuccess,
        cmd: logCmd
    }
}
