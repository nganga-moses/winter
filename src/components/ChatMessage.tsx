import React from "react";
import ReactMarkdown from "react-markdown";
import { Prism as SyntaxHighlighter } from "react-syntax-highlighter";
import { materialDark } from "react-syntax-highlighter/dist/esm/styles/prism";

interface ChatMessageProps {
    sender: "user" | "agent";
    text: string;
    isStreaming?: boolean;
}

// FIXED: define CodeBlock with proper typing
const CodeBlock: React.FC<React.ComponentPropsWithoutRef<"code">> = ({
                                                                         className,
                                                                         inline,
                                                                         children,
                                                                     }) => {
    const match = /language-(\w+)/.exec(className || "");
    const language = match?.[1] || "text";
    const code = String(children).trim();

    if (inline) {
        return (
            <code className="bg-gray-700 px-1 py-0.5 rounded text-xs font-mono text-white">
                {code}
            </code>
        );
    }

    return (
        <SyntaxHighlighter
            language={language}
            style={materialDark}
            customStyle={{
                padding: "1em",
                borderRadius: "0.5em",
                fontSize: "0.9em",
                backgroundColor: "#1e1e1e",
            }}
        >
            {code}
        </SyntaxHighlighter>
    );
};

const ChatMessage: React.FC<ChatMessageProps> = ({ sender, text, isStreaming }) => {
    const isUser = sender === "user";

    return (
        <div
            className={`p-3 rounded-lg text-sm whitespace-pre-wrap max-w-3xl ${
                isUser ? "bg-gray-800 text-white self-end" : "bg-gray-700 text-gray-100 self-start"
            } ${isStreaming ? "animate-pulse" : ""}`}
        >
            <ReactMarkdown components={{ code: CodeBlock }}>{text}</ReactMarkdown>
        </div>
    );
};

export default ChatMessage;