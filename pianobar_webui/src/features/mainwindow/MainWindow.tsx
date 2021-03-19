import { useSelector } from "react-redux";
import { useAppDispatch } from "../../app/store";
import { changeStationAction, pauseAction, resumeAction, skipAction } from "../pianobar/actions/simpleActions";
import {
    selectPianobarAlbum,
    selectPianobarArtist,
    selectPianobarConnected,
    selectPianobarPaused,
    selectPianobarRawUiState,
    selectPianobarSongDurationTime,
    selectPianobarSongPlayedTime,
    selectPianobarStationName,
    selectPianobarStations,
    selectPianobarTitle
} from "../pianobar/store/selector";
import CoverArt from "./CoverArt";

const MainWindow = () => {
    let uiState = useSelector(selectPianobarRawUiState);
    let pianobarStations = useSelector(selectPianobarStations);
    let pianobarTitle = useSelector(selectPianobarTitle);
    let pianobarAlbum = useSelector(selectPianobarAlbum);
    let pianobarArtist = useSelector(selectPianobarArtist);
    let pianobarStationName = useSelector(selectPianobarStationName);
    let pianobarPaused = useSelector(selectPianobarPaused);
    let pianobarSongPlayedTime = useSelector(selectPianobarSongPlayedTime);
    let pianobarSongDurationTime = useSelector(selectPianobarSongDurationTime);

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
        <div>
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
            {pianobarSongPlayedTime}/{pianobarSongDurationTime}&nbsp;{pianobarPaused ? "\u23F8" : "\u23F5"}
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
            <button onClick={() => dispatch(resumeAction.run())}>Resume</button>
            &nbsp;&nbsp;
            <button onClick={() => dispatch(pauseAction.run())}>Pause</button>
            <br /><br />
            <button onClick={() => dispatch(skipAction.run())}>Skip</button>
            <br /><br /><br /><br />
            Connected: {connected ? "yes" : "no"}
            <br /><br />
            Raw state:
            <br /><br />
            <table>
                <tbody>{stateList}</tbody>
            </table>
        </div>
    );
};

export default MainWindow;
