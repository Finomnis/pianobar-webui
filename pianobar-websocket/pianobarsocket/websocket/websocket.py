import asyncio
import websockets
import logging

from pianobarsocket import settings
from event_receiver import EventReceiver

_logger = logging.getLogger(__name__)


#async def websocket_connection(websocket, path):
#    _logger.info(f"Connected: {path}")
#    msg = await websocket.recv()
#    print(msg)


async def websocket_main():
#    await websocket.serve(websocket_handler, port=settings.WEBSOCKET_PORT)



async def main():
    eventReceiver = EventReceiver()

    _logger.info("Websocket main!")


def run():
    asyncio.run(main())
