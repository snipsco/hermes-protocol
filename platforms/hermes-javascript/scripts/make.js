#!/usr/bin/env node

const path = require('path')
const fs = require('fs')
const { execSync } = require('child_process')
const tmp = require('tmp')
const chalk = require('chalk')

const REPO_URL = 'https://github.com/snipsco/hermes-protocol'
const REPO_NAME = 'hermes-protocol'

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
const LIB_PATH = baseFolder =>
    path.join(baseFolder, 'target/release/libhermes_mqtt_ffi' + LIB_EXTENSION)
const LIB_DIST = path.join(__dirname, '../libhermes_mqtt_ffi' + LIB_EXTENSION)

const errorStyle = chalk.bold.red
const successStyle = chalk.bold.green
const cmdStyle = chalk.bold
const logError = err => console.error(errorStyle(err+'\n'))
const logCmd = cmd => console.log(cmdStyle(cmd+'\n'))

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
    let output = '!> Error during the build!\n'

    if(!cmd) {
        console.error(output)
        return console.error(error)
    }

    output += 'Command [' + cmd +'] exited '

    if(status) {
        output += 'with error code (' + status + ')'
    } else if(signal) {
        output += 'when receiving signal (' + signal + ')'
    }

    logError(output)
}

console.log(chalk.green.bold('>> Building hermes dynamic library from scratch.'))
console.log(chalk.yellow.bold('/!\\ Requirements: git, rust, cargo and node.js >= 8'))
console.log('\n')

const tmpDir = tmp.dirSync()
try {
    logCmd('> Cloning hermes repository.')
    cmd(`git clone ${REPO_URL}`, {
        cwd: tmpDir.name
    })
    const repoFolder = path.resolve(tmpDir.name, REPO_NAME)
    logCmd('Repository cloned @ ' + repoFolder)
    cmd('git submodule update --init --recursive', {
        cwd: repoFolder
    })

    logCmd('> Building the dynamic library from sources.')
    cmd('cargo build -p hermes-mqtt-ffi --release', {
        cwd: repoFolder
    })

    logCmd('> Copy the generated dynamic library file to the current working folder.')
    fs.copyFileSync(LIB_PATH(repoFolder), LIB_DIST)

    console.log(successStyle('> Done!'))
} catch(error) {
    printCmdError(error)
}