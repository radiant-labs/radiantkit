"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.RadiantCanvas = void 0;
const tslib_1 = require("tslib");
const react_1 = tslib_1.__importDefault(require("react"));
const RadiantCanvas = () => {
    return (react_1.default.createElement("div", { id: "canvas-container", style: {
            position: 'absolute',
            zIndex: 0,
            display: 'flex',
            alignItems: 'center',
            height: '100%',
            justifyContent: 'center',
            width: '100%',
        } }));
};
exports.RadiantCanvas = RadiantCanvas;
//# sourceMappingURL=index.js.map