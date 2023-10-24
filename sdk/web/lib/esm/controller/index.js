import { RadiantAppController } from "radiant-wasm";
export default class RadiantController {
    constructor(controller) {
        this._controller = controller;
    }
    static async createController(f) {
        return new RadiantController(await new RadiantAppController(f));
    }
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