import { RootState } from "../../../app/store";

// Selectors
export const selectPianobarUiState = (state: RootState) => state.pianobar.ui;
