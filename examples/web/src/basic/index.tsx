import { RadiantKitCanvas, RadiantKitProvider, useCurrentController } from '@radiantkit/react';
import { Box, Button, ButtonGroup, Stack, Typography } from '@mui/material';
import { useEffect, useState } from 'react';

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

const Tools = () => {
    const { controller } = useCurrentController();

    const select = async () => {
        controller && controller.activateTool(0);
    }

    const rect = async () => {
        controller && controller.activateTool(1);
    }

    useEffect(() => {
        controller && controller.addRectangle([100, 100], [100, 100]);
    }, [controller])

    return (
        <Stack direction="row" spacing={2}>
            <Typography variant="h6">Tools: </Typography>
            <ButtonGroup style={{ pointerEvents: 'all' }}>
                <Button onClick={select}>Select</Button>
                <Button onClick={rect}>Rectangle</Button>
            </ButtonGroup>
        </Stack>
    )
}

const Transform = () => {
    const { controller, response } = useCurrentController();

    const [nodeId, setNodeId] = useState<string>("00000000-0000-0000-0000-000000000000")
    const [position, setPosition] = useState({ x: 0, y: 0 })
    const [scale, setScale] = useState({ x: 1, y: 1 })

    useEffect(() => {
        if (response?.Selected) {
            let node = response.Selected.node.Rectangle;
            setNodeId(node.id)
            let transform = node.transform
            setPosition({ x: transform.position.x, y: transform.position.y })
            setScale({ x: transform.scale.x, y: transform.scale.y })
            // setRotation(transform.rotation)
        } else if (response?.TransformUpdated) {
            let transform = response.TransformUpdated
            setNodeId(transform.id)
            setPosition({ x: transform.position[0], y: transform.position[1] })
            setScale({ x: transform.scale[0], y: transform.scale[1] })
            // setRotation(transform.rotation);
        }
    }, [response])

    useEffect(() => {
        controller && controller.setTransform(nodeId, [position.x, position.y], [scale.x, scale.y]);
    }, [controller, nodeId, position, scale])

    return (
        <Stack>
            <Stack direction="row" spacing={2}>
                <Typography variant="h6">Position: </Typography>
                <input
                    type="number"
                    value={position.x}
                    onChange={(e) => {
                        setPosition({
                            x: parseFloat(e.target.value),
                            y: position.y,
                        })
                    }}
                />
                <input
                    type="number"
                    value={position.y}
                    onChange={(e) => {
                        setPosition({
                            x: position.x,
                            y: parseFloat(e.target.value),
                        })
                    }}
                />
            </Stack>
            <Box height={10} />
            <Stack direction="row" spacing={2}>
                <Typography variant="h6">Scale: </Typography>
                <input
                    type="number"
                    value={scale.x}
                    onChange={(e) => {
                        setScale({ x: parseFloat(e.target.value), y: scale.y })
                    }}
                />
                <input
                    type="number"
                    value={scale.y}
                    onChange={(e) => {
                        setScale({ x: scale.x, y: parseFloat(e.target.value) })
                    }}
                />
            </Stack>
        </Stack>
    )
}

const Color = () => {
    const { controller, response } = useCurrentController();

    const [nodeId, setNodeId] = useState<string>("00000000-0000-0000-0000-000000000000")
    const [fillColor, setFillColor] = useState('#000000')
    const [strokeColor, setStrokeColor] = useState('#000000')

    useEffect(() => {
        if (response?.Selected) {
            let node = response.Selected.node.Rectangle
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
        <Stack direction="row" spacing={2}>
            <Typography variant="h6">Color: </Typography>
            <input
                type="color"
                value={fillColor}
                onChange={(e) => {
                    setFillColor(e.target.value)
                }}
            />
             <input
                type="color"
                value={strokeColor}
                onChange={(e) => {
                    setStrokeColor(e.target.value)
                }}
            />
        </Stack>
    )
}

const BasicExample = () => {
    return (
        <RadiantKitProvider width={1600} height={1200}>
            <Stack>
                <Tools />
                <Box height={10} />
                <Transform />
                <Box height={10} />
                <Color />
                <RadiantKitCanvas />
            </Stack>
        </RadiantKitProvider>
    )
}

export default BasicExample;