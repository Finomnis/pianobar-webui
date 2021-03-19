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

export const selectPianobarConnected = (state: RootState): boolean => state.pianobar.websocket.connected;
export const selectPianobarPaused = (state: RootState): boolean => state.pianobar.player.paused;
export const selectPianobarSongPlayedSeconds = (state: RootState): number => state.pianobar.player.song_time_played;
export const selectPianobarSongDurationSeconds = (state: RootState): number => state.pianobar.player.song_time_total;


function convert_seconds_to_string(secs: number): string {
    const minutes = Math.floor(secs / 60);
    const seconds = secs % 60;
    if (seconds < 10) {
        return minutes + ":0" + seconds;
    } else {
        return minutes + ":" + seconds;
    }
}
export const selectPianobarSongPlayedTime = (state: RootState): string => {
    return convert_seconds_to_string(state.pianobar.player.song_time_played);
}
export const selectPianobarSongDurationTime = (state: RootState): string => {
    return convert_seconds_to_string(state.pianobar.player.song_time_total);
}
