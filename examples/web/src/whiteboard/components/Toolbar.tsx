import { RadiantToolType, useCurrentController } from "@radiantkit/react";
import ImageTool from "./ImageTool";
import { IconButton } from "@mui/material";
import { PanTool, Rectangle, TextFields } from "@mui/icons-material";

const Toolbar = () => {
    const { controller } = useCurrentController();
  
    const handleSelectTool = (tool: RadiantToolType) => {
      controller?.activateTool(tool);
    }

    const addText = async () => {
      controller && controller.addText("Hello World!");
  }
  
    return (
      <div style={{ display: 'flex', flexDirection: 'column', alignItems: 'center', justifyContent: 'center', height: '100vh' }}>
        <IconButton component="span" onClick={() => handleSelectTool(RadiantToolType.Select)}>
          <PanTool />
        </IconButton>
        <IconButton component="span" onClick={() => handleSelectTool(RadiantToolType.Rectangle)}>
          <Rectangle />
        </IconButton>
        <ImageTool />
        <IconButton component="span" onClick={addText}>
          <TextFields />
        </IconButton>
      </div>
    );
};

export default Toolbar;