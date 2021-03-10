import { RootState } from "../../app/store";

// Selectors
export const selectCount = (state: RootState) => state.counter.value;
