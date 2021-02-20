import asyncio
import logging

_logger = logging.getLogger(__name__)


class JsonRpcConnection:
    def __init__(self, websocket, path, rpcRequestHandler):
        self.websocket = websocket
        self.path = path
        self.rpcRequestHandler = rpcRequestHandler

    async def sendSignal(self, signal_name, **kwargs):
        _logger.info(f"SEND SIGNAL {kwargs}")

    async def run(self):
        _logger.info(f"RUN")
        await asyncio.sleep(10)
