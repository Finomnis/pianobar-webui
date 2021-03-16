import { call, takeEvery } from "@redux-saga/core/effects";
import { Action, PayloadActionCreator, createAction } from "@reduxjs/toolkit";
import { IWSRequestParams } from "rpc-websockets/dist/lib/client";
import websocket from "../websocket/websocket";

export class PianobarAction<T = void> {
    run: PayloadActionCreator<T>;
    #actionName: string;
    #payloadCreator: ((params: T) => IWSRequestParams) | null;

    constructor(actionName: string, payloadCreator: null | ((params: T) => IWSRequestParams) = null) {
        this.run = createAction<T>("pianobar/actions/" + actionName);
        this.#actionName = actionName;
        this.#payloadCreator = payloadCreator;
    }

    * handler(action: Action) {
        if (!this.run.match(action)) return;
        if (this.#payloadCreator) {
            const payload = this.#payloadCreator(action.payload);
            yield call(websocket.call.bind(websocket), this.#actionName, payload);
        } else {
            yield call(websocket.call.bind(websocket), this.#actionName);
        }
    }

    * saga() {
        yield takeEvery(this.run, this.handler.bind(this));
    }
};
