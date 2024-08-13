import { Outlet } from "react-router-dom";
import { MetricList } from "./metrics/MetricList";

export function Performance() {
  return (
    <div className="flex flex-col bg-neutral-900 w-screen items-start">
      <p className="mb-5 font-normal text-gray-500 lg:text-xl sm:px-16 xl:px-48 dark:text-gray-400">
        Performance
      </p>
      <div className="flex">
        <MetricList />
        <Outlet />
      </div>
    </div>
  );
}
