import { BrowserRouter, Route, Routes } from 'react-router-dom';
import BasicExample from './basic';
import ImageExample from './image';
import TextExample from './text';
import Whiteboard from './whiteboard';
import { Button, Divider, Toolbar } from '@mui/material';

function App() {
    return (
        <div>
            <div style={{ position: 'fixed', backgroundColor: 'white', zIndex: 1, width: '100%' }}>
                <Toolbar>
                    <Button href="/basic">Basic</Button>
                    <Button href="/image">Image</Button>
                    <Button href="/text">Text</Button>
                    <Button href="/whiteboard">Whiteboard</Button>
                </Toolbar>
                <Divider />
            </div>
            <BrowserRouter>
                <Routes>
                    <Route path="/" element={<BasicExample />} />
                    <Route path="basic" element={<BasicExample />} />
                    <Route path="image" element={<ImageExample />} />
                    <Route path="text" element={<TextExample />} />
                    <Route path="whiteboard" element={<Whiteboard />} />
                </Routes>
            </BrowserRouter>
        </div>
    )
}

export default App
