import { useState, createContext, useEffect, useRef } from "react";
import { RadiantAppController, RadiantSdk } from "radiant-sdk";

interface RadiantAppState {
    controller: RadiantAppController | null;
    response: any;
}

const RadiantAppContext = createContext<RadiantAppState>({
    controller: null,
    response: {},
});

function RadiantAppProvider({ children }: any) {
    const [controller, setController] = useState<RadiantAppController | null>(null);
    const [response, setResponse] = useState<any>({});

    const initWasm = async () => {
        console.log("Initializing wasm");
        try {
            let controller = await RadiantSdk.createAppController((message: any) => {
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
        <RadiantAppContext.Provider value={{
            controller,
            response,
        }}>
            {children}
        </RadiantAppContext.Provider>
    )
}

export { RadiantAppContext, RadiantAppProvider};
