/* eslint-disable no-console */

import { spawn } from 'child_process'
import path from 'path'
import mqtt, { MqttClient } from 'mqtt'
import {
  Hermes,
  Dialog,
  Injection,
  Feedback,
  Audio,
  FlowIntentAction,
  PlayBytesMessage,
  RegisterSoundMessage,
  Tts
} from '../../dist'
import {
  getFreePort,
  setupSubscriberJsonTest,
  setupPublisherJsonTest,
  wait
} from './tools'
import {
  LIB_ENV_FOLDER
} from '../constants'

/* Setup */

let
  mosquitto: any,
  mosquittoPort: string,
  client: MqttClient,
  hermes: Hermes,
  dialog: Dialog,
  injection: Injection,
  feedback: Feedback,
  audio: Audio,
  tts: Tts

const robustnessTestsTimeout = 60000
const robustnessIterations = 500
const robustnessDelay = 5

beforeAll(async () => {
  mosquittoPort = '' + await getFreePort()
  console.log('Launching mosquitto on port [' + mosquittoPort + ']')
  mosquitto = spawn('mosquitto', ['-p', mosquittoPort, '-v'], { stdio: 'ignore' })
  console.log('Mosquitto ready!')
  try {
    hermes = new Hermes({
      libraryPath: path.join(__dirname, `../../../../target/${LIB_ENV_FOLDER}/libhermes_mqtt_ffi`),
      logs: true,
      address: `localhost:${mosquittoPort}`
    })
    dialog = hermes.dialog()
    injection = hermes.injection()
    feedback = hermes.feedback()
    audio = hermes.audio()
    tts = hermes.tts()
  } catch (error) {
    console.error(error)
  }
})

beforeEach(done => {
  client = mqtt.connect(`mqtt://localhost:${mosquittoPort}`)
  client.on('connect', function () {
    done()
  })
  client.on('error', function(err) {
    client.end(true)
    throw err
  })
})

afterEach(() => {
  client.end(true)
})

afterAll(done => {
  if(hermes)
    hermes.destroy()
  console.log('Hermes destroyed.')
  setTimeout(() => {
    mosquitto.kill()
    console.log('Mosquitto killed.')
    done()
  }, 500)
})


/* Publish */

it('[dialog] should publish a start session event', () => {
  return setupPublisherJsonTest({
    client,
    facade: dialog,
    json: require('./messages/StartSession.json'),
    hermesTopic: 'hermes/dialogueManager/startSession',
    facadePublication: 'start_session'
  })
})
it('[dialog] should publish a continue session event', () => {
  return setupPublisherJsonTest({
    client,
    facade: dialog,
    json: require('./messages/ContinueSession.json'),
    hermesTopic: 'hermes/dialogueManager/continueSession',
    facadePublication: 'continue_session'
  })
})
it('[dialog] should publish an end session event', () => {
  return setupPublisherJsonTest({
    client,
    facade: dialog,
    json: require('./messages/EndSession.json'),
    hermesTopic: 'hermes/dialogueManager/endSession',
    facadePublication: 'end_session'
  })
})
it('[dialog] should publish a configure event', () => {
  return setupPublisherJsonTest({
    client,
    facade: dialog,
    json: require('./messages/DialogueConfigure.json'),
    hermesTopic: 'hermes/dialogueManager/configure',
    facadePublication: 'configure'
  })
})

// Injection

it('[injection] should publish an injection request event', () => {
  return setupPublisherJsonTest({
    client,
    facade: injection,
    json: require('./messages/InjectionRequest.json'),
    hermesTopic: 'hermes/injection/perform',
    facadePublication: 'injection_request'
  })
})

it('[injection] should publish an injection status request event', () => {
  return setupPublisherJsonTest({
    client,
    facade: injection,
    json: null,
    hermesTopic: 'hermes/injection/statusRequest',
    facadePublication: 'injection_status_request'
  })
})

// Feedback

it('[feedback] should publish an notification sound on event', () => {
  return setupPublisherJsonTest({
    client,
    facade: feedback,
    json: require('./messages/SiteMessage.json'),
    hermesTopic: 'hermes/feedback/sound/toggleOn',
    facadePublication: 'notification_on'
  })
})

it('[feedback] should publish an notification sound off event', () => {
  return setupPublisherJsonTest({
    client,
    facade: feedback,
    json: require('./messages/SiteMessage.json'),
    hermesTopic: 'hermes/feedback/sound/toggleOff',
    facadePublication: 'notification_off'
  })
})

// Audio

it('[audio] should publish an audio playback event', () => {
  const wavBuffer = Buffer.from([0x00, 0x01, 0x02, 0x03])
  const hermesTopic = 'hermes/audioServer/default/playBytes/8ewnjksdf093jb42'

  return new Promise(resolve => {
    const message: PlayBytesMessage = {
      id: '8ewnjksdf093jb42',
      siteId: 'default',
      wavBytes: wavBuffer.toString('base64'),
      wavBytesLen: wavBuffer.length
    }
    client.subscribe(hermesTopic, function() {
        audio.publish('play_audio', message)
    })
    client.on('message', (topic, messageBuffer) => {
        expect(wavBuffer).toEqual(messageBuffer)
        client.unsubscribe(hermesTopic)
        resolve()
    })
  })
})

// TTS

it('[tts] should publish an audio playback that the tts will be able to use later on', () => {
  const wavBuffer = Buffer.from([0x00, 0x01, 0x02, 0x03])
  const hermesTopic = 'hermes/tts/registerSound/foobar'

  return new Promise(resolve => {
    const message: RegisterSoundMessage = {
      soundId: 'foobar',
      wavSound: wavBuffer.toString('base64')
    }
    client.subscribe(hermesTopic, function() {
      tts.publish('register_sound', message)
    })
    client.on('message', (topic, messageBuffer) => {
      expect(wavBuffer).toEqual(messageBuffer)
      client.unsubscribe(hermesTopic)
      resolve()
    })
  })
})

/* Subscribe */

it('[dialog] should receive and parse a session started event', () => {
  return setupSubscriberJsonTest({
    client,
    facade: dialog,
    json: require('./messages/SessionStarted.json'),
    hermesTopic: 'hermes/dialogueManager/sessionStarted',
    facadeSubscription: 'session_started'
  })
})

it('[dialog] should receive and parse a session queued event', () => {
  return setupSubscriberJsonTest({
    client,
    facade: dialog,
    json: require('./messages/SessionQueued.json'),
    hermesTopic: 'hermes/dialogueManager/sessionQueued',
    facadeSubscription: 'session_queued'
  })
})

it('[dialog] should receive and parse a session ended event', () => {
  return setupSubscriberJsonTest({
    client,
    facade: dialog,
    json: require('./messages/SessionEnded.json'),
    hermesTopic: 'hermes/dialogueManager/sessionEnded',
    facadeSubscription: 'session_ended'
  })
})

it('[dialog] should receive and parse an intent not recognized event', () => {
  return setupSubscriberJsonTest({
    client,
    facade: dialog,
    json: require('./messages/IntentNotRecognized.json'),
    hermesTopic: 'hermes/dialogueManager/intentNotRecognized',
    facadeSubscription: 'intent_not_recognized'
  })
})

it('[dialog] should receive events related to any intent', () => {
  return setupSubscriberJsonTest({
    client,
    facade: dialog,
    json: require('./messages/Intent.json'),
    hermesTopic: 'hermes/intent/intentA',
    facadeSubscription: 'intents'
  })
})

it('[dialog] should receive events related to a specific intent', () => {
  return setupSubscriberJsonTest({
    client,
    facade: dialog,
    json: require('./messages/Intent.json'),
    hermesTopic: 'hermes/intent/anIntent',
    facadeSubscription: 'intent/anIntent'
  })
})

// Injection

it('[injection] should receive events related to an injection status', () => {
  return setupSubscriberJsonTest({
    client,
    facade: injection,
    json: require('./messages/InjectionStatus.json'),
    hermesTopic: 'hermes/injection/status',
    facadeSubscription: 'injection_status'
  })
})

// Audio

it('[audio] should receive events when a sound playback finished', async () => {
  await setupSubscriberJsonTest({
    client,
    facade: audio,
    json: require('./messages/PlayFinished.json'),
    hermesTopic: 'hermes/audioServer/default/playFinished',
    facadeSubscription: 'play_finished/default'
  })
})

it('[audio] should receive events when a sound playback finished', async () => {
  await setupSubscriberJsonTest({
    client,
    facade: audio,
    json: require('./messages/PlayFinished.json'),
    hermesTopic: 'hermes/audioServer/default/playFinished',
    facadeSubscription: 'play_finished_all'
  })
})

/* Robustness tests */

it(`[dialog] should should publish a start session message at least ${robustnessIterations} times`, () => {
  const json = require('./messages/StartSession.json')
  let counter = 0
  return new Promise(resolve => {
      client.subscribe('hermes/dialogueManager/startSession', function() {
        dialog.publish('start_session', json)
      })
      client.on('message', (_, messageBuffer) => {
        let message
        try {
            message = JSON.parse(messageBuffer.toString())
        } catch (e) {
            message = null
        }
        if(message) {
            expect(json).toMatchObject(message)
        } else {
            expect(null).toEqual(message)
        }
        if(++counter >= robustnessIterations) {
          client.unsubscribe('hermes/dialogueManager/startSession')
          resolve()
        } else {
          wait(robustnessDelay).then(() => dialog.publish('start_session', json))
        }
      })
  })
}, robustnessTestsTimeout)

it(`[dialog] should should publish an end session message at least ${robustnessIterations} times`, () => {
  const json = require('./messages/EndSession.json')
  let counter = 0
  return new Promise(resolve => {
      client.subscribe('hermes/dialogueManager/endSession', function() {
        dialog.publish('end_session', json)
      })
      client.on('message', (topic, messageBuffer) => {
        let message
        try {
            message = JSON.parse(messageBuffer.toString())
        } catch (e) {
            message = null
        }
        if(message) {
            expect(json).toMatchObject(message)
        } else {
            expect(null).toEqual(message)
        }
        if(++counter >= robustnessIterations) {
          client.unsubscribe('hermes/dialogueManager/endSession')
          resolve()
        } else {
          wait(robustnessDelay).then(() => dialog.publish('end_session', json))
        }
      })
  })
}, robustnessTestsTimeout)

it(`[dialog] should receive a session started message at least ${robustnessIterations} times`, async () => {
  for (let i = 0; i < robustnessIterations; i++) {
    await setupSubscriberJsonTest({
      client,
      facade: dialog,
      json: require('./messages/SessionStarted.json'),
      hermesTopic: 'hermes/dialogueManager/sessionStarted',
      facadeSubscription: 'session_started'
    })
    await wait(robustnessDelay)
  }
}, robustnessTestsTimeout)

it(`[dialog] should receive a session ended message at least ${robustnessIterations} times`, async () => {
  for (let i = 0; i < robustnessIterations; i++) {
    await setupSubscriberJsonTest({
      client,
      facade: dialog,
      json: require('./messages/SessionEnded.json'),
      hermesTopic: 'hermes/dialogueManager/sessionEnded',
      facadeSubscription: 'session_ended'
    })
    await wait(robustnessDelay)
  }
}, robustnessTestsTimeout)

it(`[dialog] should receive an intent message at least ${robustnessIterations} times`, () => {
  return new Promise(resolve => {
    let counter = 0
    const json = require('./messages/Intent.json')

    dialog.on('intent/anIntent', msg => {
      expect(msg).toMatchObject(json)
      if(++counter >= robustnessDelay)
          return resolve()
      client.publish('hermes/intent/anIntent', JSON.stringify(json))
    })
    client.publish('hermes/intent/anIntent', JSON.stringify(json))
  })
}, robustnessTestsTimeout)

it(`[dialog] should perform one round of dialog flow at least ${robustnessIterations} times`, () => {
  return new Promise(resolve => {
    let counter = 0
    const intentJson = require('./messages/Intent.json')
    const sessionEndedJson = require('./messages/SessionEnded.json')

    dialog.flow('anIntent', (msg, flow) => {
      expect(msg).toMatchObject(intentJson)
      flow.end()
    })

    client.subscribe('hermes/dialogueManager/continueSession', () => {
      client.subscribe('hermes/dialogueManager/endSession', () => {
        client.publish('hermes/intent/anIntent', JSON.stringify(intentJson))
      })
    })
    client.on('message', topic => {
      if(topic === 'hermes/dialogueManager/endSession') {
        client.publish('hermes/dialogueManager/sessionEnded', JSON.stringify(sessionEndedJson))
        if(++counter >= robustnessIterations)
          return resolve()
        client.publish('hermes/intent/anIntent', JSON.stringify(intentJson))
      }
    })
  })
}, robustnessTestsTimeout)

it(`[dialog] should perform at least ${robustnessIterations} rounds of dialog flow`, () => {
  return new Promise(resolve => {
    let counter = 0
    const intentJson = require('./messages/Intent.json')
    const sessionEndedJson = require('./messages/SessionEnded.json')

    const loop: FlowIntentAction = (msg, flow) => {
      expect(msg).toMatchObject(intentJson)
      if(++counter >= robustnessIterations) {
        flow.end()
      } else {
        flow.continue('anIntent', loop, { slotFiller: 'slot' })
      }
    }
    dialog.flow('anIntent', loop)

    client.subscribe('hermes/dialogueManager/continueSession', () => {
      client.subscribe('hermes/dialogueManager/endSession', () => {
        client.publish('hermes/intent/anIntent', JSON.stringify(intentJson))
      })
    })
    client.on('message', (topic: string, messageBuffer: Buffer) => {
      let message
      try { message = JSON.parse(messageBuffer.toString()) } catch (e) { message = null }
      if(topic === 'hermes/dialogueManager/continueSession') {
        expect({
          customData: null,
          intentFilter: [
            'anIntent',
          ],
          sendIntentNotRecognized: false,
          sessionId: '677a2717-7ac8-44f8-9013-db2222f7923d',
          slot: 'slot',
          text: '',
        }).toMatchObject(message)
        return client.publish('hermes/intent/anIntent', JSON.stringify(intentJson))
      }
      if(topic === 'hermes/dialogueManager/endSession') {
        expect({
          sessionId: '677a2717-7ac8-44f8-9013-db2222f7923d',
          text: ''
        }).toMatchObject(message)
        client.unsubscribe('hermes/dialogueManager/continueSession')
        client.unsubscribe('hermes/dialogueManager/endSession')
        client.publish('hermes/dialogueManager/sessionEnded', JSON.stringify(sessionEndedJson))
        return resolve()
      }
    })
  })
}, robustnessTestsTimeout)
