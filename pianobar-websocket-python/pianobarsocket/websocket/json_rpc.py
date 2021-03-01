import asyncio
import logging
import json

_logger = logging.getLogger(__name__)


class JsonRpcConnection:
    def __init__(self, websocket, path, rpcRequestHandler):
        self.websocket = websocket
        self.path = path
        self.rpcRequestHandler = rpcRequestHandler

    async def sendSignal(self, signal_name, **kwargs):
        await self.websocket.send(
            json.dumps({"jsonrpc": "2.0", "method": signal_name, "params": kwargs})
        )

    async def run(self):
        while True:
            msg = await self.websocket.recv()
            print(f"Message received: {msg}")
            raise NotImplementedError()
