// Assistant.tsx
import React, { useState, useRef, useEffect, useCallback, ChangeEvent } from 'react';
import { FaArrowUp, FaPaperclip, FaUncharted } from 'react-icons/fa';

interface Message {
    id: number;
    text: string;
    sender: 'user' | 'bot';
    streaming?: boolean;
}

const allowedExtensions = ['.pdf', '.txt', '.py', '.js', '.ts', '.tsx', '.json', '.md', '.yaml', '.yml'];

const Assistant: React.FC = () => {
    const [message, setMessage] = useState('');
    const [chatHistory, setChatHistory] = useState<Message[]>([]);
    const [uploadedFiles, setUploadedFiles] = useState<File[]>([]);
    const [inputAreaHeight, setInputAreaHeight] = useState(80);
    const [streaming, setStreaming] = useState(false);

    const chatEndRef = useRef<HTMLDivElement>(null);
    const textareaRef = useRef<HTMLTextAreaElement>(null);
    const inputAreaRef = useRef<HTMLDivElement>(null);

    const apiUrl = (window as any).__API_URL__ || 'http://127.0.0.1:6144';

    const adjustTextareaHeight = useCallback(() => {
        const textarea = textareaRef.current;
        if (textarea) {
            textarea.style.height = 'auto';
            const scrollHeight = textarea.scrollHeight;
            const maxHeight = 200;
            textarea.style.height = `${Math.min(scrollHeight, maxHeight)}px`;
            textarea.style.overflowY = scrollHeight > maxHeight ? 'auto' : 'hidden';
        }
        if (inputAreaRef.current) {
            setTimeout(() => {
                setInputAreaHeight(inputAreaRef.current!.offsetHeight);
            }, 0);
        }
    }, []);

    useEffect(() => {
        adjustTextareaHeight();
        window.addEventListener('resize', adjustTextareaHeight);
        return () => window.removeEventListener('resize', adjustTextareaHeight);
    }, [adjustTextareaHeight]);

    useEffect(() => {
        chatEndRef.current?.scrollIntoView({ behavior: 'smooth' });
    }, [chatHistory]);

    const handleInputChange = (e: ChangeEvent<HTMLTextAreaElement>) => setMessage(e.target.value);
    const handleKeyDown = (e: React.KeyboardEvent<HTMLTextAreaElement>) => {
        if (e.key === 'Enter' && !e.shiftKey) {
            e.preventDefault();
            handleSendMessage();
        }
    };

    const handleFileChange = (e: ChangeEvent<HTMLInputElement>) => {
        const files = Array.from(e.target.files || []);
        const validFiles = files.filter(file =>
            allowedExtensions.some(ext => file.name.toLowerCase().endsWith(ext))
        );
        setUploadedFiles(prev => [...prev, ...validFiles]);
    };

    const handleSendMessage = async () => {
        if (!message.trim() && uploadedFiles.length === 0) return;

        const userMsg: Message = { id: Date.now(), text: message, sender: 'user' };
        setChatHistory(prev => [...prev, userMsg]);
        setMessage('');
        setUploadedFiles([]);
        setStreaming(true);

        // Add placeholder for bot response (streaming)
        const botMsg: Message = { id: Date.now() + 1, text: '', sender: 'bot', streaming: true };
        setChatHistory(prev => [...prev, botMsg]);

        const eventSource = new EventSource(`${apiUrl}/assistant?text=${encodeURIComponent(message)}&preview=false`);

        eventSource.onmessage = (event) => {
            if (event.data === '[[END]]') {
                setStreaming(false);
                eventSource.close();
                setChatHistory(prev =>
                    prev.map(m => m.streaming ? { ...m, streaming: false } : m)
                );
                return;
            }

            setChatHistory(prev =>
                prev.map(m =>
                    m.streaming ? { ...m, text: m.text + event.data } : m
                )
            );
        };

        eventSource.onerror = () => {
            console.error('SSE error');
            setStreaming(false);
            setChatHistory(prev => [
                ...prev,
                {
                    id: Date.now() + 999,
                    text: '❌ Assistant encountered an error. Please try again.',
                    sender: 'bot',
                },
            ]);
            eventSource.close();
        };
    };

    return (
        <div className="flex h-full w-screen bg-blue-gray-900">
            <div className="flex flex-col text-gray-300 w-full">
                <div className="flex-1 overflow-y-auto px-4 max-w-4xl w-full mx-auto"
                     style={{ paddingBottom: `${inputAreaHeight + 16}px` }}>
                    {chatHistory.length === 0 && (
                        <div className="text-center text-gray-500 mt-10">How can I help?</div>
                    )}
                    {chatHistory.map((msg) => (
                        <div key={msg.id} className="my-4">
                            {msg.sender === 'bot' ? (
                                <div className="flex items-start space-x-3">
                                    <div className="w-6 h-6 rounded-full bg-gray-600 flex items-center justify-center mt-1">
                                        <FaUncharted size={14} />
                                    </div>
                                    <div className="min-w-0 text-gray-400 whitespace-pre-wrap">{msg.text}</div>
                                </div>
                            ) : (
                                <div className="flex justify-end items-end space-x-2">
                                    <div className="max-w-lg px-4 py-3 rounded-lg shadow bg-gray-800 text-gray-100">
                                        <p className="whitespace-pre-wrap">{msg.text}</p>
                                    </div>
                                    <div className="w-8 h-8 rounded-full bg-teal-600 flex items-center justify-center text-white font-bold">
                                        U
                                    </div>
                                </div>
                            )}
                        </div>
                    ))}
                    <div ref={chatEndRef} />
                </div>

                {/* Input */}
                <div ref={inputAreaRef}
                     className="absolute bottom-0 left-0 right-0 px-4 pb-4 pt-2 bg-gradient-to-t from-gray-900 via-gray-900 to-transparent">
                    <div className="w-full max-w-3xl mx-auto">
                        <div className="flex items-end p-3 bg-gray-800 rounded-2xl border border-gray-600 shadow-lg">
                            <label htmlFor="fileUpload" className="cursor-pointer p-2 text-gray-400 hover:text-white">
                                <FaPaperclip size={16} />
                                <input
                                    id="fileUpload"
                                    type="file"
                                    multiple
                                    accept={allowedExtensions.join(',')}
                                    className="hidden"
                                    onChange={handleFileChange}
                                />
                            </label>
                            <textarea
                                ref={textareaRef}
                                rows={1}
                                value={message}
                                onChange={handleInputChange}
                                onKeyDown={handleKeyDown}
                                placeholder="Message..."
                                className="flex-grow bg-transparent text-gray-200 text-sm placeholder-gray-400 border-none resize-none focus:outline-none mx-2"
                            />
                            <button
                                onClick={handleSendMessage}
                                disabled={streaming || (!message.trim() && uploadedFiles.length === 0)}
                                className={`p-2 rounded-md transition-colors ${
                                    message.trim() || uploadedFiles.length
                                        ? 'bg-blue-700 text-white hover:bg-blue-600'
                                        : 'bg-gray-600 text-gray-400 cursor-not-allowed'
                                }`}
                            >
                                <FaArrowUp size={16} />
                            </button>
                        </div>
                        {uploadedFiles.length > 0 && (
                            <div className="mt-2 text-xs text-gray-400">
                                {uploadedFiles.map((file) => (
                                    <div key={file.name}>📎 {file.name}</div>
                                ))}
                            </div>
                        )}
                    </div>
                </div>
            </div>
        </div>
    );
};

export default Assistant;