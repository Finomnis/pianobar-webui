import { createSlice, PayloadAction } from "@reduxjs/toolkit";
import { PlayerState } from "./playerState";

let initialState: {
    ui: { [key: string]: object },
    player: PlayerState,
    websocket: { connected: boolean },
} = {
    ui: {},
    player: {
        paused: true,
        song_time_played: 0,
        song_time_total: 0
    },
    websocket: {
        connected: false,
    },
};

const slice = createSlice({
    name: "pianobar",
    initialState,
    reducers: {
        uiEventReceived: (
            state,
            action: PayloadAction<{ command: string; state: object }>
        ) => {
            state.ui = {};
            for (const [key, value] of Object.entries(action.payload.state)) {
                state.ui[key] = value;
            }
        },
        playerStateReceived: (
            state,
            action: PayloadAction<{ state: PlayerState }>
        ) => {
            state.player = action.payload.state;
        },
        websocketConnectionOpened: (state) => {
            state.websocket.connected = true;
        },
        websocketConnectionClosed: (state) => {
            state.websocket.connected = false;
        },
    },
});

// Slice exports
export const {
    uiEventReceived,
    playerStateReceived,
    websocketConnectionOpened,
    websocketConnectionClosed,
} = slice.actions;

export default slice.reducer;
