import { RadiantAppController } from "radiant-runtime";

export class RadiantController {
    _controller: RadiantAppController;

    constructor(controller: RadiantAppController) {
        this._controller = controller;
    }

    static async createController(f: Function): Promise<RadiantController> {
        return new RadiantController(await new RadiantAppController(f));   
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

    addRectangle(position: number[], scale: number[]) {
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
}