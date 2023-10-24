import { useContext, useEffect, useState } from 'react'
import { useCurrentController } from 'radiant-sdk'

function componentToHex(c: number) {
    var hex = c.toString(16)
    return hex.length === 1 ? '0' + hex : hex
}

function rgbToHex(r: number, g: number, b: number) {
    return '#' + componentToHex(r) + componentToHex(g) + componentToHex(b)
}

function hexTorgba(hex: string) {
    if (hex.length === 4) {
        hex = hex.replace(/([^#])/g, '$1$1')
    }
    return [
        parseInt(hex.substr(1, 2), 16),
        parseInt(hex.substr(3, 2), 16),
        parseInt(hex.substr(5, 2), 16),
        255,
    ]
}

export function RadiantColorPanel() {
    const { controller, response } = useCurrentController();

    const [nodeId, setNodeId] = useState<number>(0)
    const [fillColor, setFillColor] = useState('#000000')
    const [strokeColor, setStrokeColor] = useState('#000000')

    useEffect(() => {
        if (response?.NodeSelected) {
            let node = response.NodeSelected.Rectangle
            setNodeId(node.id)
            let { fill_color, stroke_color } = node.color
            setFillColor(rgbToHex(fill_color[0], fill_color[1], fill_color[2]))
            setStrokeColor(
                rgbToHex(stroke_color[0], stroke_color[1], stroke_color[2])
            )
        }
    }, [response])

    useEffect(() => {
        controller && controller.setFillColor(nodeId, hexTorgba(fillColor));
    }, [controller, fillColor, nodeId])

    useEffect(() => {
        controller && controller.setStrokeColor(nodeId, hexTorgba(strokeColor));
    }, [controller, strokeColor, nodeId])

    return (
        <div>
            <h1>Color</h1>
            <div>
                <label>Fill</label>
                <input
                    type="color"
                    value={fillColor}
                    onChange={(e) => {
                        setFillColor(e.target.value)
                    }}
                />
            </div>
            <div>
                <label>Stroke</label>
                <input
                    type="color"
                    value={strokeColor}
                    onChange={(e) => {
                        setStrokeColor(e.target.value)
                    }}
                />
            </div>
        </div>
    )
}
