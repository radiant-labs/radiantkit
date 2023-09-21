import { useContext, useEffect, useState } from "react";
import { RadiantAppContext } from "../contexts/RadiantAppContext";

function componentToHex(c: number) {
    var hex = c.toString(16);
    return hex.length === 1 ? "0" + hex : hex;
}

function rgbToHex(r: number, g: number, b: number) {
    return "#" + componentToHex(r) + componentToHex(g) + componentToHex(b);
}

export function RadiantColorPanel() {
    const { response } = useContext(RadiantAppContext);

    const [fillColor, setFillColor] = useState("#000000");
    const [strokeColor, setStrokeColor] = useState("#000000");

    useEffect(() => {
        if (response?.NodeSelected) {
            let node = response.NodeSelected.Rectangle;
            let { fill_color, stroke_color } = node.color;
            setFillColor(rgbToHex(fill_color[0], fill_color[1], fill_color[2]));
            setStrokeColor(rgbToHex(stroke_color[0], stroke_color[1], stroke_color[2]));
        }
    }, [response]);

    return (
        <div>
            <h1>Color</h1>
            <div>
                <label>Fill</label>
                <input type="color" value={fillColor} onChange={(e) => {
                    setFillColor(e.target.value);
                }} />
            </div>
            <div>
                <label>Stroke</label>
                <input type="color" value={strokeColor} onChange={(e) => {
                    setStrokeColor(e.target.value);
                }} />
            </div>
        </div>
    );
}
