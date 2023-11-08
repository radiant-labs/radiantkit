import { RadiantCanvas, useCurrentController } from 'radiant-sdk';
import { Box, Button, Stack } from '@mui/material';

const ImageLoader = () => {
    const { controller } = useCurrentController();

   const loadImage = async (path: string) => {
        controller && controller.addImage(path);
   }

    return (
        <Stack direction="row" spacing={2}>
            <Button onClick={() => loadImage('https://i.imgur.com/XbLP6ux.png')}>Load Image</Button>
        </Stack>
    )
}

const ImageExample = () => {
    return (
        <Stack>
            <ImageLoader />
            <Box height={10} />
            <RadiantCanvas />
        </Stack>
    )
}

export default ImageExample;
