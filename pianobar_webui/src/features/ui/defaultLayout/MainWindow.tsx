import { Box } from "@material-ui/core";
import MainContent from "./MainContent";
import PlayerController from "./PlayerController";
import TitleBar from "./TitleBar";

const MainWindow = () => (
    <Box display="flex" flexDirection="column" width="100%" height="100%">
        <TitleBar />
        <MainContent />
        <PlayerController />
    </Box >
);

export default MainWindow;
