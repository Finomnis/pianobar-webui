import { all } from "redux-saga/effects";

import { pianobarWebsocketSaga } from "./pianobar/websocket/websocket";

export default function* rootSaga() {
    yield all([pianobarWebsocketSaga()]);
}
