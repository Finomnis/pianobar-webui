import { Client } from "rpc-websockets";
import store from "../../../app/store";
import { uiEventReceived } from "../store/slice";

export function initializeUiEventReceiver(websocket: Client) {
    websocket.on("ui_event", (payload) =>
        store.dispatch(uiEventReceived(payload))
    );
}
