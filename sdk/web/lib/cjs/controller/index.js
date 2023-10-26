"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.RadiantController = void 0;
const tslib_1 = require("tslib");
const radiant_runtime_1 = require("radiant-runtime");
class RadiantController {
    constructor(controller) {
        this._controller = controller;
    }
    static createController(f) {
        return tslib_1.__awaiter(this, void 0, void 0, function* () {
            return new RadiantController(yield new radiant_runtime_1.RadiantAppController(f));
        });
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
exports.RadiantController = RadiantController;
//# sourceMappingURL=index.js.map