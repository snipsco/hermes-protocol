const { spawn } = require('child_process')
const mqtt = require('mqtt')
const { Hermes, tools } = require('../../src')
const {
  getFreePort,
  camelize,
  setupSubscriberTest,
  setupPublisherTest
} = require('./tools')

/* Setup */

let mosquittoPort
let mosquitto
let hermes
let dialog
// let injection
let client

// Log segfaults
const SegfaultHandler = require('segfault-handler')
SegfaultHandler.registerHandler('crash.log')

beforeAll(async () => {
  mosquittoPort = await getFreePort()
  mosquitto = spawn('mosquitto', ['-p', mosquittoPort, '-v'])
  hermes = new Hermes({ logs: true, address: `localhost:${mosquittoPort}` })
  tools.keepAlive(20)
  dialog = hermes.dialog()
  // injection = hermes.injection()
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
  tools.killKeepAlive()
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

it('should publish a start session event', () => {
  return setupPublisherTest({
    client,
    dialog,
    publishedJson: require('./hermesPublished/StartSession.json'),
    expectedJson: require('./mqttPublished/StartSession.json'),
    hermesTopic: 'hermes/dialogueManager/startSession',
    dialogPublication: 'start_session'
  })
})
it('should publish a continue session event', () => {
  return setupPublisherTest({
    client,
    dialog,
    publishedJson: require('./hermesPublished/ContinueSession.json'),
    hermesTopic: 'hermes/dialogueManager/continueSession',
    dialogPublication: 'continue_session'
  })
})
it('should publish an end session event', () => {
  return setupPublisherTest({
    client,
    dialog,
    publishedJson: require('./hermesPublished/EndSession.json'),
    hermesTopic: 'hermes/dialogueManager/endSession',
    dialogPublication: 'end_session'
  })
})

/* Subscribe */

it('should receive and parse a session started event', () => {
  return setupSubscriberTest({
    client,
    dialog,
    mqttJson: require('./mqttPublished/SessionStarted.json'),
    hermesTopic: 'hermes/dialogueManager/sessionStarted',
    dialogSubscription: 'session_started'
  })
})

it('should receive and parse a session queued event', () => {
  return setupSubscriberTest({
    client,
    dialog,
    mqttJson: require('./mqttPublished/SessionQueued.json'),
    hermesTopic: 'hermes/dialogueManager/sessionQueued',
    dialogSubscription: 'session_queued'
  })
})

it('should receive and parse a session ended event', () => {
  return setupSubscriberTest({
    client,
    dialog,
    mqttJson: require('./mqttPublished/SessionEnded.json'),
    expectedJson: require('./hermesPublished/SessionEnded.json'),
    hermesTopic: 'hermes/dialogueManager/sessionEnded',
    dialogSubscription: 'session_ended'
  })
})

it('should receive and parse an intent not recognized event', () => {
  return setupSubscriberTest({
    client,
    dialog,
    mqttJson: require('./mqttPublished/IntentNotRecognized.json'),
    hermesTopic: 'hermes/dialogueManager/intentNotRecognized',
    dialogSubscription: 'intent_not_recognized'
  })
})

it('should receive events related to any intent', () => {
  return setupSubscriberTest({
    client,
    dialog,
    mqttJson: require('./mqttPublished/Intent.json'),
    expectedJson: require('./hermesPublished/Intent.json'),
    hermesTopic: 'hermes/intent/intentA',
    dialogSubscription: 'intents'
  })
})

it('should receive events related to a specific intent', () => {
  return setupSubscriberTest({
    client,
    dialog,
    mqttJson: require('./mqttPublished/Intent.json'),
    expectedJson: require('./hermesPublished/Intent.json'),
    hermesTopic: 'hermes/intent/anIntent',
    dialogSubscription: 'intent/anIntent'
  })
})