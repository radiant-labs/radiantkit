import { useState, createContext, useEffect, useRef } from "react";
import init, { RadiantAppController } from "radiant-wasm";

class RadiantAppState {
    controller: RadiantAppController | null = null;
}

const RadiantAppContext = createContext<RadiantAppState>({
    controller: null,
});

function RadiantAppProvider({ children }: any) {
    const [appState, setAppState] = useState<RadiantAppState>({ controller: null });

    const initWasm = async () => {
        console.log("Initializing wasm");
        try {
            await init();
            let controller = await new RadiantAppController((message: any) => {
                console.log(message);
            });
            setAppState({
                controller,
            });
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
        <RadiantAppContext.Provider value={appState}>
            {children}
        </RadiantAppContext.Provider>
    )
}

export { RadiantAppContext, RadiantAppProvider};
