const fs = require('fs')
const path = require('path')
const { spawn } = require('child_process')
const mqtt = require('mqtt')
const { Hermes } = require('../../dist')
const { LIB_ENV_FOLDER } = require('../constants')

// Log segfaults
const SegfaultHandler = require('segfault-handler')
SegfaultHandler.registerHandler('crash.log')

let mosquitto, hermes

beforeAll(async () => {
    console.log('Launching secure mosquitto')
    mosquitto = spawn('mosquitto', ['-c', path.join(__dirname, 'tls/mosquitto-tls.conf')], { stdio: 'ignore' })
    console.log('Mosquitto server using TLS configuration is ready!')
    try {
      hermes = new Hermes({
        libraryPath: path.join(__dirname, `../../../../target/${LIB_ENV_FOLDER}/libhermes_mqtt_ffi`),
        logs: true,
        address: 'localhost:18886',
        username: 'foo',
        password: 'bar',
        tls_hostname: 'localhost',
        tls_ca_file: [path.join(__dirname, 'tls/ca.cert')],
        // tls_ca_path: [path.join(__dirname, 'tls')],
        tls_client_key: path.join(__dirname, 'tls/client.key'),
        tls_client_cert: path.join(__dirname, 'tls/client.cert')
      })
    } catch (error) {
      console.error(error)
    }
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

it('should connect to a secure TLS mosquitto server', done => {
    const message = {
        siteId: 'default',
        sessionId: 'session id'
    }
    const expectedMessage = {
        custom_data: null,
        input: null,
        session_id: 'session id',
        site_id: 'default'
    }
    const client = mqtt.connect('mqtts://localhost:18886', {
        username: 'foo',
        password: 'bar',
        ca: fs.readFileSync(path.join(__dirname, 'tls/ca.cert')),
        key: fs.readFileSync(path.join(__dirname, 'tls/client.key')),
        cert: fs.readFileSync(path.join(__dirname, 'tls/client.cert')),
        rejectUnauthorized: false
    })
    client.on('error', function(err) {
        client.end({ force: true })
        done()
        throw new Error(err)
    })
    client.on('connect', function () {
        client.publish('hermes/dialogueManager/intentNotRecognized', JSON.stringify(message))
    })

    const callback = function(msg) {
        expect(expectedMessage).toEqual(msg)
        client.end()
        done()
    }
    hermes.dialog().on('intent_not_recognized', callback)
})
