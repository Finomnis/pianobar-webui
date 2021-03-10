import { createSlice } from "@reduxjs/toolkit";

const slice = createSlice({
    name: "ui",
    initialState: {
        state: {},
    },
    reducers: {
        newUiState: (state, action) => {
            state.state = action.payload;
        },
    },
});

// Slice exports
export const { newUiState } = slice.actions;
export default slice.reducer;
