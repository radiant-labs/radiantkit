import React, { forwardRef } from "react";
const RadiantCanvas = forwardRef(({}, ref) => {
    return (React.createElement("div", { id: "canvas-container", style: {
            position: 'absolute',
            zIndex: 0,
            display: 'flex',
            alignItems: 'center',
            height: '100%',
            justifyContent: 'center',
            width: '100%',
        } }));
});
export default RadiantCanvas;
//# sourceMappingURL=index.js.map