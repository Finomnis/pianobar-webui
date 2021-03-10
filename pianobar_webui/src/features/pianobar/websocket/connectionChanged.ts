import { Client } from "rpc-websockets";
import store from "../../../app/store";
import {
    websocketConnectionOpened,
    websocketConnectionClosed,
} from "../store/slice";

export function initializeConnectionHandlers(websocket: Client) {
    websocket.on("open", () => store.dispatch(websocketConnectionOpened()));
    websocket.on("close", () => store.dispatch(websocketConnectionClosed()));
}
