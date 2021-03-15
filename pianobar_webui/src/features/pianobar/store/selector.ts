import { RootState } from "../../../app/store";

// Selectors
export const selectPianobarRawUiState = (state: RootState) => state.pianobar.ui;

function selectPianobarStateString(state: RootState, key: string): string {
    let uiState = selectPianobarRawUiState(state);

    let result = "";
    if (key in uiState) {
        let value = uiState[key];
        if (typeof value === "string") {
            result = value;
        }
    }
    return result;
}

function selectPianobarStateNumber(state: RootState, key: string): number {
    const result = selectPianobarStateString(state, key);
    if (result === "") {
        return NaN;
    }
    return Number(result);
}

export const selectPianobarCoverArt = (state: RootState): string => {
    return selectPianobarStateString(state, "coverArt");
};

export const selectPianobarAlbum = (state: RootState): string => {
    return selectPianobarStateString(state, "album");
};

export const selectPianobarArtist = (state: RootState): string => {
    return selectPianobarStateString(state, "artist");
};
export const selectPianobarTitle = (state: RootState): string => {
    return selectPianobarStateString(state, "title");
};

export const selectPianobarStationName = (state: RootState): string => {
    return selectPianobarStateString(state, "stationName");
};

export const selectPianobarSongDurationSeconds = (state: RootState): number => {
    return selectPianobarStateNumber(state, "songDuration");
};

export const selectPianobarSongPlayedSeconds = (state: RootState): number => {
    return selectPianobarStateNumber(state, "songPlayed");
};

export const selectPianobarRating = (state: RootState): number => {
    return selectPianobarStateNumber(state, "rating");
};

export const selectPianobarStations = (state: RootState): string[] => {
    let uiState = selectPianobarRawUiState(state);

    let result: string[] = [];

    if ("stations" in uiState) {
        let stations = uiState["stations"];
        if (Array.isArray(stations)) {
            for (const element of stations) {
                if (typeof element !== "string") {
                    console.error("'stations' entry contains invalid values:", stations);
                    return [];
                }
                result.push(element);
            }
        }
    }

    return result;
};
