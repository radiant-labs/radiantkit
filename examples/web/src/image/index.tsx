import { RadiantCanvas, RadiantProvider, useCurrentController } from 'radiant-sdk';
import { Box, Button, Stack, TextField } from '@mui/material';
import { useState } from 'react';

const ImageLoader = () => {
    const [path, setPath] = useState<string>('');
    const { controller } = useCurrentController();

    const loadImage = async (path: string) => {
        controller && controller.addImage(path);
    }

    const handleUpload = (e: any) => {
        const fileReader = new FileReader();
        fileReader.readAsDataURL(e.target.files[0]);
        fileReader.onload = (e) => {
            const path = e.target?.result as string;
            setPath(path);
            loadImage(path);
        };
    }

    return (
        <Stack direction="row" spacing={2}>
            <TextField 
                style={{ width: 300 }}
                label="Image Path" 
                variant="outlined" 
                placeholder='https://i.imgur.com/XbLP6ux.png' 
                value={path} 
                onChange={(e) => setPath(e.target.value)} />
            <Button onClick={() => loadImage(path || 'https://i.imgur.com/XbLP6ux.png')}>Load Image</Button>
            <input
                accept="image/*"
                style={{ display: 'none' }}
                id="raised-button-file"
                multiple
                type="file"
                onChange={handleUpload}
            />
            <label htmlFor="raised-button-file">
                <Button component="span" style={{ height: "100%" }}>
                    Upload
                </Button>
            </label> 
        </Stack>
    )
}

const ImageExample = () => {
    return (
        <RadiantProvider>
            <Stack>
                <ImageLoader />
                <Box height={10} />
                <RadiantCanvas />
        </Stack>
        </RadiantProvider>
    )
}

export default ImageExample;
