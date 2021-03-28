import React from "react";
import { Box } from "@material-ui/core";
import { useAppDispatch } from "../../../../app/store";
import CoverArt from "../../widgets/CoverArt";
import { changeStationAction } from "../../../pianobar/actions/simpleActions";
import { useSelector } from "react-redux";
import {
    selectPianobarAlbum,
    selectPianobarArtist,
    selectPianobarConnected,
    selectPianobarRawUiState,
    selectPianobarStationName,
    selectPianobarStations,
    selectPianobarTitle
} from "../../../pianobar/store/selector";

const MainContent = () => {

    let uiState = useSelector(selectPianobarRawUiState);
    let pianobarStations = useSelector(selectPianobarStations);
    let pianobarTitle = useSelector(selectPianobarTitle);
    let pianobarAlbum = useSelector(selectPianobarAlbum);
    let pianobarArtist = useSelector(selectPianobarArtist);
    let pianobarStationName = useSelector(selectPianobarStationName);

    let connected = useSelector(selectPianobarConnected);

    let dispatch = useAppDispatch();

    let stateList = Object.entries(uiState).map(([key, value]) => (
        <tr key={key}>
            <td>{key}</td>
            <td>{String(value)}</td>
        </tr>
    ));

    const changeStation = (e: any) => {
        e.preventDefault();

        const station = parseInt(e.target[0].value);
        dispatch(changeStationAction.run({ stationId: station }));

        return false;
    }

    return (
        <Box width="100%" height="100%" overflow="auto">
            <CoverArt width="300px" height="300px" />
            <br />
            - {pianobarStationName} -
            <br />
            <table>
                <tbody>
                    <tr>
                        <td>Song:</td><td>{pianobarTitle}</td>
                    </tr><tr>
                        <td>Artist:</td><td>{pianobarArtist}</td>
                    </tr><tr>
                        <td>Album:</td><td>{pianobarAlbum}</td>
                    </tr>
                </tbody>
            </table>
            <br />
            <br />
            <form onSubmit={changeStation}>
                <label>Station:&nbsp;
                    <select required>
                        {
                            pianobarStations.map((station, index) => (
                                <option value={index} key={index}>{station}</option>
                            ))
                        }
                    </select>
                    <button>Change Station</button>
                </label>
            </form>
            <br />
            <br /><br /><br /><br />
            Connected: {connected ? "yes" : "no"}
            <br /><br />
            Raw state:
            <br /><br />
            <table>
                <tbody>{stateList}</tbody>
            </table>
        </Box>
    );
};
export default React.memo(MainContent);
