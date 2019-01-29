#!/usr/bin/env node

const path = require('path')
const fs = require('fs')
const { execSync } = require('child_process')
const tmp = require('tmp')

const {
    LIB_PATH,
    LIB_DIST,
    REPO_URL,
    REPO_NAME,
    logger
} = require('./utils')

const cmd = (command, options = {}) => {
    try {
        execSync(command, {
            stdio: 'inherit',
            ...options
        })
    } catch (error) {
        error.cmd = command
        throw error
    }
}

function printCmdError (error) {
    const { cmd, status, signal } = error
    let output = 'Error during the build!\n'

    if(!cmd) {
        logger.error(output)
        logger.error(error)
        return
    }

    output += 'Command [' + cmd +'] exited '

    if(status) {
        output += 'with error code (' + status + ')'
    } else if(signal) {
        output += 'when receiving signal (' + signal + ')'
    }

    logger.error(output + '\n')
}

logger.success('- Building hermes dynamic library from scratch.')
logger.warning('/!\\ Requirements: git, rust, cargo and node.js >= 8\n')

const tmpDir = tmp.dirSync()
try {
    logger.cmd('- Cloning hermes repository.\n')
    cmd(`git clone ${REPO_URL}`, {
        cwd: tmpDir.name
    })
    const repoFolder = path.resolve(tmpDir.name, REPO_NAME)
    logger.cmd('Repository cloned @ ' + repoFolder + '\n')
    cmd('git submodule update --init --recursive', {
        cwd: repoFolder
    })

    logger.cmd('- Building the dynamic library from sources.\n')
    cmd('cargo build -p hermes-mqtt-ffi --release', {
        cwd: repoFolder
    })

    logger.cmd('- Copy the generated dynamic library file to the current working folder.\n')
    fs.copyFileSync(LIB_PATH(repoFolder), LIB_DIST)

    logger.success('> Done!\n')
} catch(error) {
    printCmdError(error)
}
