import React from "react";

interface FileExplorerProps {
    files: string[];
    onSelect: (filename: string) => void;
    selected: string;
}

const FileExplorer: React.FC<FileExplorerProps> = ({ files, onSelect, selected }) => {
    return (
        <div className="bg-gray-900 w-64 p-2 border-r border-gray-700 overflow-y-auto">
            <h2 className="text-sm font-bold text-gray-400 mb-2">Files</h2>
            <ul className="space-y-1">
                {files.map((file) => (
                    <li
                        key={file}
                        onClick={() => onSelect(file)}
                        className={`cursor-pointer px-2 py-1 rounded text-sm ${
                            selected === file ? "bg-gray-700 text-white" : "text-gray-400 hover:bg-gray-800"
                        }`}
                    >
                        {file}
                    </li>
                ))}
            </ul>
        </div>
    );
};

export default FileExplorer;