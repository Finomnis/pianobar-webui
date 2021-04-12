import React from "react";
import { useSelector } from "react-redux";
import { selectPianobarCoverArt } from "../../../pianobar/store/selector";
import AutoSizer from "react-virtualized-auto-sizer";
import note from "./musical-note.svg";
import { Box } from "@material-ui/core";


const CoverArt = () => {
    let coverArtUrl = useSelector(selectPianobarCoverArt);
    return (
        <AutoSizer>
            {({ height, width }) => {
                const length = Math.min(width, height);
                return (
                    <Box
                        display="flex"
                        justifyContent="center"
                        alignItems="center"
                        width={width}
                        height={height}
                    >
                        <Box width={length} height={length} boxShadow={8}>
                            {coverArtUrl === "" ? (
                                <div
                                    style={{
                                        width: "100%",
                                        height: "100%",
                                        display: "flex",
                                        background: "#d3d3d3",
                                        justifyContent: "center",
                                        alignItems: "center",
                                    }}
                                >
                                    <img
                                        src={note}
                                        alt="coverArtPlaceholder"
                                        width="50%"
                                        height="50%"
                                        key="coverArtFallback"
                                    />
                                </div>
                            ) : (
                                <img
                                    src={coverArtUrl}
                                    alt="coverArt"
                                    width="100%"
                                    height="100%"
                                    key="coverArt"
                                />
                            )
                            }
                        </Box>
                    </Box>
                );
            }}
        </AutoSizer >
    );
};
export default React.memo(CoverArt);
