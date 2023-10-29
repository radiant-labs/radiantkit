import { RadiantToolbar } from './components/RadiantToolbar'
import { RadiantPropertiesPanel } from './components/RadiantPropertiesPanel'
import { RadiantCanvas, RadiantProvider } from 'radiant-sdk';

function App() {
    return (
        <RadiantProvider>
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
            <RadiantCanvas />
        </RadiantProvider>
    )
}

export default App
