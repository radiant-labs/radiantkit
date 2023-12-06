import { RadiantKitCanvas, RadiantKitProvider, useCurrentController } from '@radiantkit/react';
import { Box, Button, Stack, TextField } from '@mui/material';
import { useEffect, useState } from 'react';

const Text = () => {
    const [nodeId, setNodeId] = useState<number>(0)
    const [text, setText] = useState<string>('');

    const { controller, response } = useCurrentController();

    useEffect(() => {
        if (response?.Selected) {
            let node = response.Selected.node.Text;
            setNodeId(node.id)
        } 
    }, [response]);

    const addText = async (text: string) => {
        controller && controller.addText(text);
    }

    const editText = async (text: string) => {
        controller && controller.setText(nodeId, text);
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
            <Button onClick={() => editText(text || 'Hello World!')}>Edit Text</Button>
        </Stack>
    )
}

const TextExample = () => {
    return (
        <RadiantKitProvider width={1600} height={1200}>
            <Stack>
                <Text />
                <Box height={10} />
                <RadiantKitCanvas />
            </Stack>
        </RadiantKitProvider>
    )
}

export default TextExample;
