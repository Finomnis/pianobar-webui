import asyncio
import json
import logging

from pianobarsocket import settings

_logger = logging.getLogger(__name__)


class EventReceiver:
    def __init__(self, websocket):
        self.websocket = websocket

    async def handle_connection(self, reader, writer):
        _logger.info("Event provider connected!")
        raw_data = await reader.read()
        _logger.info("New event received ...")
        data = json.loads(raw_data.decode("utf-8"))

        await self.websocket.handle_event(data["command"], data["state"])

    async def run(self):
        server = await asyncio.start_server(
            self.handle_connection, "localhost", port=settings.EVENT_PORT
        )
        await server.serve_forever()
