import React from "react";
import { Box, Select, Typography } from "@material-ui/core";
import { useAppDispatch } from "../../../../app/store";
import CoverArt from "../../widgets/CoverArt";
import { changeStationAction } from "../../../pianobar/actions/simpleActions";
import { useSelector } from "react-redux";
import {
    selectPianobarAlbum,
    selectPianobarArtist,
    selectPianobarStationId,
    selectPianobarStations,
    selectPianobarTitle
} from "../../../pianobar/store/selector";
import Popups from "../Popups";

const MainContent = () => {

    let pianobarStations = useSelector(selectPianobarStations);
    let pianobarTitle = useSelector(selectPianobarTitle);
    let pianobarAlbum = useSelector(selectPianobarAlbum);
    let pianobarArtist = useSelector(selectPianobarArtist);
    let pianobarStationId = useSelector(selectPianobarStationId);

    let dispatch = useAppDispatch();

    const handleChange = (event: React.ChangeEvent<{ name?: string; value: unknown }>) => {
        const value = event.target.value;
        if (typeof (value) != "string")
            return;
        const station = parseInt(value);

        dispatch(changeStationAction.run({ stationId: station }));
    };

    return (
        <Box
            flex="1 0 0"
            overflow="hidden"
            display="flex"
            flexDirection="column"
            alignItems="center"
        >
            <Box flex="1 0 0" /> {/* space */}
            <Select
                native
                value={pianobarStationId}
                onChange={handleChange}
                label="Station"
            >
                {
                    pianobarStations.map((station, index) => (
                        <option value={index} key={index}>{station}</option>
                    ))
                }
            </Select>
            <Box flex="1 0 0" /> {/* space */}

            <Box flex="10 0 0" width="90%">
                <CoverArt />
            </Box>

            <Box flex="0.7 0 0" /> {/* space */}
            <Box width="100%">
                <Typography variant="h6" align="center" noWrap>
                    {pianobarTitle}
                </Typography>
                <Typography noWrap align="center">
                    {pianobarArtist}
                </Typography>
                <Typography noWrap align="center">
                    {pianobarAlbum}
                </Typography>
            </Box>
            <Box flex="1 0 0" /> {/* space */}

            <Popups />
        </Box >
    );
};
export default React.memo(MainContent);
