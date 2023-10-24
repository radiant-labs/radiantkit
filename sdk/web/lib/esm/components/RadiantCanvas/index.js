import React, { forwardRef } from "react";
export const RadiantCanvas = forwardRef(({}, ref) => {
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
//# sourceMappingURL=index.js.map