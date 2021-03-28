import { Box, IconButton } from "@material-ui/core";
import PlayArrowIcon from "@material-ui/icons/PlayArrow";
import PauseIcon from '@material-ui/icons/Pause';
import SkipNextIcon from '@material-ui/icons/SkipNext';
import { useSelector } from "react-redux";
import {
    selectPianobarPaused,
    //selectPianobarSongDurationSeconds,
    //selectPianobarSongPlayedSeconds,
    //selectPianobarSongDurationTime,
    //selectPianobarSongPlayedTime,
} from "../../../pianobar/store/selector";
import { useAppDispatch } from "../../../../app/store";
import { pauseAction, resumeAction, skipAction } from "../../../pianobar/actions/simpleActions";
import styles from "./styles.module.css";

type PlayerControllerProps = {
    width: string;
    height: string;
};

const PlayerController = (props: PlayerControllerProps) => {
    const paused = useSelector(selectPianobarPaused);
    //const songDurationSeconds = useSelector(selectPianobarSongDurationSeconds);
    //const songPlayedSeconds = useSelector(selectPianobarSongPlayedSeconds);
    //const songDurationTime = useSelector(selectPianobarSongDurationTime);
    //const songPlayedTime = useSelector(selectPianobarSongPlayedTime);

    const dispatch = useAppDispatch();

    return (
        <Box
            color="primary.contrastText"
            bgcolor="primary.main"
            display="flex"
            flex="0 0 auto"
            alignItems="center"
            width={props.width}
            height={props.height}
            overflow="hidden"
        >
            <Box flex="1 0 0" display="flex" justifyContent="flex-end" className={styles.buttonList}>
            </Box>
            <Box flex="0 0 auto" >
                <IconButton color="inherit">
                    {
                        paused
                            ? <PlayArrowIcon
                                style={{ fontSize: 45 }}
                                onClick={() => dispatch(resumeAction.run())} />
                            : <PauseIcon
                                style={{ fontSize: 45 }}
                                onClick={() => dispatch(pauseAction.run())} />
                    }
                </IconButton>
            </Box>
            <Box flex="1 0 0" display="flex" justifyContent="flex-start" className={styles.buttonList}>
                <IconButton color="inherit">
                    <SkipNextIcon onClick={() => dispatch(skipAction.run())} />
                </IconButton>
            </Box>
        </Box >
    );
};
export default PlayerController;
