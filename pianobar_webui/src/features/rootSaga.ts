import { all } from "redux-saga/effects";
import { simpleActionsSaga } from "./pianobar/actions/simpleActions";

import { pianobarWebsocketSaga } from "./pianobar/websocket/websocket";

export default function* rootSaga() {
    yield all([pianobarWebsocketSaga(), simpleActionsSaga()]);
}
