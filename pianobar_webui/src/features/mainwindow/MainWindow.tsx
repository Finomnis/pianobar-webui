import { useSelector } from "react-redux";
import { selectPianobarRawUiState } from "../pianobar/store/selector";
import CoverArt from "./CoverArt";

const MainWindow = () => {
    let uiState = useSelector(selectPianobarRawUiState);

    let stateList = Object.entries(uiState).map(([key, value]) => (
        <tr key={key}>
            <td>{key}</td>
            <td>{String(value)}</td>
        </tr>
    ));

    return (
        <div>
            <table>
                <tbody>{stateList}</tbody>
            </table>
            <CoverArt width="400px" height="400px" />
        </div>
    );
};

export default MainWindow;
