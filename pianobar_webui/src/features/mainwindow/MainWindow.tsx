import { useSelector } from "react-redux";
import { useAppDispatch } from "../../app/store";
import { changeStationAction, pauseAction, resumeAction, skipAction } from "../pianobar/actions/simpleActions";
import { selectPianobarRawUiState, selectPianobarStations } from "../pianobar/store/selector";
import CoverArt from "./CoverArt";

const MainWindow = () => {
    let uiState = useSelector(selectPianobarRawUiState);
    let pianobarStations = useSelector(selectPianobarStations);
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
            <table>
                <tbody>{stateList}</tbody>
            </table>
            <CoverArt width="400px" height="400px" />

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
        </div>
    );
};

export default MainWindow;
