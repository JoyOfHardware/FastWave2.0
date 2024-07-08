from .bindings.component import exports
from .bindings.component.imports import host

name = "Python Test Decoder"

class Decoder(exports.Decoder):
    def init(self) -> None:
        # @TODO it panics with error `7: 0xae8683 - libcomponentize_py_runtime.so!componentize-py#Dispatch`
        # - see https://github.com/bytecodealliance/componentize-py/blob/e20d9e6706ff1421cd8001449acb51eb9c87d0c6/runtime/src/lib.rs#L404
        # host.log(f"{name} initialized")
        return None

    def name(self) -> str:
        return name

    def format_signal_value(self, value: str) -> str:
        return value + "!"
