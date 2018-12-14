import { spawn } from 'child_process'
import path from 'path'
import mqtt from 'mqtt'
// eslint-disable-next-line
import { Hermes, Dialog, Injection, Feedback } from '../../dist'
import {
  getFreePort,
  camelize,
  setupSubscriberTest,
  setupPublisherTest,
  wait
} from './tools'
import {
  LIB_ENV_FOLDER
} from '../constants'

/* Setup */

let
  mosquitto,
  mosquittoPort,
  client,
  hermes: Hermes,
  dialog: Dialog,
  injection: Injection,
  feedback: Feedback

const robustnessTestsTimeout = 60000
const robustnessIterations = 500
const robustnessDelay = 5

// Log segfaults
const SegfaultHandler = require('segfault-handler')
SegfaultHandler.registerHandler('crash.log')

beforeAll(async () => {
  mosquittoPort = await getFreePort()
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
    client.end({ force: true })
    throw new Error(err)
  })
})

afterEach(() => {
  client.end({ force: true })
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

/* Tools */

it('[tools] should camelize stuff properly', () => {
  const uncamelized = {
    snake_case_int: 1,
    snake_case_string: 'toto_titi',
    snake_case_object: {
      nested_snake_case_array: [
        1, 2, { in_array_item: 'test_test' }
      ]
    }
  }
  const camelized = camelize(uncamelized)
  expect(camelized).toEqual({
    snakeCaseInt: 1,
    snakeCaseString: 'toto_titi',
    snakeCaseObject: {
      nestedSnakeCaseArray: [
        1, 2, { inArrayItem: 'test_test' }
      ]
    }
  })
})

/* Publish */

it('[dialog] should publish a start session event', () => {
  return setupPublisherTest({
    client,
    facade: dialog,
    publishedJson: require('./hermesPublished/StartSession.json'),
    expectedJson: require('./mqttPublished/StartSession.json'),
    hermesTopic: 'hermes/dialogueManager/startSession',
    facadePublication: 'start_session'
  })
})
it('[dialog] should publish a continue session event', () => {
  return setupPublisherTest({
    client,
    facade: dialog,
    publishedJson: require('./hermesPublished/ContinueSession.json'),
    hermesTopic: 'hermes/dialogueManager/continueSession',
    facadePublication: 'continue_session'
  })
})
it('[dialog] should publish an end session event', () => {
  return setupPublisherTest({
    client,
    facade: dialog,
    publishedJson: require('./hermesPublished/EndSession.json'),
    hermesTopic: 'hermes/dialogueManager/endSession',
    facadePublication: 'end_session'
  })
})

// Injection

it('[injection] should publish an injection request event', () => {
  return setupPublisherTest({
    client,
    facade: injection,
    publishedJson: require('./hermesPublished/InjectionRequest.json'),
    expectedJson: require('./mqttPublished/InjectionRequest.json'),
    hermesTopic: 'hermes/injection/perform',
    facadePublication: 'injection_request'
  })
})

it('[injection] should publish an injection status request event', () => {
  return setupPublisherTest({
    client,
    facade: injection,
    publishedJson: null,
    hermesTopic: 'hermes/injection/statusRequest',
    facadePublication: 'injection_status_request'
  })
})

// Feedback

it('[feedback] should publish an notification sound on event', () => {
  return setupPublisherTest({
    client,
    facade: feedback,
    publishedJson: require('./hermesPublished/SiteMessage.json'),
    hermesTopic: 'hermes/feedback/sound/toggleOn',
    facadePublication: 'notification_on'
  })
})

it('[feedback] should publish an notification sound off event', () => {
  return setupPublisherTest({
    client,
    facade: feedback,
    publishedJson: require('./hermesPublished/SiteMessage.json'),
    hermesTopic: 'hermes/feedback/sound/toggleOff',
    facadePublication: 'notification_off'
  })
})

/* Subscribe */

it('[dialog] should receive and parse a session started event', () => {
  return setupSubscriberTest({
    client,
    facade: dialog,
    mqttJson: require('./mqttPublished/SessionStarted.json'),
    hermesTopic: 'hermes/dialogueManager/sessionStarted',
    facadeSubscription: 'session_started'
  })
})

it('[dialog] should receive and parse a session queued event', () => {
  return setupSubscriberTest({
    client,
    facade: dialog,
    mqttJson: require('./mqttPublished/SessionQueued.json'),
    hermesTopic: 'hermes/dialogueManager/sessionQueued',
    facadeSubscription: 'session_queued'
  })
})

it('[dialog] should receive and parse a session ended event', () => {
  return setupSubscriberTest({
    client,
    facade: dialog,
    mqttJson: require('./mqttPublished/SessionEnded.json'),
    expectedJson: require('./hermesPublished/SessionEnded.json'),
    hermesTopic: 'hermes/dialogueManager/sessionEnded',
    facadeSubscription: 'session_ended'
  })
})

it('[dialog] should receive and parse an intent not recognized event', () => {
  return setupSubscriberTest({
    client,
    facade: dialog,
    mqttJson: require('./mqttPublished/IntentNotRecognized.json'),
    hermesTopic: 'hermes/dialogueManager/intentNotRecognized',
    facadeSubscription: 'intent_not_recognized'
  })
})

it('[dialog] should receive events related to any intent', () => {
  return setupSubscriberTest({
    client,
    facade: dialog,
    mqttJson: require('./mqttPublished/Intent.json'),
    expectedJson: require('./hermesPublished/Intent.json'),
    hermesTopic: 'hermes/intent/intentA',
    facadeSubscription: 'intents'
  })
})

it('[dialog] should receive events related to a specific intent', () => {
  return setupSubscriberTest({
    client,
    facade: dialog,
    mqttJson: require('./mqttPublished/Intent.json'),
    expectedJson: require('./hermesPublished/Intent.json'),
    hermesTopic: 'hermes/intent/anIntent',
    facadeSubscription: 'intent/anIntent'
  })
})

// Injection

it('[injection] should receive events related to an injection status', () => {
  return setupSubscriberTest({
    client,
    facade: injection,
    mqttJson: require('./mqttPublished/InjectionStatus.json'),
    hermesTopic: 'hermes/injection/status',
    facadeSubscription: 'injection_status'
  })
})

/* Robustness tests */

it(`[dialog] should should publish a start session message at least ${robustnessIterations} times`, () => {
  const publishedJson = { ...require('./hermesPublished/StartSession.json') }
  const expected = require('./mqttPublished/StartSession.json')
  let counter = 0
  return new Promise(resolve => {
      client.subscribe('hermes/dialogueManager/startSession', function() {
        dialog.publish('start_session', publishedJson)
      })
      client.on('message', (topic, messageBuffer) => {
        let message
        try {
            message = JSON.parse(messageBuffer.toString())
        } catch (e) {
            message = null
        }
        if(message) {
            expect(expected).toMatchObject(message)
        } else {
            expect(null).toEqual(message)
        }
        if(++counter >= robustnessIterations) {
          client.unsubscribe('hermes/dialogueManager/startSession')
          resolve()
        } else {
          wait(robustnessDelay).then(() => dialog.publish('start_session', publishedJson))
        }
      })
  })
}, robustnessTestsTimeout)

it(`[dialog] should should publish an end session message at least ${robustnessIterations} times`, () => {
  const publishedJson = { ...require('./hermesPublished/EndSession.json') }
  const expected = require('./mqttPublished/EndSession.json')
  let counter = 0
  return new Promise(resolve => {
      client.subscribe('hermes/dialogueManager/endSession', function() {
        dialog.publish('end_session', publishedJson)
      })
      client.on('message', (topic, messageBuffer) => {
        let message
        try {
            message = JSON.parse(messageBuffer.toString())
        } catch (e) {
            message = null
        }
        if(message) {
            expect(expected).toMatchObject(message)
        } else {
            expect(null).toEqual(message)
        }
        if(++counter >= robustnessIterations) {
          client.unsubscribe('hermes/dialogueManager/endSession')
          resolve()
        } else {
          wait(robustnessDelay).then(() => dialog.publish('end_session', publishedJson))
        }
      })
  })
}, robustnessTestsTimeout)

it(`[dialog] should receive a session started message at least ${robustnessIterations} times`, async () => {
  for (let i = 0; i < robustnessIterations; i++) {
    await setupSubscriberTest({
      client,
      facade: dialog,
      mqttJson: require('./mqttPublished/SessionStarted.json'),
      hermesTopic: 'hermes/dialogueManager/sessionStarted',
      facadeSubscription: 'session_started'
    })
    await wait(robustnessDelay)
  }
}, robustnessTestsTimeout)

it(`[dialog] should receive a session ended message at least ${robustnessIterations} times`, async () => {
  for (let i = 0; i < robustnessIterations; i++) {
    await setupSubscriberTest({
      client,
      facade: dialog,
      mqttJson: require('./mqttPublished/SessionEnded.json'),
      expectedJson: require('./hermesPublished/SessionEnded.json'),
      hermesTopic: 'hermes/dialogueManager/sessionEnded',
      facadeSubscription: 'session_ended'
    })
    await wait(robustnessDelay)
  }
}, robustnessTestsTimeout)

it(`[dialog] should receive an intent message at least ${robustnessIterations} times`, () => {
  return new Promise(resolve => {
    let counter = 0
    const mqttIntentMessageString = JSON.stringify(require('./mqttPublished/Intent.json'))
    const hermesIntentMessage = require('./hermesPublished/Intent.json')

    dialog.on('intent/anIntent', msg => {
      expect(msg).toMatchObject(hermesIntentMessage)
      if(++counter >= robustnessDelay)
          return resolve()
      client.publish('hermes/intent/anIntent', mqttIntentMessageString)
    })
    client.publish('hermes/intent/anIntent', mqttIntentMessageString)
  })
}, robustnessTestsTimeout)

it(`[dialog] should perform one round of dialog flow at least ${robustnessIterations} times`, () => {
  return new Promise(resolve => {
    let counter = 0
    const mqttIntentMessageString = JSON.stringify(require('./mqttPublished/Intent.json'))
    const hermesIntentMessage = require('./hermesPublished/Intent.json')
    const mqttSessionEndedMessageString = JSON.stringify(require('./mqttPublished/SessionEnded.json'))

    dialog.flow('anIntent', (msg, flow) => {
      expect(msg).toMatchObject(hermesIntentMessage)
      flow.end()
    })

    client.subscribe('hermes/dialogueManager/continueSession', () => {
      client.subscribe('hermes/dialogueManager/endSession', () => {
        client.publish('hermes/intent/anIntent', mqttIntentMessageString)
      })
    })
    client.on('message', topic => {
      if(topic === 'hermes/dialogueManager/endSession') {
        client.publish('hermes/dialogueManager/sessionEnded', mqttSessionEndedMessageString)
        if(++counter >= robustnessIterations)
          return resolve()
        client.publish('hermes/intent/anIntent', mqttIntentMessageString)
      }
    })
  })
}, robustnessTestsTimeout)

it(`[dialog] should perform at least ${robustnessIterations} rounds of dialog flow`, () => {
  return new Promise(resolve => {
    let counter = 0
    const mqttIntentMessageString = JSON.stringify(require('./mqttPublished/Intent.json'))
    const mqttSessionEndedMessageString = JSON.stringify(require('./mqttPublished/SessionEnded.json'))
    const hermesIntentMessage = require('./hermesPublished/Intent.json')

    const loop = (msg, flow) => {
      expect(msg).toMatchObject(hermesIntentMessage)
      if(++counter >= robustnessIterations) {
        flow.end()
      } else {
        flow.continue('anIntent', loop)
      }
    }
    dialog.flow('anIntent', loop)

    client.subscribe('hermes/dialogueManager/continueSession', () => {
      client.subscribe('hermes/dialogueManager/endSession', () => {
        client.publish('hermes/intent/anIntent', mqttIntentMessageString)
      })
    })
    client.on('message', (topic) => {
      if(topic === 'hermes/dialogueManager/continueSession') {
        return client.publish('hermes/intent/anIntent', mqttIntentMessageString)
      }
      if(topic === 'hermes/dialogueManager/endSession') {
        client.unsubscribe('hermes/dialogueManager/continueSession')
        client.unsubscribe('hermes/dialogueManager/endSession')
        client.publish('hermes/dialogueManager/sessionEnded', mqttSessionEndedMessageString)
        return resolve()
      }
    })
  })
}, robustnessTestsTimeout)