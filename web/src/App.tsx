import init, { hello, setJSMessageHandler, handleMessage } from "radiant-wasm";
import './App.css';

function App() {
  const helloFromWasm = async () => {
    console.log("Hello from wasm");
    try {
      await init();
      setJSMessageHandler((message: string) => {
        console.log("Message", message);
      });
      handleMessage({
        RadiantNodeMessage: "Render"
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
