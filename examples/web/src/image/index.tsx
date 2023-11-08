import { RadiantCanvas, RadiantProvider, useCurrentController } from 'radiant-sdk';
import { Box, Button, Stack, TextField } from '@mui/material';
import { useState } from 'react';

const ImageLoader = () => {
    const [path, setPath] = useState<string>('');
    const { controller } = useCurrentController();

   const loadImage = async (path: string) => {
        controller && controller.addImage(path);
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
