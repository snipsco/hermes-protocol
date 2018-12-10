#!/usr/bin/env node
// const os = require('os')

// TODO : check platform for existing dynamic lib.

// If not, then require make to build from scratch.

if(process.env.HERMES_BUILD_FROM_SOURCES)
    require('./make')