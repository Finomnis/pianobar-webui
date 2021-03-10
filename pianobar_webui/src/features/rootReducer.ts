import { combineReducers } from "redux";
import pianobarUiReducer from "./pianobar_ui/pianobarUiSlice";
import counterReducer from "./counter/counterSlice";

export default combineReducers({
    counter: counterReducer,
    pianobar: combineReducers({
        ui: pianobarUiReducer,
    }),
});
