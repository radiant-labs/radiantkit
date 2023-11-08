import { BrowserRouter, Route, Routes } from 'react-router-dom';
import BasicExample from './basic';
import ImageExample from './image';
import { Button, ButtonGroup } from '@mui/material';
import TextExample from './text';

function App() {
    return (
        <div>
            <BrowserRouter>
                <Routes>
                    <Route path="/" element={<div />} />
                    <Route path="basic" element={<BasicExample />} />
                    <Route path="image" element={<ImageExample />} />
                    <Route path="text" element={<TextExample />} />
                </Routes>
            </BrowserRouter>
            <ButtonGroup orientation='vertical'>
                <Button href="/basic">Basic</Button>
                <Button href="/image">Image</Button>
                <Button href="/text">Text</Button>
            </ButtonGroup>
        </div>
    )
}

export default App
