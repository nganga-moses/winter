import React, { useEffect, useState } from 'react';
import {
    XMarkIcon,
    MinusSmallIcon,
    PlusSmallIcon,
} from '@heroicons/react/20/solid';
import { QuestionMarkCircleIcon } from '@heroicons/react/16/solid';
import { getCurrentWindow } from '@tauri-apps/api/window';

const Titlebar: React.FC = () => {
    const [win, setWin] = useState<any>(null);

    useEffect(() => {
        if ('__TAURI_INTERNALS__' in window || '__TAURI_METADATA__' in window) {
            setWin(getCurrentWindow());
        }
    }, []);

    if (!win) return null;

    const handleClose = () => win.close();
    const handleMinimize = () => win.minimize();
    const handleMaximize = () => win.toggleMaximize();

    return (
        <div
            data-tauri-drag-region="true"
            className="flex items-center justify-between w-full h-10 px-3 text-white select-none bg-blue-gray-900"
        >
            {/* Left window controls */}
            <div className="flex items-center space-x-2">
                {[
                    {
                        onClick: handleClose,
                        color: 'bg-red-400',
                        hover: 'hover:!bg-red-500',
                        icon: <XMarkIcon className="w-2 h-2 text-black" />,
                    },
                    {
                        onClick: handleMinimize,
                        color: 'bg-yellow-500',
                        hover: 'hover:!bg-yellow-400',
                        icon: <MinusSmallIcon className="w-3 h-3 text-black" />,
                    },
                    {
                        onClick: handleMaximize,
                        color: 'bg-green-500',
                        hover: 'hover:!bg-green-500',
                        icon: <PlusSmallIcon className="w-3 h-3 text-black" />,
                    },
                ].map(({ onClick, color, hover, icon }, i) => (
                    <button
                        key={i}
                        onClick={onClick}
                        className={`w-3 h-3 ${color} ${hover} rounded-full flex items-center justify-center group relative transition-colors`}
                    >
                        <div className="absolute transition-all transform scale-90 opacity-0 group-hover:scale-125 group-hover:opacity-100">
                            {icon}
                        </div>
                    </button>
                ))}
            </div>

            {/* Center placeholder */}
            <div className="flex-1 text-center text-xs text-gray-400 font-mono tracking-wider pointer-events-none" />

            {/* Right-side help icon */}
            <div className="flex items-center justify-end w-20">
                <button className="group p-1 rounded-md transition">
                    <QuestionMarkCircleIcon className="h-5 w-5 text-gray-500 group-hover:text-blue-400 group-hover:scale-110 transition-all" />
                </button>
            </div>
        </div>
    );
};

export default Titlebar;