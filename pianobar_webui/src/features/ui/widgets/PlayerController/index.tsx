import { Box, Hidden, IconButton, LinearProgress, Theme, withStyles } from "@material-ui/core";
import PlayArrowIcon from "@material-ui/icons/PlayArrow";
import PauseIcon from '@material-ui/icons/Pause';
import SkipNextIcon from '@material-ui/icons/SkipNext';
import { useSelector } from "react-redux";
import {
    selectPianobarPaused,
    selectPianobarSongDurationSeconds,
    selectPianobarSongPlayedSeconds,
    selectPianobarSongDurationTime,
    selectPianobarSongPlayedTime,
} from "../../../pianobar/store/selector";
import { useAppDispatch } from "../../../../app/store";
import { pauseAction, resumeAction, skipAction } from "../../../pianobar/actions/simpleActions";
import styles from "./styles.module.css";

const SongLinearProgress = withStyles((theme: Theme) => {
    /* console.log("THEME:", theme); */
    return {
        root: {
            height: 4,
            marginBottom: -2,
        },
        bar: {
            backgroundColor: theme.palette.primary.light,
        },
    };
})(LinearProgress);

type PlayerControllerProps = {
};

const ProgressBar = () => {
    const songDurationSeconds = useSelector(selectPianobarSongDurationSeconds);
    const songPlayedSeconds = useSelector(selectPianobarSongPlayedSeconds);
    return <SongLinearProgress color="primary" variant="determinate" value={100 * (songPlayedSeconds / songDurationSeconds)} />;
};


const SongTime = () => {
    const songDurationTime = useSelector(selectPianobarSongDurationTime);
    const songPlayedTime = useSelector(selectPianobarSongPlayedTime);
    return <Box flex="0 0 auto" display="flex" flexDirection="row" alignItems="stretch"
        marginRight="2em" style={{ opacity: 0.5 }}
        fontSize=".9em"
    >
        <Box>{songPlayedTime}</Box>
        <Box marginX=".8em" bgcolor="white" width="1px" />
        <Box>{songDurationTime}</Box>
    </Box>;
};

const PlayerController = (props: PlayerControllerProps) => {
    const paused = useSelector(selectPianobarPaused);

    const dispatch = useAppDispatch();

    return (
        <Box
            display="flex"
            flex="0 0 auto"
            flexDirection="column"
            width="100%"
            alignItems="stretch"
            overflow="hidden"
        >
            <ProgressBar />
            <Box
                color="primary.contrastText"
                bgcolor="primary.main"
                display="flex"
                flex="0 0 auto"
                alignItems="center"
                overflow="hidden"
            >
                <Box flex="1 0 0" display="flex" justifyContent="flex-end">
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
                <Box flex="1 0 0" display="flex" justifyContent="flex-start" alignItems="center">
                    <Box className={styles.buttonHolder}>
                        <IconButton color="inherit">
                            <SkipNextIcon onClick={() => dispatch(skipAction.run())} />
                        </IconButton>
                    </Box>

                    <Box flex="1 0 0" /> {/* SPACER */}

                    <Hidden xsDown>
                        <SongTime />
                    </Hidden>
                </Box>
            </Box >
        </Box>
    );
};
export default PlayerController;
