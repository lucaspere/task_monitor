import { Charts } from "../../Charts";
import { MetricDetails } from "./MetricDetails";

export function MetricChart() {
  return (
    <div>
      <h4>Memory</h4>
      <Charts />
      <MetricDetails />
    </div>
  );
}
