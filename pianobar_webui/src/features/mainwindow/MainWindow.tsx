import { connect } from "react-redux";
import { RootState } from "../../app/store";

type MainWindowProps = {
    uiState: object;
    connected: boolean;
    coverArt: string;
};

const MainWindow = (props: MainWindowProps) => {
    let stateList = Object.entries(props.uiState).map(([key, value]) => (
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
            <img
                src={props.coverArt}
                alt="Unable to load cover art."
                width="400px"
                height="400px"
            ></img>
        </div>
    );
};

const mapStateToProps = (state: RootState /*, ownProps*/): MainWindowProps => {
    let coverArt = "";
    if ("coverArt" in state.pianobar.ui) {
        coverArt = state.pianobar.ui["coverArt"];
    }
    return {
        uiState: state.pianobar.ui,
        connected: state.pianobar.websocket.connected,
        coverArt: coverArt,
    };
};

const mapDispatchToProps = {};

export default connect(mapStateToProps, mapDispatchToProps)(MainWindow);
