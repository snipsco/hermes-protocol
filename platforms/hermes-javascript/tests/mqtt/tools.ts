import { createServer, AddressInfo } from 'net'
import camelcase from 'camelcase'
import ApiSubset from '../../dist/api/ApiSubset'

export const wait = (time: number) => new Promise(resolve => setTimeout(resolve, time))

export const getFreePort: () => Promise<number> = () => {
    return new Promise((resolve, reject) => {
        const server = createServer()
        server.on('error', err => {
            reject(err)
        })
        server.on('listening', () => {
            const port = (server.address() as AddressInfo).port
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

type PublisherJsonTestArgs= {
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
    // eslint-disable-next-line
    return new Promise(async resolve => {
        facade.once(facadeSubscription, message => {
            expect(message).toMatchObject(json)
            resolve()
        })
        await wait(5)
        client.publish(hermesTopic, JSON.stringify(json))
    })
}