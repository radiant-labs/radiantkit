import { Button, ButtonGroup } from "@mui/material";
import { useContext } from "react";
import { RadiantAppContext } from "../contexts/RadiantAppContext";

export function RadiantToolbar() {
    const appState = useContext(RadiantAppContext);

    const select = async () => {
        appState.controller && appState.controller.handleMessage({
            SelectTool: "Selection"
        });
    }

    const rect = async () => {
        appState.controller && appState.controller.handleMessage({
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
