import { Action } from "redux";
import { put, takeEvery, delay } from "redux-saga/effects";

import { incrementAsync, incrementByAmount } from "./counterSlice";

function* incrementAsyncSaga(action: Action) {
    // Required to make typescript happy
    if (!incrementAsync.match(action)) return;

    yield delay(1000);
    yield put(incrementByAmount(action.payload));
}

export default function* counterSaga() {
    yield takeEvery(incrementAsync, incrementAsyncSaga);
}
