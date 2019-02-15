export enum terminationType {
    nominal = 'nominal',
    siteUnavailable = 'siteUnavailable',
    abortedByUser = 'abortedByUser',
    intentNotRecognized = 'intentNotRecognized',
    timeout = 'timeout',
    error = 'error'
}

export enum terminationType_legacy {
    nominal = 1,
    unavailable,
    abortedByUser,
    intentNotRecognized,
    timeout,
    error
}
