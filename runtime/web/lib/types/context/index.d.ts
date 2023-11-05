import React from "react";
import { RadiantController } from "../controller";
export interface RadiantState {
    controller: RadiantController | null;
    response: any;
}
declare function RadiantProvider({ children }: any): React.JSX.Element;
declare const useCurrentController: () => RadiantState;
export { RadiantProvider, useCurrentController };
//# sourceMappingURL=index.d.ts.map