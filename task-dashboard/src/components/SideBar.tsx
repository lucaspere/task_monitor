import { CgPerformance } from "react-icons/cg";
import { NavLink, Outlet } from "react-router-dom";

export function SideBar() {
  return (
    <div className="w-screen min-h-screen flex">
      <div>
        <NavLink to="/monitor/processes" className="flex">
          <CgPerformance />
          <p>Processes</p>
        </NavLink>
        <NavLink to="/monitor/performance" className="flex">
          <CgPerformance />
          <p>Performance</p>
        </NavLink>
      </div>
      <Outlet />
    </div>
  );
}
// Compare this snippet from src/components/TopBar.tsx:
//
// export function TopBar() {
//   return (
//     <div>
//       <Text>TopBar</Text>
//     </div>
//   );
// }
