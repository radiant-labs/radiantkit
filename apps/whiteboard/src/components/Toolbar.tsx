import { RadiantToolType, useCurrentController } from "@radiantkit/react";

const Toolbar = () => {
    const { controller } = useCurrentController();
  
    const handleSelectTool = (tool: RadiantToolType) => {
      controller?.activateTool(tool);
    }
  
    return (
      <div style={{ display: 'flex', flexDirection: 'column', alignItems: 'center', justifyContent: 'center', height: '100vh' }}>
        <button onClick={() => handleSelectTool(RadiantToolType.Select)}>Selection Tool</button>
        <button onClick={() => handleSelectTool(RadiantToolType.Rectangle)}>Rectangle Tool</button>
      </div>
    );
};

export default Toolbar;