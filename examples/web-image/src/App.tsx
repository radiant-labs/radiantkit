import { RadiantCanvas, RadiantProvider, useCurrentController } from 'radiant-sdk';
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

function App() {
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

export default App
