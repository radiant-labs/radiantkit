import init, { RadiantAppController } from "radiant-wasm";
import './App.css';

let controller: RadiantAppController | null = null;

function App() {
  const helloFromWasm = async () => {
    console.log("Hello from wasm");
    try {
      await init();
      controller = await new RadiantAppController((message: string) => {
        console.log("Message", message);
      });
      
    } catch (error) {
      console.log(error);
    }
  };

  const selectFirstNode = async () => {
    controller && controller.handleMessage({
      RadiantMessage: "Render"
    });
  }
  
  return (
    <div>
      <button onClick={() => helloFromWasm()}>Hello</button>
      <button onClick={() => selectFirstNode()}>Select</button>
    </div>
  );
}

export default App;
