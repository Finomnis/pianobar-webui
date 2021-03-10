import { call } from "@redux-saga/core/effects";
import { Client } from "rpc-websockets";

import { WEBSOCKET_PORT } from "../../../config";
import { initializeUiEvents } from "./uiEvents";
import websocket from "./websocket";

export function* pianobarWebsocketSaga() {
    // Register notification listeners
    yield call(initializeUiEvents, websocket);

    // Start connection
    yield call(websocket.connect.bind(websocket));
}

// Create websocket
export default new Client(
    "ws://" + window.location.hostname + ":" + WEBSOCKET_PORT + "/ws",
    { autoconnect: false, reconnect: true, max_reconnects: 0 }
);
