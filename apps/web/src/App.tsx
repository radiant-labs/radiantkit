import { RadiantToolbar } from "./components/RadiantToolbar";
import { Stack } from "@mui/material";
import { RadiantAppProvider } from "./contexts/RadiantAppContext";
import { RadiantPropertiesPanel } from "./components/RadiantPropertiesPanel";

function App() {
  return (
    <RadiantAppProvider>
      <div>
        <Stack direction="row" spacing={2}>
          <RadiantToolbar />
          <RadiantPropertiesPanel />
        </Stack>
      </div>
    </RadiantAppProvider>
  );
}

export default App;
