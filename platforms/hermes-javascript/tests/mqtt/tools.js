const { createServer } = require('net')
const camelcase = require('camelcase')

const exportedObject = {
    getFreePort: () => {
        return new Promise((resolve, reject) => {
            const server = createServer()
            server.on('error', err => {
                reject(err)
            })
            server.on('listening', () => {
                const port = server.address().port
                server.close()
                resolve(port)
            })
            server.listen()
        })
    },
    camelize: item => {
        if(typeof item !== 'object' || !item)
            return item
        if(item instanceof Array) {
            return item.map(value => exportedObject.camelize(value))
        }
        Object.entries(item).forEach(([ key, value ]) => {
            const camelizedKey = camelcase(key)
            const isSameKey = key === camelizedKey
            item[camelizedKey] = exportedObject.camelize(value)
            if(!isSameKey) {
                delete item[key]
            }
        })
        return item
    },
    setupPublisherTest: ({
        client,
        facade,
        publishedJson,
        expectedJson,
        hermesTopic,
        facadePublication
    }) => {
        publishedJson = publishedJson && { ...publishedJson }
        return new Promise(resolve => {
            client.subscribe(hermesTopic, function() {
                facade.publish(facadePublication, publishedJson)
            })
            client.on('message', (topic, messageBuffer) => {
                let message
                try {
                    message = JSON.parse(messageBuffer.toString())
                } catch (e) {
                    message = null
                }
                if(message) {
                    const expected = expectedJson || exportedObject.camelize(publishedJson)
                    expect(expected).toMatchObject(message)
                } else {
                    expect(null).toEqual(message)
                }
                client.unsubscribe(hermesTopic)
                resolve()
            })
        })
    },
    setupSubscriberTest: ({
        client,
        facade,
        mqttJson,
        expectedJson,
        hermesTopic,
        facadeSubscription
    }) => {
        mqttJson = { ...mqttJson }
        return new Promise(resolve => {
            facade.once(facadeSubscription, message => {
                const expected = expectedJson || exportedObject.camelize(mqttJson)
                const received = expectedJson ? message : exportedObject.camelize(message)
                expect(received).toMatchObject(expected)
                resolve()
            })
            client.publish(hermesTopic, JSON.stringify(mqttJson))
        })
    }
}

module.exports = exportedObject