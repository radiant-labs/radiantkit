import init, { RadiantAppController } from "radiant-wasm";
import './App.css';
import { useState } from "react";

let controller: RadiantAppController | null = null;

function App() {
  const [selectedInfo, setSelectedInfo] = useState<string>("");

  const initWasm = async () => {
    console.log("Initializing wasm");
    try {
      await init();
      controller = await new RadiantAppController((message: any) => {
        setSelectedInfo(JSON.stringify(message));
      });
      
    } catch (error) {
      console.log(error);
    }
  };

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
    <div>
      <button onClick={() => initWasm()}>Init</button>
      <button onClick={() => select()}>Select</button>
      <button onClick={() => rect()}>Rectangle</button>
      <div>{selectedInfo}</div>
    </div>
  );
}

export default App;
