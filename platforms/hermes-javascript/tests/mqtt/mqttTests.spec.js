const { spawn } = require('child_process')
const path = require('path')
const mqtt = require('mqtt')
const { Hermes } = require('../../src')
const {
  getFreePort,
  camelize,
  setupSubscriberTest,
  setupPublisherTest
} = require('./tools')
const {
  LIB_ENV_FOLDER
} = require('../constants')

/* Setup */

let
  mosquitto,
  mosquittoPort,
  client,
  hermes,
  dialog,
  injection,
  feedback

// Log segfaults
const SegfaultHandler = require('segfault-handler')
SegfaultHandler.registerHandler('crash.log')

beforeAll(async () => {
  mosquittoPort = await getFreePort()
  mosquitto = spawn('mosquitto', ['-p', mosquittoPort, '-v'])
  hermes = new Hermes({
    libraryPath: path.join(__dirname, `../../../../target/${LIB_ENV_FOLDER}/libhermes_mqtt_ffi`),
    logs: true,
    address: `localhost:${mosquittoPort}`
  })
  dialog = hermes.dialog()
  injection = hermes.injection()
  feedback = hermes.feedback()
})

beforeEach(done => {
  client = mqtt.connect(`mqtt://localhost:${mosquittoPort}`)
  client.on('connect', function () {
    done()
  })
  client.on('error', function(err) {
    client.end()
    throw new Error(err)
  })
})

afterEach(() => {
  client.end({ force: true })
})

afterAll(() => {
  hermes.destroy()
  mosquitto.kill()
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