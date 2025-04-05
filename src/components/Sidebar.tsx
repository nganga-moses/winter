import { useLocation, Link } from "react-router-dom";
import { useState } from "react";
import {
    FaGraduationCap,
    FaCoins,
    FaUserCircle,
    FaUncharted,
    FaQrcode,
    FaCodeBranch,
    FaBug,
    FaCompressArrowsAlt
} from "react-icons/fa";
import clsx from "clsx";

interface NavItemProps {
    to: string;
    icon: JSX.Element;
    text: string;
    expanded: boolean;
    active: boolean;
}

const NavItem: React.FC<NavItemProps> = ({ to, icon, text, expanded, active }) => (
    <Link
        to={to}
        className={clsx(
            "flex items-center space-x-2 p-3 rounded hover:bg-gray-900 transition-colors duration-150",
            active && "bg-gray-800"
        )}
    >
        <span className="text-lg ">{icon}</span>
        {expanded && <span className="text-sm">{text}</span>}
    </Link>
);

const Sidebar: React.FC = () => {
    const [expanded, setExpanded] = useState<boolean>(false);
    const location = useLocation();

    return (
        <aside
            className={`h-screen bg-blue-gray-900 text-gray flex flex-col justify-between border-r border-gray-900
                  transition-all duration-300 w-14`}

        >
            {/* Top Section - Main Navigation */}
            <div>
                <div className="flex items-center h-16">
                    <img className="w-12 h-10 pl-2" src="/logo.png" alt="CoFounder"/>{expanded ?
                    <span
                        className="text-lg font-bold pt-2 pl-3">Leo</span> : ""

                }
                </div>

                <nav className="flex flex-col p-2 space-y-2">
                    <NavItem
                        to="/dashboard"
                        icon={<FaQrcode/>}
                        text="Home"
                        expanded={expanded}
                        active={location.pathname === "/dashboard"}/>
                    <NavItem
                        to="/assistant"
                        icon={<FaUncharted/>}
                        text="Assistant"
                        expanded={expanded}
                        active={location.pathname === "/assistant"}/>
                    <NavItem
                        to="/docs"
                        icon={<FaGraduationCap/>}
                        text="Documentation"
                        expanded={expanded}
                        active={location.pathname === "/docs"}/>
                  </nav>
            </div>

            {/* Bottom Section - Settings, Commands, Profile */}
            <div className="p-2 space-y-2">
                <NavItem to="/bugs" icon={<FaBug/>} text="Bugs" active={location.pathname === "/bugs"} expanded={expanded}/>
                <NavItem to="/repo" icon={<FaCodeBranch/>} text="Github" active={location.pathname === "/repo"} expanded={expanded}/>
                <NavItem to="/usage" icon={<FaCoins/>} text="Usage" active={location.pathname === "/commands"} expanded={expanded}/>
                <NavItem to="/profile" icon={<FaUserCircle/>} text="Profile" active={location.pathname === "/profile"} expanded={expanded}/>
            </div>
        </aside>
    );
};

export default Sidebar;