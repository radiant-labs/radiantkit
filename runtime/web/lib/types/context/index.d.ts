import React from "react";
import { RadiantKitController } from "../controller";
export interface RadiantKitState {
    controller: RadiantKitController | null;
    response: any;
}
declare function RadiantKitProvider({ children }: any): React.JSX.Element;
declare const useCurrentController: () => RadiantKitState;
export { RadiantKitProvider, useCurrentController };
//# sourceMappingURL=index.d.ts.map