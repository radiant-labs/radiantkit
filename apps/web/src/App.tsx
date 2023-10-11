import { RadiantToolbar } from './components/RadiantToolbar'
import { Stack } from '@mui/material'
import { RadiantAppProvider } from './contexts/RadiantAppContext'
import { RadiantPropertiesPanel } from './components/RadiantPropertiesPanel'

function App() {
    return (
        <RadiantAppProvider>
            <div
                id="overlay"
                style={{
                    position: 'absolute',
                    zIndex: 1,
                    display: 'flex',
                    justifyContent: 'space-between',
                    width: '100%',
                    pointerEvents: 'none',
                }}
            >
                <RadiantToolbar />
                <RadiantPropertiesPanel />
            </div>
            <div
                id="canvas-container"
                style={{
                    position: 'absolute',
                    zIndex: 0,
                    display: 'flex',
                    alignItems: 'center',
                    height: '100%',
                    justifyContent: 'center',
                    width: '100%',
                }}
            />
        </RadiantAppProvider>
    )
}

export default App
