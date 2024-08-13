import { TimerIcon } from "@radix-ui/react-icons";
import { NavLink } from "react-router-dom";
const metricList = ["CPU", "Memory"];
export function MetricList() {
  return (
    <div className="flex flex-col">
      {metricList.map((metric) => (
        <NavLink to={metric.toLocaleLowerCase()} key={metric}>
          <TimerIcon />
          {metric}
        </NavLink>
      ))}
    </div>
  );
}
