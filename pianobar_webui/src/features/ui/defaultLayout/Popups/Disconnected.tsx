import { Backdrop, CircularProgress, createStyles, makeStyles, Theme } from "@material-ui/core";
import React from "react";
import { useSelector } from "react-redux";
import { selectPianobarConnected } from "../../../pianobar/store/selector";

const useStyles = makeStyles((theme: Theme) =>
    createStyles({
        backdrop: {
            zIndex: theme.zIndex.drawer + 1,
            color: '#fff',
        },
    }),
);

const Disconnected = () => {
    const classes = useStyles();
    const connected = useSelector(selectPianobarConnected);
    return (
        <Backdrop className={classes.backdrop} open={!connected}>
            <CircularProgress color="inherit" />
        </Backdrop>
    );
};

export default React.memo(Disconnected);
