import React from "react";
import { RadiantKitController } from "../controller";
export interface RadiantKitState {
    controller: RadiantKitController | null;
    response: any;
}
declare function RadiantKitProvider({ width, height, children }: {
    width: number | undefined;
    height: number | undefined;
    children: any;
}): React.JSX.Element;
declare const useCurrentController: () => RadiantKitState;
export { RadiantKitProvider, useCurrentController };
//# sourceMappingURL=index.d.ts.map