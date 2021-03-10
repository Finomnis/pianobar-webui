import { createSlice } from "@reduxjs/toolkit";

const slice = createSlice({
    name: "pianobar",
    initialState: {
        ui: {},
    },
    reducers: {
        uiEventReceived: (s, action) => {
            let { state } = action.payload;
            s.ui = state;
        },
    },
});

// Slice exports
export const { uiEventReceived } = slice.actions;
export default slice.reducer;
