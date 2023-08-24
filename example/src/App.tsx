import init, { hello } from "rucan-wasm";
import './App.css';

function App() {
  const helloFromWasm = async () => {
    await init();
    hello();
  };
  
  return (
    <div>
      <button onClick={() => helloFromWasm()}>Hello</button>
    </div>
  );
}

export default App;
