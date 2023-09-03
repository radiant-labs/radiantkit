import init, { hello } from "radiant-wasm";
import './App.css';

function App() {
  const helloFromWasm = async () => {
    console.log("Hello from wasm");
    try {
      await init();
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
