module.exports = {
    LIB_ENV_FOLDER: process.env.HERMES_TEST_ENVIRONMENT === 'release' ? 'release' : 'debug',
}