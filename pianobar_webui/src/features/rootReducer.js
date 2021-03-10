import { combineReducers } from "redux";
import pianobarReducer from "./pianobar/pianobarSlice";
import counterReducer from "./counter/counterSlice";

export default combineReducers({
  counter: counterReducer,
  pianobar: pianobarReducer,
});
