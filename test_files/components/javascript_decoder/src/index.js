import { log } from "component:decoder/host"

const name = "Javascript Test Decoder"

export const decoder = {
    init() {
        log(`${name} initialized`)
    },
    name() {
        return name
    },
    formatSignalValue(value) {
        value + "!"
    }
}


