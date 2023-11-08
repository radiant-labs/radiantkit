import { RadiantAppController } from "radiant-runtime";
export declare class RadiantController {
    _controller: RadiantAppController;
    constructor(controller: RadiantAppController);
    static createController(f: Function): Promise<RadiantController>;
    /**
     * Activates the provided tool.
     *
     * @param tool the tool to activate.
     */
    activateTool(toolId: number): void;
    addRectangle(position: number[], scale: number[]): void;
    addImage(path: string, name?: string, position?: number[], scale?: number[]): void;
    setTransform(nodeId: number, position: number[], scale: number[]): void;
    setFillColor(nodeId: number, color: number[]): void;
    setStrokeColor(nodeId: number, color: number[]): void;
}
//# sourceMappingURL=index.d.ts.map