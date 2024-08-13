import { SideBar } from "./SideBar";
import { TopBar } from "./TopBar";

export function Monitor() {
  return (
    <div className="container max-md bg-neutral-800 box-content p-2.5 my-auto mx-0 w-lvw">
      <div className="flex bg-slate-500">
        <h1>Monitor</h1>
        <TopBar />
      </div>
      <br className="" />
      <div className="flex justify-between">
        <SideBar />
        {/* <Metric /> */}
      </div>
    </div>
  );
}
