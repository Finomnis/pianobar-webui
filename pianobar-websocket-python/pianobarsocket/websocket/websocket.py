import asyncio
import websockets
import logging

from typing import Set

from pianobarsocket import settings
from .event_receiver import EventReceiver
from .json_rpc import JsonRpcConnection

_logger = logging.getLogger(__name__)


class Websocket:
    def __init__(self):
        self.state = dict()
        self.clients: Set[JsonRpcConnection] = set()

    async def handle_rpc_requests(self):
        raise NotImplementedError()

    async def websocket_connection_handler(
        self, websocket: websockets.WebSocketServerProtocol, path: str
    ):
        (host, port) = websocket.remote_address
        _logger.info(f"'{host}:{port}' connected")

        # Initiate JsonRPC connection
        connection = JsonRpcConnection(websocket, path, self.handle_rpc_requests)

        # Send current state as a welcome message
        await connection.sendSignal("event", command=None, state=self.state)

        # Register connection
        self.clients.add(connection)

        try:
            # Communicate
            await connection.run()
        except websockets.ConnectionClosedOK:
            _logger.info(f"'{host}:{port}' disconnected")
        except Exception as e:
            _logger.warning(f"'{host}:{port}' lost connection: {e}")
        finally:
            # Unregister connection
            self.clients.remove(connection)

    async def websocket_server(self):
        _logger.info("Starting websocket server ...")
        await websockets.serve(
            self.websocket_connection_handler, port=settings.WEBSOCKET_PORT
        )

    async def main(self):
        _logger.info("Starting services ...")
        await asyncio.gather(
            EventReceiver(self).run(),
            self.websocket_server(),
        )

    async def handle_event(self, command, state):
        self.state = state

        tasks = []
        for client in self.clients:
            tasks.append(
                asyncio.create_task(
                    client.sendSignal("event", command=command, state=self.state)
                )
            )

        for task in tasks:
            try:
                await task
            except Exception as e:
                _logger.warn(f"Exception while sending event to client: {e}")


def run():
    logging.basicConfig(level=logging.INFO)
    asyncio.run(Websocket().main())
