import { useEffect, useRef, useState } from "react";
import { FaArrowCircleUp, FaRobot, FaSpinner } from "react-icons/fa";
import ChatMessage from "./ChatMessage";
import { useParams } from "react-router-dom";
import { v4 as uuidv4 } from "uuid";

interface Message {
    sender: "user" | "agent";
    text: string;
    isStreaming?: boolean;
    timestamp?: string;
    status?: string;
    threadId?: string;
    phase?: string;
    summary?: string;
}

const statusMap: Record<string, string> = {
    interpreting: "Reviewing your message...",
    thinking: "Thinking...",
    planning: "Planning...",
    building: "Building code...",
    reviewing: "Reviewing changes...",
    scaffolding: "Scaffolding...",
    idle: "Vibing...",
};

const ChatWindow: React.FC = () => {
    const { id: projectId } = useParams();
    const [messages, setMessages] = useState<Message[]>([]);
    const [input, setInput] = useState("");
    const [streamingMsg, setStreamingMsg] = useState<string | null>(null);
    const [statusLabel, setStatusLabel] = useState<string | null>(null);
    const [isStreaming, setIsStreaming] = useState(false);
    const [threads, setThreads] = useState<{ id: string; summary: string; phase?: string }[]>([]);
    const [activeThreadId, setActiveThreadId] = useState<string | null>(null);
    const messagesEndRef = useRef<HTMLDivElement>(null);

    // Load messages on mount
    useEffect(() => {
        const stored = localStorage.getItem(`chat_${projectId}`);
        if (stored) {
            try {
                const parsed = JSON.parse(stored) as Message[];
                setMessages(parsed);
            } catch (e) {
                console.error("❌ Failed to parse saved messages:", e);
            }
        }
    }, [projectId]);

    // Auto-scroll on update
    useEffect(() => {
        messagesEndRef.current?.scrollIntoView({ behavior: "smooth" });
    }, [messages, streamingMsg]);

    // Regenerate thread list
    useEffect(() => {
        const grouped = new Map<string, { summary: string; phase?: string }>();
        for (const msg of messages) {
            if (!msg.threadId) continue;
            if (!grouped.has(msg.threadId)) {
                grouped.set(msg.threadId, {
                    summary: msg.summary || msg.text.slice(0, 40),
                    phase: msg.phase,
                });
            }
        }

        const uniqueThreads = Array.from(grouped.entries()).map(([id, meta]) => ({
            id,
            ...meta,
        }));

        setThreads(uniqueThreads);

        if (!activeThreadId && uniqueThreads.length > 0) {
            setActiveThreadId(uniqueThreads[uniqueThreads.length - 1].id);
        }
    }, [messages]);

    const sendMessage = async () => {
        if (!input.trim() || isStreaming) return;

        const currentThreadId = activeThreadId ?? uuidv4();
        if (!activeThreadId) setActiveThreadId(currentThreadId);

        const userMsg: Message = {
            sender: "user",
            text: input,
            timestamp: new Date().toISOString(),
            threadId: currentThreadId,
        };

        setMessages((prev) => {
            const updated = [...prev, userMsg];
            localStorage.setItem(`chat_${projectId}`, JSON.stringify(updated));
            return updated;
        });

        setInput("");
        setStreamingMsg("");
        setStatusLabel(statusMap["thinking"]);
        setIsStreaming(true);

        const threadQuery = currentThreadId ? `&threadId=${encodeURIComponent(currentThreadId)}` : "";

        const eventSource = new EventSource(
            `${import.meta.env.VITE_API_URL}/agent/chat/${projectId}?text=${encodeURIComponent(userMsg.text)}${threadQuery}`
        );

        let temp = "";

        eventSource.onmessage = (event) => {
            if (event.data.startsWith("status:")) {
                const statusKey = event.data.replace("status:", "").trim();
                setStatusLabel(statusMap[statusKey] || "Working...");
                return;
            }

            if (event.data === "[[END]]") {
                const finalMsg: Message = {
                    sender: "agent",
                    text: temp,
                    timestamp: new Date().toISOString(),
                    threadId: currentThreadId,
                };

                setMessages((prev) => {
                    const updated = [...prev, finalMsg];
                    localStorage.setItem(`chat_${projectId}`, JSON.stringify(updated));
                    return updated;
                });

                setStreamingMsg(null);
                setStatusLabel(null);
                setIsStreaming(false);
                eventSource.close();
                return;
            }

            temp += event.data;
            setStreamingMsg(temp);
        };

        eventSource.onerror = () => {
            console.error("❌ SSE connection error");
            eventSource.close();
            setStatusLabel("⚠️ Connection lost");
            setIsStreaming(false);
        };
    };

    return (
        <div className="flex flex-col h-full bg-[#1E1E1E]">
            {/* Title Bar */}
            <div className="h-10 bg-[#252526] px-4 flex items-center border-b border-[#333333]" data-tauri-drag-region>
                <FaRobot className="text-[#007ACC] mr-2" />
                <span className="text-sm text-[#CCCCCC] font-medium">Leo</span>
                {statusLabel && <span className="ml-auto text-xs text-[#007ACC] animate-pulse">{statusLabel}</span>}
            </div>

            {/* Threads */}
            <div className="flex space-x-2 px-4 py-2 overflow-x-auto text-sm border-b border-[#333333] bg-[#252526]">
                {threads.map((thread) => (
                    <button
                        key={thread.id}
                        onClick={() => setActiveThreadId(thread.id)}
                        className={`px-2 py-1 rounded text-xs ${
                            thread.id === activeThreadId ? "bg-[#007ACC] text-white" : "bg-[#333333] text-[#CCCCCC]"
                        }`}
                    >
                        {thread.summary || "Unnamed"}
                    </button>
                ))}
            </div>

            {/* Chat Messages */}
            <div className="flex-1 px-4 py-2 overflow-y-auto space-y-4">
                {messages
                    .filter((msg) => msg.threadId === activeThreadId)
                    .map((msg, i) => (
                        <ChatMessage key={i} sender={msg.sender} text={msg.text} />
                    ))}
                {streamingMsg && <ChatMessage sender="agent" text={streamingMsg} isStreaming />}
                <div ref={messagesEndRef} />
            </div>

            {/* Input */}
            <div className="p-4 border-t border-[#333333] bg-[#252526]">
                <div className="flex items-center bg-[#333333] rounded-lg">
                    <input
                        type="text"
                        placeholder="Ask Leo anything..."
                        className="flex-1 bg-transparent text-[#CCCCCC] px-4 py-3 rounded-lg outline-none"
                        value={input}
                        onChange={(e) => setInput(e.target.value)}
                        onKeyUp={(e) => e.key === "Enter" && sendMessage()}
                    />
                    <button
                        onClick={sendMessage}
                        className="p-2 text-[#007ACC] hover:text-[#0098FF] disabled:opacity-50 disabled:cursor-not-allowed"
                        disabled={isStreaming || !input.trim()}
                    >
                        {isStreaming ? (
                            <FaSpinner className="animate-spin" />
                        ) : (
                            <FaArrowCircleUp className="text-xl" />
                        )}
                    </button>
                </div>
            </div>
        </div>
    );
};

export default ChatWindow;