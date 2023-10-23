"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.RadiantSdk = void 0;
const tslib_1 = require("tslib");
tslib_1.__exportStar(require("radiant-wasm"), exports);
const radiant_wasm_1 = tslib_1.__importStar(require("radiant-wasm"));
class RadiantSdk {
    static createAppController(f) {
        return tslib_1.__awaiter(this, void 0, void 0, function* () {
            yield (0, radiant_wasm_1.default)();
            return yield new radiant_wasm_1.RadiantAppController(f);
        });
    }
}
exports.RadiantSdk = RadiantSdk;
//# sourceMappingURL=index.js.map