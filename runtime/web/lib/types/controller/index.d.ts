import { RadiantKitAppController } from "@radiantkit/radiantkit";
export declare class RadiantKitController {
    _controller: RadiantKitAppController;
    constructor(controller: RadiantKitAppController);
    static createController(client_id: bigint, f: Function, width: number | undefined, height: number | undefined): Promise<RadiantKitController>;
    /**
     * Activates the provided tool.
     *
     * @param tool the tool to activate.
     */
    activateTool(toolId: number): void;
    addRectangle(position?: number[], scale?: number[]): void;
    addImage(path: string, name?: string, position?: number[], scale?: number[]): void;
    addText(text: string, position?: number[], scale?: number[]): void;
    setTransform(nodeId: number, position: number[], scale: number[]): void;
    setFillColor(nodeId: number, color: number[]): void;
    setStrokeColor(nodeId: number, color: number[]): void;
    setText(nodeId: number, text: string): void;
}
//# sourceMappingURL=index.d.ts.map