import React, { forwardRef } from "react";

export const RadiantCanvas = () => {
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
};
