import { Client } from "rpc-websockets";
import store from "../../../app/store";
import { playerStateReceived } from "../store/slice";

export function initializePlayerStateReceiver(websocket: Client) {
    websocket.on("player_state", (payload) =>
        store.dispatch(playerStateReceived(payload))
    );
}
