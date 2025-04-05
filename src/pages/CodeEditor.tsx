import {useEffect, useState} from "react";
import MonacoEditor from "../components/MonacoEditor";
import FileExplorer from "../components/FileExplorer";
import {listFiles, readFile, writeFile} from "../api/filesystem.tsx";

const CodeEditorPage = () => {
    const [fileList, setFileList] = useState<string[]>([]);
    const [selectedFile, setSelectedFile] = useState<string>("");
    const [fileContents, setFileContents] = useState<string>("");

    useEffect(() => {
        const load = async () => {
            const files = await listFiles();
            setFileList(files);
            if (files.length > 0) {
                const content = await readFile(files[0]);
                setSelectedFile(files[0]);
                setFileContents(content);
            }
        };
        load();
    }, []);

    const handleChange = (newVal: string | undefined)=> {
        setFileContents(newVal || "");
        if (selectedFile) {
            writeFile(selectedFile, newVal || "").catch(console.error);
        }
    };
    const handleSelectFile = async (filename: string)=>{
        const content = await readFile(filename);
        setSelectedFile(filename);
        setFileContents(content);
    };

    return (
        <div className="flex h-full">
            <FileExplorer
                files={fileList}
                onSelect={handleSelectFile}
                selected={selectedFile}
            />
            <div className="flex-1">
                <MonacoEditor
                    fileName={selectedFile}
                    content={fileContents}
                    onChange={handleChange}
                />
            </div>
        </div>
    );
};

export default CodeEditorPage;