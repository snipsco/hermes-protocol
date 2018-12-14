import { createServer } from 'net'
import camelcase from 'camelcase'
// eslint-disable-next-line
import ApiSubset from '../../dist/api/ApiSubset'

export const wait = (time) => new Promise(resolve => setTimeout(resolve, time))

export const getFreePort = () => {
    return new Promise((resolve, reject) => {
        const server = createServer()
        server.on('error', err => {
            reject(err)
        })
        server.on('listening', () => {
            const port = (server.address() as any).port
            server.close()
            resolve(port)
        })
        server.listen()
    })
}

export const camelize = item => {
    if(typeof item !== 'object' || !item)
        return item
    if(item instanceof Array) {
        return item.map(value => camelize(value))
    }
    (Object as any).entries(item).forEach(([ key, value ]) => {
        const camelizedKey = camelcase(key)
        const isSameKey = key === camelizedKey
        item[camelizedKey] = camelize(value)
        if(!isSameKey) {
            delete item[key]
        }
    })
    return item
}

type PublisherTestArgs = {
    client: any,
    facade: ApiSubset,
    publishedJson: any,
    expectedJson?: any,
    hermesTopic: string,
    facadePublication: string
}
export const setupPublisherTest = ({
    client,
    facade,
    publishedJson,
    expectedJson,
    hermesTopic,
    facadePublication
} : PublisherTestArgs) => {
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
                const expected = expectedJson || camelize(publishedJson)
                expect(expected).toMatchObject(message)
            } else {
                expect(null).toEqual(message)
            }
            client.unsubscribe(hermesTopic)
            resolve()
        })
    })
}

type SubscriberTestArgs = {
    client: any,
    facade: ApiSubset,
    mqttJson: any,
    expectedJson?: any,
    hermesTopic: string,
    facadeSubscription: string
}
export const setupSubscriberTest = ({
    client,
    facade,
    mqttJson,
    expectedJson = null,
    hermesTopic,
    facadeSubscription
} : SubscriberTestArgs) => {
    mqttJson = { ...mqttJson }
    return new Promise(async resolve => {
        facade.once(facadeSubscription, message => {
            const expected = expectedJson || camelize(mqttJson)
            const received = expectedJson ? message : camelize(message)
            expect(received).toMatchObject(expected)
            resolve()
        })
        await wait(5)
        client.publish(hermesTopic, JSON.stringify(mqttJson))
    })
}