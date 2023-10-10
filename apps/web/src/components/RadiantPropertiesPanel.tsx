import { RadiantColorPanel } from "./RadiantColorPanel";
import { RadiantTransformPanel } from "./RadiantTransformPanel";
import { Stack } from "@mui/material";

export function RadiantPropertiesPanel() {
    return (
        <Stack>
            <RadiantTransformPanel />
            <RadiantColorPanel />
        </Stack>
    );
}
