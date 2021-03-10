import { createSlice, PayloadAction } from "@reduxjs/toolkit";

const slice = createSlice({
    name: "pianobar",
    initialState: {
        ui: {},
        websocket: {
            connected: false,
        },
    },
    reducers: {
        uiEventReceived: (
            state,
            action: PayloadAction<{ state: object; command: string }>
        ) => {
            state.ui = action.payload.state;
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
