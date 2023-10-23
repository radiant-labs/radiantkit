export * from "radiant-wasm";

import init, { RadiantAppController } from "radiant-wasm";

export class RadiantSdk {
    static async createAppController(f: Function): Promise<RadiantAppController> {
        await init();
        return await new RadiantAppController(f);
    }
}