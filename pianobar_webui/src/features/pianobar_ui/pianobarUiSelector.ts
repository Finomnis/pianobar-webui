import { RootState } from "../../app/store";

// Selectors
export const selectUi = (state: RootState) => state.pianobar.ui.state;
