import { InsertPhoto } from "@mui/icons-material";
import { IconButton } from "@mui/material";
import { useCurrentController } from "@radiantkit/react";

const ImageTool = () => {
    const { controller } = useCurrentController();

    const loadImage = async (path: string) => {
        controller && controller.addImage(path);
    }

    const handleUpload = (e: any) => {
        const fileReader = new FileReader();
        fileReader.readAsDataURL(e.target.files[0]);
        fileReader.onload = (e) => {
            const path = e.target?.result as string;
            loadImage(path);
        };
    }

    return (
        <div>
            <input
                accept="image/*"
                style={{ display: 'none' }}
                name="image-upload"
                id="image-upload"
                type="file"
                onChange={handleUpload}
            />
            <label htmlFor="image-upload">
                <IconButton component="span" style={{ height: "100%" }}>
                    <InsertPhoto />
                </IconButton>
            </label> 
        </div>
    )
}

export default ImageTool;
