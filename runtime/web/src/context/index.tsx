import React, { useState, createContext, useEffect, useRef, useContext } from "react";
import init, { Vec3 } from "@radiantkit/radiantkit";
import { RadiantKitController } from "controller";

export interface RadiantKitState {
    controller: RadiantKitController | null;
    response: any;
}

const RadiantKitContext = createContext<RadiantKitState>({
    controller: null,
    response: {},
});

export interface RadiantKitProviderProps {
    width?: number;
    height?: number;
    children?: any;
}

function RadiantKitProvider({ width, height, children }: RadiantKitProviderProps) {
    const [controller, setController] = useState<RadiantKitController | null>(null);
    const [response, setResponse] = useState<any>({});

    const initWasm = async () => {
        try {
            await init();
            let controller = await RadiantKitController.createController((message: any) => {
                setResponse(message);
            }, width, height);
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
        <RadiantKitContext.Provider value={{
            controller,
            response,
        }}>
            {children}
        </RadiantKitContext.Provider>
    )
}

const useCurrentController = () => {
    return useContext(RadiantKitContext);
}

export { RadiantKitProvider, useCurrentController };
