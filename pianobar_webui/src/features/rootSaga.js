import { call, all } from "redux-saga/effects";

import counterSaga from "./counter/counterSaga";

function* helloSaga() {
  yield call(console.log, "Hello Sagas!");
}

export default function* rootSaga() {
  yield all([helloSaga(), counterSaga()]);
}
