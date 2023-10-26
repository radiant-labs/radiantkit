import { RadiantAppController } from "radiant-runtime";
export class RadiantController {
    constructor(controller) {
        this._controller = controller;
    }
    static async createController(f) {
        return new RadiantController(await new RadiantAppController(f));
    }
    /**
     * Activates the provided tool.
     *
     * @param tool the tool to activate.
     */
    activateTool(tool) {
        this._controller.handleMessage({
            SelectTool: tool
        });
    }
    setTransform(nodeId, position, scale) {
        this._controller.handleMessage({
            SetTransform: {
                id: nodeId,
                position,
                scale,
            },
        });
    }
    setFillColor(nodeId, color) {
        this._controller.handleMessage({
            SetFillColor: {
                id: nodeId,
                fill_color: color,
            },
        });
    }
    setStrokeColor(nodeId, color) {
        this._controller.handleMessage({
            SetStrokeColor: {
                id: nodeId,
                stroke_color: color,
            },
        });
    }
}
//# sourceMappingURL=index.js.map