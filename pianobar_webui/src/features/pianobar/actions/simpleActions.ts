import { all } from "@redux-saga/core/effects";
import { PianobarAction } from "./baseClass";


export const pauseAction = new PianobarAction("pause");
export const resumeAction = new PianobarAction("resume");
export const skipAction = new PianobarAction("skip");
export const changeStationAction = new PianobarAction<{ stationId: number }>("change_station", (params) => ({ "station_id": params.stationId }));


export function* simpleActionsSaga() {
    yield all([pauseAction.saga(), resumeAction.saga(), changeStationAction.saga(), skipAction.saga()]);
}
