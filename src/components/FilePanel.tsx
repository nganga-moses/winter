// src/components/FilePanel.tsx
import React from 'react';
import { FaUpload, FaFileAlt, FaTrash } from 'react-icons/fa';

// Example file structure
interface UploadedFile {
    id: string;
    name: string;
    size: number; // in bytes
}

const FilePanel: React.FC = () => {
    // Placeholder State & Logic
    const [uploadedFiles, setUploadedFiles] = React.useState<UploadedFile[]>([
        { id: '1', name: 'document_analysis.pdf', size: 1024 * 500 },
        { id: '2', name: 'image_data_very_long_name_to_test_truncation.png', size: 1024 * 120 },
        { id: '3', name: 'report.docx', size: 1024 * 75 },
    ]);

    const handleFileUpload = (event: React.ChangeEvent<HTMLInputElement>) => {
        const files = event.target.files;
        if (files && files.length > 0) {
            // TODO: Implement actual file upload logic
            const newFile: UploadedFile = { id: Date.now().toString(), name: files[0].name, size: files[0].size };
            setUploadedFiles(prev => [...prev, newFile]);
            console.log('Uploading:', files[0]);
        }
    };

    const handleDeleteFile = (fileId: string) => {
        // TODO: Implement actual delete logic
        setUploadedFiles(prev => prev.filter(f => f.id !== fileId));
        console.log('Deleting file ID:', fileId);
    };

    const formatBytes = (bytes: number, decimals = 2): string => {
        if (bytes === 0) return '0 Bytes';
        const k = 1024;
        const dm = decimals < 0 ? 0 : decimals;
        const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB'];
        const i = Math.floor(Math.log(bytes) / Math.log(k));
        return parseFloat((bytes / Math.pow(k, i)).toFixed(dm)) + ' ' + sizes[i];
    }

    return (
        // Panel container using blue-gray
        <div className="h-full w-full bg-blue-gray-800 text-blue-gray-200 flex flex-col p-4 border-l border-blue-gray-700">
            <h2 className="text-lg font-semibold mb-4 text-blue-gray-100 flex-shrink-0">Files</h2>

            {/* Upload Area */}
            <div className="mb-4 flex-shrink-0">
                <label
                    htmlFor="file-upload"
                    className="w-full flex items-center justify-center px-4 py-2 border-2 border-dashed border-blue-gray-600 rounded-md cursor-pointer hover:border-blue-gray-500 hover:bg-blue-gray-700 transition-colors"
                >
                    <FaUpload className="mr-2 text-blue-gray-400" />
                    <span className="text-sm text-blue-gray-300">Click to upload files</span>
                </label>
                <input id="file-upload" name="file-upload" type="file" className="sr-only" onChange={handleFileUpload} multiple />
            </div>

            {/* File List Area - Takes remaining space and scrolls */}
            <div className="flex-grow overflow-y-auto space-y-2 pr-1"> {/* Scrollable area */}
                {uploadedFiles.length === 0 && (
                    <p className="text-sm text-blue-gray-500 text-center mt-4">No files uploaded yet.</p>
                )}
                {uploadedFiles.map((file) => (
                    <div key={file.id} className="flex items-center justify-between p-2 bg-blue-gray-700 rounded-md hover:bg-blue-gray-600">
                        <div className="flex items-center min-w-0 mr-2">
                            <FaFileAlt className="text-blue-gray-400 mr-2 flex-shrink-0" />
                            <span className="text-sm truncate" title={file.name}>{file.name}</span>
                        </div>
                        <div className="flex items-center flex-shrink-0">
                            <span className="text-xs text-blue-gray-400 mr-2">{formatBytes(file.size)}</span>
                            <button onClick={() => handleDeleteFile(file.id)} className="text-blue-gray-500 hover:text-red-500 focus:outline-none" aria-label={`Delete ${file.name}`}><FaTrash size={14}/></button>
                        </div>
                    </div>
                ))}
            </div>
        </div>
    );
};

export default FilePanel;