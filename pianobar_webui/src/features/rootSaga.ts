import { all } from "redux-saga/effects";

import counterSaga from "./counter/counterSaga";
import { pianobarWebsocketSaga } from "./pianobar/websocket/websocket";

export default function* rootSaga() {
    yield all([counterSaga(), pianobarWebsocketSaga()]);
}
