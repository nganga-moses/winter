import {Outlet} from "react-router-dom";
import SideBar from "../components/Sidebar";

import React from "react";
import TopBar from "../components/TopBar.tsx";


const ShellLayout: React.FC = () => {

    return (
        <div className="h-screen overflow-hidden flex flex-col ">
             <div className="flex flex-1 h-full text-white bg-neutral-900 overflow-hidden">
                {/* Sidebar */}
                <SideBar/>
                {/* Main Panel */}
                <div className="flex flex-col flex-1 overflow-hidden">
                    {/* Top Bar (Tauri Drag) */}
                    {/* Main Content */}
                    <div className="flex flex-1 overflow-hidden">
                        <div
                            className={`transition-all duration-300 overflow-y-auto w-full"
                        }`}
                        >
                            <Outlet/>
                        </div>

                    </div>
                </div>

            </div>
        </div>
    );
};
export default ShellLayout;