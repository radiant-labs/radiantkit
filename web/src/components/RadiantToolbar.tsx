import { Button, ButtonGroup } from "@mui/material";
import { useContext } from "react";
import { RadiantAppContext } from "../contexts/RadiantAppContext";

export function RadiantToolbar() {
    const { controller } = useContext(RadiantAppContext);

    const select = async () => {
        controller && controller.handleMessage({
            SelectTool: "Selection"
        });
    }

    const rect = async () => {
        controller && controller.handleMessage({
            SelectTool: "Rectangle"
        });
    }

    return (
        <ButtonGroup orientation="vertical">
            <Button onClick={select}>Select</Button>
            <Button onClick={rect}>Rectangle</Button>
        </ButtonGroup>
    );
}
