"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const tslib_1 = require("tslib");
const react_1 = tslib_1.__importStar(require("react"));
const RadiantCanvas = (0, react_1.forwardRef)(({}, ref) => {
    return (react_1.default.createElement("div", { id: "canvas-container", style: {
            position: 'absolute',
            zIndex: 0,
            display: 'flex',
            alignItems: 'center',
            height: '100%',
            justifyContent: 'center',
            width: '100%',
        } }));
});
exports.default = RadiantCanvas;
//# sourceMappingURL=index.js.map