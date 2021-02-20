import sys
import logging
import json
import socket

from pianobarsocket import settings

_logger = logging.getLogger(__name__)


def run():
    # Read args
    args = sys.argv[1:]
    if len(args) != 1:
        _logger.error(f"Invalid arguments received: {args}")
        return
    eventCommand = args[0]

    # Read stdin data
    rawEventData = sys.stdin.read()
    eventState = {}
    for entry in rawEventData.split("\n"):
        splitEntry = entry.split("=", 1)
        if len(splitEntry) != 2:
            continue
        eventState[splitEntry[0]] = splitEntry[1]

    # Merge station# entries to 'stations' list
    stations = []
    if "stationCount" in eventState:
        stationCount = int(eventState["stationCount"])
        del eventState["stationCount"]

        stations = [None] * stationCount

        for i in range(stationCount):
            stationId = f"station{i}"

            if stationId not in eventState:
                _logger.error(f"Invalid station list. {stationId} does not exist.")
                continue

            stations[i] = eventState[stationId]
            del eventState[stationId]
    eventState["stations"] = stations

    # Convert data to JSON
    message = json.dumps(
        {
            "command": eventCommand,
            "state": eventState,
        }
    ).encode("utf-8")

    # Send data to websocket server
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        # now connect to the web server on port 80 - the normal http port
        try:
            s.connect(("127.0.0.1", settings.EVENT_PORT))
        except ConnectionRefusedError as e:
            _logger.error(f"Unable to connect to websocket server: {e}")
            return

        totalsent = 0
        while totalsent < len(message):
            sent = s.send(message[totalsent:])
            if sent == 0:
                _logger.error("Sending the message failed.")
                return
            totalsent += sent
