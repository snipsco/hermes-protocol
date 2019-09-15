# hermes-reason


[![CircleCI](https://circleci.com/gh/yourgithubhandle/hermes-reason/tree/master.svg?style=svg)](https://circleci.com/gh/yourgithubhandle/hermes-reason/tree/master)


**Contains the following libraries and executables:**

```
hermes-reason@0.0.0
│
├─test/
│   name:    TestHermesReason.exe
│   main:    TestHermesReason
│   require: hermes-reason.lib
│
├─library/
│   library name: hermes-reason.lib
│   namespace:    HermesReason
│   require:
│
└─executable/
    name:    HermesReasonApp.exe
    main:    HermesReasonApp
    require: hermes-reason.lib
```

## Developing:

```
npm install -g esy
git clone <this-repo>
esy install
esy build
```

## Running Binary:

After building the project, you can run the main binary that is produced.

```
esy x HermesReasonExample.exe
```

## Running Tests:

```
# Runs the "test" command in `package.json`.
esy test
```
