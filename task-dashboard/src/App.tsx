import { BrowserRouter, Route, Routes } from "react-router-dom";
import "./App.css";
import { Monitor } from "./components/Monitor";
import { Performance } from "./components/Performance";
import { MetricChart } from "./components/metrics/MetricChart";

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/monitor" element={<Monitor />}>
          <Route path="performance" element={<Performance />}>
            <Route path="cpu" element={<MetricChart />} />
            <Route path="memory" element={<MetricChart />} />
          </Route>
          <Route path="processes" element={<div>Processes</div>} />
        </Route>
      </Routes>
    </BrowserRouter>
  );
}

export default App;
