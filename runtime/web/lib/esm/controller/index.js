import { RadiantKitAppController } from "@radiantkit/radiantkit";
export class RadiantKitController {
    constructor(controller) {
        this._controller = controller;
    }
    static async createController(client_id, f, width, height) {
        return new RadiantKitController(await new RadiantKitAppController(client_id, f, width, height));
    }
    /**
     * Activates the provided tool.
     *
     * @param tool the tool to activate.
     */
    activateTool(toolId) {
        this._controller.handleMessage({
            SceneMessage: {
                SelectTool: {
                    id: toolId,
                },
            },
        });
    }
    addRectangle(position = [100, 100], scale = [100, 100]) {
        this._controller.handleMessage({
            AddRectangle: {
                position,
                scale,
            },
        });
    }
    addImage(path, name = "", position = [100, 100], scale = [100, 100]) {
        this._controller.handleMessage({
            AddImage: {
                name,
                path,
            },
        });
    }
    addText(text, position = [100, 100], scale = [100, 100]) {
        var _a;
        this._controller.handleMessage({
            AddText: {
                text,
                position,
            },
        });
        (_a = document.getElementById("radiantkit-canvas")) === null || _a === void 0 ? void 0 : _a.focus();
    }
    setTransform(nodeId, position, scale) {
        this._controller.handleMessage({
            SceneMessage: {
                SetTransform: {
                    id: nodeId,
                    position,
                    scale,
                },
            },
        });
    }
    setFillColor(nodeId, color) {
        this._controller.handleMessage({
            SceneMessage: {
                SetFillColor: {
                    id: nodeId,
                    fill_color: color,
                },
            },
        });
    }
    setStrokeColor(nodeId, color) {
        this._controller.handleMessage({
            SceneMessage: {
                SetStrokeColor: {
                    id: nodeId,
                    stroke_color: color,
                },
            },
        });
    }
    setText(nodeId, text) {
        this._controller.handleMessage({
            TextMessage: {
                SetText: {
                    id: nodeId,
                    text,
                },
            },
        });
    }
}
//# sourceMappingURL=index.js.map