import { createServer } from 'net'
import camelcase from 'camelcase'
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

/* Publish */

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

type PublisherJsonTestArgs = {
    client: any,
    facade: ApiSubset,
    json: any,
    hermesTopic: string,
    facadePublication: string
}
export const setupPublisherJsonTest = ({
    client,
    facade,
    json,
    hermesTopic,
    facadePublication
} : PublisherJsonTestArgs) => {
    json = json && { ...json }
    return new Promise(resolve => {
        client.subscribe(hermesTopic, function() {
            facade.publish(facadePublication, json)
        })
        client.on('message', (topic, messageBuffer) => {
            let message
            try {
                message = JSON.parse(messageBuffer.toString())
            } catch (e) {
                message = null
            }
            if(message) {
                expect(message).toMatchObject(json)
            } else {
                expect(message).toEqual(null)
            }
            client.unsubscribe(hermesTopic)
            resolve()
        })
    })
}

/* Subscribe */

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

type SubscriberJsonTestArgs = {
    client: any,
    facade: ApiSubset,
    json: any,
    hermesTopic: string,
    facadeSubscription: string
}
export const setupSubscriberJsonTest = ({
    client,
    facade,
    json,
    hermesTopic,
    facadeSubscription
} : SubscriberJsonTestArgs) => {
    return new Promise(async resolve => {
        facade.once(facadeSubscription, message => {
            expect(message).toMatchObject(json)
            resolve()
        })
        await wait(5)
        client.publish(hermesTopic, JSON.stringify(json))
    })
}