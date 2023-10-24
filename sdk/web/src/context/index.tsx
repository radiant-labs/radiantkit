import React, { useState, createContext, useEffect, useRef, useContext } from "react";
import init from "radiant-wasm";
import RadiantController from "controller";

interface RadiantState {
    controller: RadiantController | null;
    response: any;
}

const RadiantContext = createContext<RadiantState>({
    controller: null,
    response: {},
});

function RadiantProvider({ children }: any) {
    const [controller, setController] = useState<RadiantController | null>(null);
    const [response, setResponse] = useState<any>({});

    const initWasm = async () => {
        console.log("Initializing wasm");
        try {
            await init();
            let controller = await RadiantController.createController((message: any) => {
                console.log(message);
                setResponse(message);
            });
            setController(controller);
        } catch (error) {
            console.log(error);
        }
    };

    const [, setTimesRun] = useState(0);
    const counter = useRef<number>(0);
    const effectCalled = useRef<boolean>(false);

    useEffect(() => {
        if (effectCalled.current) return;
        counter.current += 1;
        setTimesRun(counter.current);
        effectCalled.current = true;
        initWasm();
    }, []);

    return (
        <RadiantContext.Provider value={{
            controller,
            response,
        }}>
            {children}
        </RadiantContext.Provider>
    )
}

const useCurrentController = () => {
    return useContext(RadiantContext);
}

export { RadiantProvider, useCurrentController };
