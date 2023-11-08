import { BrowserRouter, Route, Routes } from 'react-router-dom';
import BasicExample from './basic';
import ImageExample from './image';
import { Button, ButtonGroup } from '@mui/material';

function App() {
    return (
        <div>
            <BrowserRouter>
                <Routes>
                    <Route path="/" element={<div />} />
                    <Route path="basic" element={<BasicExample />} />
                    <Route path="image" element={<ImageExample />} />
                </Routes>
            </BrowserRouter>
            <ButtonGroup orientation='vertical'>
                <Button href="/basic">Basic</Button>
                <Button href="/image">Image</Button>
            </ButtonGroup>
        </div>
    )
}

export default App
