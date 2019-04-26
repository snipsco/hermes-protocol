#!/usr/bin/env node

const os = require('os')
const fs = require('fs')
const readline = require('readline')
const wretch = require('wretch').default

const { logger, osIsRaspbian, hermesMqttVersion, LIB_EXTENSION, LIB_DIST } = require('./utils')

const skipOnSelfInstall = process.cwd() === process.env.INIT_CWD

const request = wretch(`http://s3.amazonaws.com/snips/hermes-mqtt/${hermesMqttVersion}`).polyfills({
    fetch: require('node-fetch')
})

const OS_SUPPORTED = [
    'linux',
    'darwin'
]

const ARCHITECTURES_SUPPORTED = [
    'arm',
    'x64'
]

function getPlatformName () {
    // Linux or Darwin or Windows_NT
    const osType = os.type().toLowerCase()
    // 'arm', 'arm64', 'ia32', 'mips', 'mipsel', 'ppc', 'ppc64', 's390', 's390x', 'x32', 'x64'.
    const architecture = os.arch()
    // os version
    const osRelease = os.release()
    // cpu data
    const cpuInfo = os.cpus()[0]

    // Eliminate unsupported Operating Systems
    if (OS_SUPPORTED.indexOf(osType) < 0) {
        logger.warning(`There is no prebuilt dynamic library file available for your operating system (${osType}).\n`)
        return false
    }
    // Eliminate unsupported architectures
    if (ARCHITECTURES_SUPPORTED.indexOf(architecture) < 0) {
        logger.warning(`There is no prebuilt dynamic library file available for your hardware architecture (${architecture}).\n`)
        return false
    }

    // Specific to MacOS
    if(osType === 'darwin') {
        // Minimum osx version supported is El Capitan (15.0.0)
        if((+osRelease.split('.')[0]) < 15) {
            logger.warning(`There is no prebuilt dynamic library file available for your version of MacOS (${osRelease}).\n`)
            return false
        }
        // Do not support macbook with arm cpus (if they ever exist…)
        if(architecture === 'arm') {
            logger.warning('There is no prebuilt dynamic library file available for macintosh having ARM cpus.\n')
            return false
        }
        return 'macos-darwin-x86_64'
    }

    // Specific to Raspbian
    if(osIsRaspbian()) {
        // Support only ARMv6 & ARMv7
        if(
            cpuInfo.model.toLowerCase().indexOf('armv7') < 0 &&
            cpuInfo.model.toLocaleLowerCase().indexOf('armv6') < 0
        ) {
            logger.warning('Prebuilt dynamic library file is only available for ARMv7 cpus.\n')
            return false
        }
        return 'linux-raspbian-armhf'
    }

    // Arm but not raspbian
    if(architecture === 'arm')
        return false

    // Linux x86
    return 'linux-debian-x86_64'
}


if(skipOnSelfInstall || process.env.HERMES_SKIP_SHARED_LIB) {
    // Skipping post-install step on hermes-javascript self install or
    // if the HERMES_SKIP_SHARED_LIB environment variable is set.
} else {
    logger.cmd('- Checking platform support.')
    const platformName = getPlatformName()

    if(process.env.HERMES_BUILD_FROM_SOURCES || !platformName) {
        // If platform is not supported, then require make to build from scratch.
        require('./make')
    } else {
        logger.cmd('- Downloading the hermes mqtt dynamic library file…')
        logger.cmd('Target: ' + LIB_DIST)

        const libraryFileName = 'libhermes_mqtt_ffi' + LIB_EXTENSION

        request
            .url(`/${platformName}/${libraryFileName}`)
            .get()
            .res(res => {
                const length = res.headers.get('content-length')
                let downloaded = 0
                return new Promise(resolve => {
                    logger.cmd(`Downloaded ${downloaded / 1000} of ${length / 1000} KB`)
                    const fileStream = fs.createWriteStream(LIB_DIST)
                    res.body.pipe(fileStream)
                    const onChunk = chunk => {
                        downloaded += chunk.length
                        readline.moveCursor(process.stdout, 0, -1)
                        logger.cmd(`Downloaded ${downloaded / 1000} of ${length / 1000} KB`)
                    }
                    res.body.on('data', onChunk)
                    res.body.once('end', () => {
                        fileStream.removeListener('data', onChunk)
                        return resolve()
                    })
                })
            })
            .then(() => logger.success('> Done!'))
            .catch(error => {
                logger.error('An error occured while downloading the dynamic library.')
                logger.cmd('You can build hermes-javascript from source by setting the HERMES_BUILD_FROM_SOURCES environment variable to true.')
                logger.cmd('Example: env HERMES_BUILD_FROM_SOURCES=true npm install hermes-javascript\n')
                logger.error(error.message)
            })
    }
}
