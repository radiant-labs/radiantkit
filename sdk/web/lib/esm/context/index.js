import React, { useState, createContext, useEffect, useRef, useContext } from "react";
import init from "radiant-wasm";
import RadiantController from "../controller";
const RadiantContext = createContext({
    controller: null,
    response: {},
});
function RadiantProvider({ children }) {
    const [controller, setController] = useState(null);
    const [response, setResponse] = useState({});
    const initWasm = async () => {
        console.log("Initializing wasm");
        try {
            await init();
            let controller = await RadiantController.createController((message) => {
                console.log(message);
                setResponse(message);
            });
            setController(controller);
        }
        catch (error) {
            console.log(error);
        }
    };
    const [, setTimesRun] = useState(0);
    const counter = useRef(0);
    const effectCalled = useRef(false);
    useEffect(() => {
        if (effectCalled.current)
            return;
        counter.current += 1;
        setTimesRun(counter.current);
        effectCalled.current = true;
        initWasm();
    }, []);
    return (React.createElement(RadiantContext.Provider, { value: {
            controller,
            response,
        } }, children));
}
const useCurrentController = () => {
    return useContext(RadiantContext);
};
export { RadiantProvider, useCurrentController };
//# sourceMappingURL=index.js.map