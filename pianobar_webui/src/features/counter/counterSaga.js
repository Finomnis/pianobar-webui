import { put, takeEvery, delay } from "redux-saga/effects";

import { incrementAsync, incrementByAmount } from "./counterSlice";

function* incrementAsyncSaga(action) {
    yield delay(1000);
    yield put(incrementByAmount(action.payload));
}

export default function* counterSaga() {
    yield takeEvery(incrementAsync, incrementAsyncSaga);
}
