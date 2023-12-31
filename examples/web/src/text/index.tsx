import { RadiantKitCanvas, RadiantKitProvider, useCurrentController } from '@radiantkit/react';
import { Box, Button, Stack, TextField } from '@mui/material';
import { useState } from 'react';

const Text = () => {
    const [text, setText] = useState<string>('');

    const { controller } = useCurrentController();

    const addText = async (text: string) => {
        controller && controller.addText(text);
    }

    return (
        <Stack direction="row" spacing={2}>
            <TextField 
                style={{ width: 300 }}
                label="Text" 
                variant="outlined"
                placeholder='Hello World!' 
                value={text} 
                onChange={(e) => setText(e.target.value)} />
            <Button onClick={() => addText(text || 'Hello World!')}>Add Text</Button>
        </Stack>
    )
}

const TextExample = () => {
    return (
        <RadiantKitProvider width={1600} height={1200}>
            <Stack>
                <Box height={80} />
                <Text />
                <Box height={10} />
                <RadiantKitCanvas />
            </Stack>
        </RadiantKitProvider>
    )
}

export default TextExample;
