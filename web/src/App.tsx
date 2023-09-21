import { RadiantToolbar } from "./components/RadiantToolbar";
import { Stack } from "@mui/material";
import { RadiantAppProvider } from "./contexts/RadiantAppContext";

function App() {
  return (
    <RadiantAppProvider>
      <div>
        <Stack direction="row" spacing={2}>
          <RadiantToolbar />
        </Stack>
      </div>
    </RadiantAppProvider>
  );
}

export default App;
