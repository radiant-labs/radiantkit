import init, { hello, MessageController } from "radiant-wasm";
import './App.css';

function App() {
  const helloFromWasm = async () => {
    console.log("Hello from wasm");
    try {
      await init();
      MessageController.setJSMessageHandler((message: string) => {
        console.log("Message", message);
      });
      MessageController.handleMessage({
        RadiantMessage: "Render"
      });
    } catch (error) {
      console.log(error);
    }
    hello();
  };
  
  return (
    <div>
      <button onClick={() => helloFromWasm()}>Hello</button>
    </div>
  );
}

export default App;
