import React from "react";
import { RadiantKitController } from "../controller";
export interface RadiantKitState {
    controller: RadiantKitController | null;
    response: any;
}
export interface RadiantKitProviderProps {
    client_id?: bigint;
    collaborate?: boolean;
    width?: number;
    height?: number;
    paddingX?: number;
    paddingY?: number;
    children?: any;
}
declare function RadiantKitProvider({ client_id, collaborate, width, height, paddingX, paddingY, children }: RadiantKitProviderProps): React.JSX.Element;
declare const useCurrentController: () => RadiantKitState;
export { RadiantKitProvider, useCurrentController };
//# sourceMappingURL=index.d.ts.map