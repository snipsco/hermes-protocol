export const LIB_ENV_FOLDER =
    process.env.HERMES_TEST_ENVIRONMENT === 'release' ?
        'release' :
    'debug'
