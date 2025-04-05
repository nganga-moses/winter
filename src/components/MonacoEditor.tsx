import Editor, {OnMount} from "@monaco-editor/react";
import {useRef} from "react";

interface MonacoEditorProps {
    fileName: string;
    content: string;
    onChange: (newValue: string | undefined) => void;
}

const MonacoEditor: React.FC<MonacoEditorProps> = ({fileName, content, onChange}) => {
    const extension = fileName.split(".").pop();
    const languageMap: Record<string, string> = {
        ts: "typescript",
        tsx: "typescript",
        js: "javascript",
        jsx: "javascript",
        py: "python",
        json: "json",
        md: "markdown",
        html: "html",
        css: "css"
    };
    const editorRef = useRef<any>(null);

    const handleMount: OnMount = (editor, monaco) => {
        editorRef.current = editor;

        editor.addAction({
            id: "winter-review-code",
            label: "Review this code with Winter",
            contextMenuGroupId: "navigation",
            contextMenuOrder: 1,
            run: () => {
                const selectedCode = editor.getModel().getValueInRange(editor.getSelections());
                if (selectedCode.trim()) {
                    //Trigger Agent Code Review
                    window.dispatchEvent(new CustomEvent("winter-code-review", {
                            detail: {
                                code: selectedCode,
                                file: fileName
                            },
                        })
                    );
                }
            },
        });
    };
    return (
        <Editor
            height="100%"
            defaultLanguage="python"
            value={content}
            onChange={onChange}
            theme="vs-dark"
            onMount={handleMount}
            options={{
                fontSize: 14,
                minimap: {enabled: false},
                scrollBeyondLastLine: false,
                automaticLayout: true,
            }}
        />
    );
};

export default MonacoEditor;