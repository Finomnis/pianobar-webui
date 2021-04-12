import { Box } from "@material-ui/core";
import React from "react";
import useFitText from "use-fit-text";

type Props = {
    children:
    | React.ReactChild
    | React.ReactChild[];
}

const TextAutoShrinker = ({ children }: Props) => {
    const { ref, fontSize } = useFitText();
    return (
        <Box width="100%" display="flex" alignItems="center">
            <div>&#8203;{/* Zero space character. Preserves line height */}</div>
            <div ref={ref} style={{ width: "100%", fontSize }}>
                {children}
            </div>
        </Box>
    );
};
export default TextAutoShrinker;
