import {Outlet} from "react-router-dom";
import SideBar from "../components/Sidebar";
import ChatWindow from "../components/ChatWindow.tsx";
import {FaGithub, FaRobot,FaTimes} from "react-icons/fa";
import React, {useState} from "react";

const ShellLayout: React.FC = () => {
    const [chatVisible, setChatVisible] = useState<boolean>(false);

    return(
        <div className="flex h-screen text-white bg-gray-gra-900 overflow-hidden">
            {/* Sidebar */}
            <SideBar/>
            {/* Main Panel */}
            <div className="flex flex-col flex-1 overflow-hidden">
                {/* Top Bar (Tauri Drag) */}
                <header
                    className="h-10 bg-blue-gray-900/70 backdrop-blur border-b border-gray-700 flex items-center justify-between px-4 text-sm"
                    data-tauri-drag-region=""
                    >
                    <div className="flex items-center space-x-3">
                        <span className="text-gray-400 font-semibold">Winter</span>
                        <a href="/git-integrations" className="text-gray-400 hover:text-white">
                            <FaGithub size={16} />
                        </a>
                    </div>
                    <button
                        onClick={() => setChatVisible((prev) => !prev)}
                        className="text-gray-400 hover:text-white"
                    >
                        {chatVisible ? <FaTimes size={16}/> : <FaRobot size={16}/>}
                    </button>

                </header>
                {/* Main Content */}
                <div className="flex flex-1 overflow-hidden">
                    <div
                        className={`transition-all duration-300 overflow-y-auto ${
                            chatVisible ? "w-2/3" : "w-full"
                        }`}
                    >
                        <Outlet />
                    </div>

                    {chatVisible && (
                        <div className="w-1/3 border-l border-gray-700">
                            <ChatWindow />
                        </div>
                    )}
                </div>
            </div>

        </div>
    );
};
export default ShellLayout;