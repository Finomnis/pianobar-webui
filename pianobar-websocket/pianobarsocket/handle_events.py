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
    event_command = args[0]

    # Read stdin data
    raw_event_data = sys.stdin.read()
    event_state = {}
    for entry in raw_event_data.split("\n"):
        split_entry = entry.split("=", 1)
        if len(split_entry) != 2:
            continue
        event_state[split_entry[0]] = split_entry[1]

    # Merge station# entries to 'stations' list
    stations = []
    if "stationCount" in event_state:
        station_count = int(event_state["stationCount"])
        del event_state["stationCount"]

        stations = [None] * station_count

        for i in range(station_count):
            station_id = f"station{i}"

            if station_id not in event_state:
                _logger.error(f"Invalid station list. {station_id} does not exist.")
                continue

            stations[i] = event_state[station_id]
            del event_state[station_id]
    event_state["stations"] = stations

    # Convert data to JSON
    message = json.dumps(
        {
            "command": event_command,
            "state": event_state,
        }
    ).encode("utf-8")

    # Send data to websocket server
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        # now connect to the web server on port 80 - the normal http port
        try:
            s.connect(("localhost", settings.EVENT_PORT))
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
