import { configureStore } from "@reduxjs/toolkit";
import createSagaMiddleware from "redux-saga";
import { useDispatch } from "react-redux";

import rootSaga from "../features/rootSaga";
import rootReducer from "../features/rootReducer";

// Create saga middleware
const sagaMiddleware = createSagaMiddleware();

// Initialize redux store
const store = configureStore({
    reducer: rootReducer,
    middleware: (defaultMiddleware) =>
        defaultMiddleware({ thunk: false }).concat(sagaMiddleware),
    devTools: process.env.NODE_ENV !== "production",
});

// Start saga
sagaMiddleware.run(rootSaga);

export default store;

// Typescript stuff
export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;
export const useAppDispatch = () => useDispatch<AppDispatch>();
