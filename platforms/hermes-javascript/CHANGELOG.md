## [0.4.1](https://github.com/snipsco/hermes-protocol/compare/js/0.4.0...0.4.1) (2019-10-17)

* **Chore**: Use node 12 compatible dependencies (ffi & ref).


# [0.4.0](https://github.com/snipsco/hermes-protocol/compare/js/0.3.14...0.4.0) (2019-10-03)


### Features

* **injection:** add injection complete & reset bindings ([0bae1b5](https://github.com/snipsco/hermes-protocol/commit/0bae1b5))
* **nlu:** Add alternatives for intents and slots. ([8e16503](https://github.com/snipsco/hermes-protocol/commit/8e16503))
* **nlu:** make slot alternatives field optional ([d93a84e](https://github.com/snipsco/hermes-protocol/commit/d93a84e))


### BREAKING CHANGES

* **nlu:** new mandatory field "alternatives" added to the NluSlot type.



## [0.3.14](https://github.com/snipsco/hermes-protocol/compare/js/0.3.13...0.3.14) (2019-07-18)

* **Chore**: Bump hermes mqtt version to `0.67.0` ([8b4068f](https://github.com/snipsco/hermes-protocol/commit/8b4068f))

## [0.3.13](https://github.com/snipsco/hermes-protocol/compare/js/0.3.12...0.3.13) (2019-07-18)


### Features

* **message:** add session ended component field on timeouts ([fb20157](https://github.com/snipsco/hermes-protocol/commit/fb20157))



## [0.3.12](https://github.com/snipsco/hermes-protocol/compare/js/0.3.11...0.3.12) (2019-06-17)


### Bug Fixes

* **postinstall:** bad raspbian os check ([f77553c](https://github.com/snipsco/hermes-protocol/commit/f77553c))



## [0.3.11](https://github.com/snipsco/hermes-protocol/compare/js/0.3.10...0.3.11) (2019-05-18)


### Bug Fixes

* **js:** prevent potential tiny memory leak ([d148945](https://github.com/snipsco/hermes-protocol/commit/d148945))



## [0.3.10](https://github.com/snipsco/hermes-protocol/compare/js/0.3.9...0.3.10) (2019-05-17)


### Bug Fixes

* **dialog flow:** returning an object did not actually impact the message ([802d0b6](https://github.com/snipsco/hermes-protocol/commit/802d0b6))



## [0.3.9](https://github.com/snipsco/hermes-protocol/compare/js/0.3.8...0.3.9) (2019-05-16)

* **Chore**: Types and enums have their specific import location now: `import { /* ... */ } from 'hermes-javascript/type'`


## [0.3.8](https://github.com/snipsco/hermes-protocol/compare/js/0.3.7...0.3.8) (2019-05-14)


### Bug Fixes

* **postinstall:** handle different lib name for windows users ([6e86721](https://github.com/snipsco/hermes-protocol/commit/6e86721))



## [0.3.7](https://github.com/snipsco/hermes-protocol/compare/js/0.3.6...0.3.7) (2019-03-27)


### Bug Fixes

* **js:** missing PercentageSlotValue type definition ([b428d5f](https://github.com/snipsco/hermes-protocol/commit/b428d5f))



## [0.3.6](https://github.com/snipsco/hermes-protocol/compare/js/0.3.5...0.3.6) (2019-03-18)


### Bug Fixes

* **js:** Rename configure message instantName to intentId ([23d204b](https://github.com/snipsco/hermes-protocol/commit/23d204b))


### BREAKING CHANGES

* **js:** breaks configure message publishing



## [0.3.5](https://github.com/snipsco/hermes-protocol/compare/js/0.3.4...0.3.5) (2019-03-05)

**Upgraded dependencies.**

## [0.3.4](https://github.com/snipsco/hermes-protocol/compare/js/0.3.3...0.3.4) (2019-03-05)


### Features

* **js:** add asr confidence (intent message) ([8225a2b](https://github.com/snipsco/hermes-protocol/commit/8225a2b))
* **js:** dialogue subset, add root intents configuration ([bb9a9fc](https://github.com/snipsco/hermes-protocol/commit/bb9a9fc))



## [0.3.2](https://github.com/snipsco/hermes-protocol/compare/js/0.3.1...0.3.2) (2019-03-04)


### Bug Fixes

* **js:** postinstall step triggering for CI builds ([b3b71fd](https://github.com/snipsco/hermes-protocol/commit/b3b71fd))
* **js:** sessionFlow, do not publish a continue session message ([7c6acba](https://github.com/snipsco/hermes-protocol/commit/7c6acba))



## [0.3.1](https://github.com/snipsco/hermes-protocol/compare/js/0.3.0...0.3.1) (2019-02-27)


### Bug Fixes

* **js:** case sensitive tts import ([fe8af56](https://github.com/snipsco/hermes-protocol/commit/fe8af56))



# [0.3.0](https://github.com/snipsco/hermes-protocol/compare/js/0.2.5...0.3.0) (2019-02-27)


### Bug Fixes

* build from sources now checks out the correct tagged version ([27fa193](https://github.com/snipsco/hermes-protocol/commit/27fa193))


### Code Refactoring

* **js:** remove legacy api ([0baf712](https://github.com/snipsco/hermes-protocol/commit/0baf712))


### Features

* **js:** add slot filling api ([edfb902](https://github.com/snipsco/hermes-protocol/commit/edfb902))
* **js:** new tts facade that allows to register a tts sound ([e2dd93c](https://github.com/snipsco/hermes-protocol/commit/e2dd93c))


### BREAKING CHANGES

* **js:** breaks actions relying on the old messages format



