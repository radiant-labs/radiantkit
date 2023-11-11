import React, { useState, createContext, useEffect, useRef, useContext } from "react";
import init from "@radiantkit/radiantkit";
import { RadiantKitController } from "../controller";
const RadiantKitContext = createContext({
    controller: null,
    response: {},
});
function RadiantKitProvider({ children }) {
    const [controller, setController] = useState(null);
    const [response, setResponse] = useState({});
    const initWasm = async () => {
        try {
            await init();
            let controller = await RadiantKitController.createController((message) => {
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
    return (React.createElement(RadiantKitContext.Provider, { value: {
            controller,
            response,
        } }, children));
}
const useCurrentController = () => {
    return useContext(RadiantKitContext);
};
export { RadiantKitProvider, useCurrentController };
//# sourceMappingURL=index.js.map