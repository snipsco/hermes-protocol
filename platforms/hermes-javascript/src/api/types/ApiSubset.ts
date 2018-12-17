export type SubscribeEventDescriptor = {
    fullEventName: string,
    dropEventName: string,
    messageStruct?: any, // CStruct
    messageClass?: any, // Casteable
    additionalArguments?: (eventName: string) => any[]
}

export type PublishEventDescriptor = {
    fullEventName: string,
    messageClass?: any,
    forgedStruct?: any,
    forgeOptions?: { [key: string]: (property: string) => any }
}

export type MessageListener = (message?: { [key: string]: any }) => void
export type FFIFunctionCall = (functionName: string, ...args: any[]) => void
export type SubsetConstructor<Subset> = new (protocolHandler: Buffer, call: FFIFunctionCall) => Subset