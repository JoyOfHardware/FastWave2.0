import { log } from "component:javascript-decoder/host"

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


