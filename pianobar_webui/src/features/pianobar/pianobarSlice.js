import { createSlice } from "@reduxjs/toolkit";

export const counterSlice = createSlice({
    name: "pianobar",
    initialState: {
        ui: {},
    },
    reducers: {
        updateUi: (state, action) => {
            state.ui = action.payload;
        },
    },
});

// Slice exports
export const { updateUi } = counterSlice.actions;
export default counterSlice.reducer;

// Selectors
export const selectUi = (state) => state.pianobar.ui;
