import { HermesOptions } from './HermesOptions'

export type SubscribeEventDescriptor = {
    fullEventName: string,
    additionalArguments?: (eventName: string) => any[]
}

export type PublishEventDescriptor = {
    fullEventName: string
}

export type MessageListener<T = {[key: string]: any}> = (message: T, ...args: any[]) => void
export type FFIFunctionCall = (functionName: string, ...args: any[]) => void
export type SubsetConstructor<Subset> = new (
    protocolHandler: Buffer,
    call: FFIFunctionCall,
    options: HermesOptions
) => Subset
