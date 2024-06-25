from .bindings.component import exports
from .bindings.component.imports import host

name = "Python Test Decoder"

class Decoder(exports.Decoder):
    def init(self) -> None:
        host.log(f"{name} initialized")

    def name(self) -> str:
        return name

    def format_signal_value(self, value: str) -> str:
        return str + "!"
