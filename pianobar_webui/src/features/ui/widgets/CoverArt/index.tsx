import { useSelector } from "react-redux";
import { selectPianobarCoverArt } from "../../../pianobar/store/selector";
import note from "./musical-note.svg";

type CoverArtProps = {
    width: string;
    height: string;
};

const CoverArt = (props: CoverArtProps) => {
    let coverArtUrl = useSelector(selectPianobarCoverArt);
    return (
        <div
            style={{
                display: "flex",
                background: "black",
                width: props.width,
                height: props.height,
            }}
        >
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
            )}
        </div>
    );
};
export default React.memo(CoverArt);
