import { RadiantKitAppController, Vec3 } from "@radiantkit/radiantkit";

export class RadiantKitController {
    _controller: RadiantKitAppController;

    constructor(controller: RadiantKitAppController) {
        this._controller = controller;
    }

    static async createController(f: Function, width: number | undefined, height: number | undefined): Promise<RadiantKitController> {
        return new RadiantKitController(await new RadiantKitAppController(f, width, height));   
    }

    /**
     * Activates the provided tool.
     *
     * @param tool the tool to activate.
     */
    activateTool(toolId: number) {
        this._controller.handleMessage({
            SceneMessage: {
                SelectTool:  {
                    id: toolId,
                },
            },
        });
    }

    addRectangle(position: number[] = [100, 100], scale: number[] = [100, 100]) {
        this._controller.handleMessage({
            AddRectangle: {
                position,
                scale,
            },
        });
    }

    addImage(path: string, name: string = "", position: number[] = [100, 100], scale: number[] =[100, 100]) {
        this._controller.handleMessage({
            AddImage: {
                name,
                path,
            },
        });
    }

    addText(text: string, position: number[] = [100, 100], scale: number[] =[100, 100]) {
        this._controller.handleMessage({
            AddText: {
                text,
                position,
            },
        });
        document.getElementById("radiantkit-canvas")?.focus();
    }

    setTransform(nodeId: number, position: number[], scale: number[]) {
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

    setFillColor(nodeId: number, color: number[]) {
        this._controller.handleMessage({
            SceneMessage: {
                SetFillColor: {
                    id: nodeId,
                    fill_color: color,
                },
            },
        });
    }

    setStrokeColor(nodeId: number, color: number[]) {
        this._controller.handleMessage({
            SceneMessage: {
                SetStrokeColor: {
                    id: nodeId,
                    stroke_color: color,
                },
            },
        });
    }

    setText(nodeId: number, text: string) {
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
