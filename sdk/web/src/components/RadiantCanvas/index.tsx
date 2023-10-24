import React, { forwardRef } from "react";

const RadiantCanvas = forwardRef(({}, ref) => {
    return (
        <div
            id="canvas-container"
            style={{
                position: 'absolute',
                zIndex: 0,
                display: 'flex',
                alignItems: 'center',
                height: '100%',
                justifyContent: 'center',
                width: '100%',
            }}
        />
    );
});

export default RadiantCanvas;