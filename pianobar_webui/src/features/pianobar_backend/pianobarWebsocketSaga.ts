import { delay } from "@redux-saga/core/effects";

export default function* pianobarWebsocketSaga() {
    console.log("Websocket saga started.");
    yield delay(1000);
    console.log("Websocket saga ended.");
}
