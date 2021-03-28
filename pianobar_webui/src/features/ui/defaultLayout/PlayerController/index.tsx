import React from "react";
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
    console.log("THEME:", theme);
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

const ProgressBar = React.memo(() => {
    const songDurationSeconds = useSelector(selectPianobarSongDurationSeconds);
    const songPlayedSeconds = useSelector(selectPianobarSongPlayedSeconds);
    return <SongLinearProgress color="primary" variant="determinate" value={100 * (songPlayedSeconds / songDurationSeconds)} />;
});

const SongTime = React.memo(() => {
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
});

const PlayPauseButton = React.memo(() => {
    const dispatch = useAppDispatch();
    const paused = useSelector(selectPianobarPaused);
    return <IconButton color="inherit" onClick={() => paused ? dispatch(resumeAction.run()) : dispatch(pauseAction.run())}>
        {
            paused
                ? <PlayArrowIcon style={{ fontSize: 45 }} />
                : <PauseIcon style={{ fontSize: 45 }} />
        }
    </IconButton>
});

const PlayerController = () => {
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
                    <PlayPauseButton />
                </Box>
                <Box flex="1 0 0" display="flex" justifyContent="flex-start" alignItems="center">
                    <Box className={styles.buttonHolder}>
                        <IconButton color="inherit" onClick={() => dispatch(skipAction.run())} >
                            <SkipNextIcon />
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
export default React.memo(PlayerController);
