import { combineReducers } from "redux";
import pianobarReducer from "./pianobar/store/slice";

export default combineReducers({
    pianobar: pianobarReducer,
});
