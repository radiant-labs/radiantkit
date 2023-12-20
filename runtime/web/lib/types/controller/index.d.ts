import { RadiantKitAppController } from "@radiantkit/radiantkit";
export declare class RadiantKitController {
    _controller: RadiantKitAppController;
    constructor(controller: RadiantKitAppController);
    static createController(client_id: bigint, collaboration: boolean, f: Function, width: number | undefined, height: number | undefined): Promise<RadiantKitController>;
    /**
     * Activates the provided tool.
     *
     * @param tool the tool to activate.
     */
    activateTool(toolId: number): void;
    addRectangle(position?: number[], scale?: number[]): void;
    addImage(path: string, name?: string, position?: number[], scale?: number[]): void;
    addText(text: string, position?: number[], scale?: number[]): void;
    setTransform(nodeId: string, position: number[], scale: number[]): void;
    setFillColor(nodeId: string, color: number[]): void;
    setStrokeColor(nodeId: string, color: number[]): void;
    setText(nodeId: string, text: string): void;
}
//# sourceMappingURL=index.d.ts.map