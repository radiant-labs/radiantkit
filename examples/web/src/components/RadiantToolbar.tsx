import { Button, ButtonGroup } from "@mui/material";
import { useCurrentController } from 'radiant-sdk'

export function RadiantToolbar() {
    const { controller } = useCurrentController();

    const select = async () => {
        controller && controller.activateTool(0);
    }

    const rect = async () => {
        controller && controller.activateTool(1);
    }

    return (
        <ButtonGroup orientation="vertical" style={{ pointerEvents: 'all' }}>
            <Button onClick={select}>Select</Button>
            <Button onClick={rect}>Rectangle</Button>
        </ButtonGroup>
    );
}
