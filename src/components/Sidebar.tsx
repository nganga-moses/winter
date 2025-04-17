import { useLocation, Link } from "react-router-dom";
import {
    FaGraduationCap,
    FaCoins,
    FaUserCircle,
    FaUncharted,
    FaQrcode,
    FaCodeBranch,
    FaBug, FaQuestionCircle,
} from "react-icons/fa";
import clsx from "clsx";

interface NavItemProps {
    to: string;
    icon: JSX.Element;
    text: string;
    active: boolean;
}

const NavItem: React.FC<NavItemProps> = ({ to, icon, text, active }) => (
    <Link
        to={to}
        className={clsx(
            "group relative flex items-center space-x-2 p-3 rounded hover:bg-gray-900 transition-colors duration-150",
            active && "bg-gray-800"
        )}
    >
        <span className="text-lg">{icon}</span>
            <div className="absolute left-full top-1/2 transform -translate-y-1/2 ml-2 px-2 py-1 rounded bg-gray-800 text-white text-xs whitespace-nowrap opacity-0 group-hover:opacity-100 transition-opacity z-10 shadow-md border border-gray-700">
                {text}
            </div>

    </Link>
);

const Sidebar: React.FC = () => {
    const location = useLocation();

    return (
        <aside
            className={`h-full bg-blue-gray-900 text-gray flex flex-col justify-between border-r border-gray-900
                  transition-all duration-300 w-14`}>
            {/* Top Section - Main Navigation */}
            <div>
                <div className="flex items-center h-16">
                    <img className="pl-2" src="/icon-logo.png" alt="Winter"/>
                </div>

                <nav className="flex flex-col p-2 space-y-2">
                    <NavItem
                        to="/dashboard"
                        icon={<FaQrcode/>}
                        text="Home"
                        active={location.pathname === "/dashboard"}/>
                    <NavItem
                        to="/assistant"
                        icon={<FaUncharted/>}
                        text=" Engineering Assistant"
                        active={location.pathname === "/assistant"}/>
                    <NavItem
                        to="/docs"
                        icon={<FaQuestionCircle/>}
                        text="Documentation"
                        active={location.pathname === "/docs"}/>
                  </nav>
            </div>

            {/* Bottom Section - Settings, Commands, Profile */}
            <div className="p-2 space-y-2">
                 <NavItem to="/usage" icon={<FaCoins/>} text="Usage" active={location.pathname === "/commands"} />
                <NavItem to="/profile" icon={<FaUserCircle/>} text="Profile" active={location.pathname === "/profile"} />
            </div>
        </aside>
    );
};

export default Sidebar;