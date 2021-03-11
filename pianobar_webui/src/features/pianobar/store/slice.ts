import { createSlice, PayloadAction } from "@reduxjs/toolkit";

let initialState: {
    ui: { [key: string]: object };
    websocket: { connected: boolean };
} = {
    ui: {},
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
    websocketConnectionOpened,
    websocketConnectionClosed,
} = slice.actions;

export default slice.reducer;
