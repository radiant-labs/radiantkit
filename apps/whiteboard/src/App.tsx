import { RadiantKitProvider } from '@radiantkit/react';
import Toolbar from './components/Toolbar';

const App = () => {
  return (
    <RadiantKitProvider client_id={BigInt(4)} collaborate={true} width={undefined} height={undefined}>
      <div style={{ display: 'flex' }}>
        <div style={{ zIndex: 1 }}>
          <Toolbar />
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
      </div>
    </RadiantKitProvider>
  );
};

export default App;