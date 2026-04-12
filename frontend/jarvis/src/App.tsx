import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [isCollapsed, setIsCollapsed] = useState(false);  

  async function handleStart(){
    setIsCollapsed(true);
    await invoke("start");
  }

  // Collapsed bar UI - renders after fold animation completes
  if (isCollapsed) {
    return (
      <main className="collapsed-bar">
        <h3 style={{ margin: 12, color: "white" }}>J.A.R.V.I.S</h3>
        <button onClick={async () => {
          setIsCollapsed(false);
          await invoke("expand");
        }}>Expand</button>
      </main>
    );
  }

  // Original full UI
  return (
    <main className={`container ${isCollapsed ? "collapsing" : ""}`}>
      <h1 style={{color: "white"}}>J.A.R.V.I.S</h1>

      {/* Start button - triggers smooth fold animation */}
      <button
        onClick={handleStart}
        className="start-button"
        style={{ marginTop: "2rem" }}
      >
        Start
      </button>
    </main>
  );
}

export default App;
