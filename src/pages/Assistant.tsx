// Assistant.tsx
import React, { useState, useRef, useEffect, useCallback, ChangeEvent } from 'react';
import { FaPlus, FaMicrophone, FaArrowUp, FaRobot, FaPaperclip } from 'react-icons/fa';

interface Message {
    id: number;
    text: string;
    sender: 'user' | 'bot';
}

const allowedExtensions = ['.pdf', '.txt', '.py', '.js', '.ts', '.tsx', '.json', '.md', '.yaml', '.yml'];

const Assistant: React.FC = () => {
    const [message, setMessage] = useState('');
    const [chatHistory, setChatHistory] = useState<Message[]>([]);
    const [uploadedFiles, setUploadedFiles] = useState<File[]>([]);
    const [inputAreaHeight, setInputAreaHeight] = useState(80);

    const chatEndRef = useRef<HTMLDivElement>(null);
    const textareaRef = useRef<HTMLTextAreaElement>(null);
    const inputAreaRef = useRef<HTMLDivElement>(null);

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
                if (inputAreaRef.current) {
                    setInputAreaHeight(inputAreaRef.current.offsetHeight);
                }
            }, 0);
        }
    }, []);

    useEffect(() => {
        adjustTextareaHeight();
        const handleResize = () => adjustTextareaHeight();
        window.addEventListener('resize', handleResize);
        return () => window.removeEventListener('resize', handleResize);
    }, [adjustTextareaHeight]);

    useEffect(() => {
        setTimeout(() => chatEndRef.current?.scrollIntoView({ behavior: 'smooth' }), 100);
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

        // Send to backend
        const formData = new FormData();
        formData.append('message', message);
        uploadedFiles.forEach(file => formData.append('files', file));

        try {
            const res = await fetch(`${import.meta.env.VITE_API_URL}/assistant/message`, {
                method: 'POST',
                body: formData,
            });
            const data = await res.json();
            const botMsg: Message = { id: Date.now() + 1, text: data.response || 'No response.', sender: 'bot' };
            setChatHistory(prev => [...prev, botMsg]);
        } catch (err) {
            console.error('Assistant error:', err);
            setChatHistory(prev => [
                ...prev,
                { id: Date.now() + 1, text: '❌ Error from assistant. Try again later.', sender: 'bot' },
            ]);
        }
    };

    return (
        <div className="flex h-full">
            <div className="w-2/3 h-full flex flex-col relative bg-gray-900 text-gray-300">
                <div
                    className="flex-1 min-h-0 overflow-y-auto px-4 space-y-6 w-full max-w-3xl mx-auto"
                    style={{ paddingBottom: `${inputAreaHeight + 16}px` }}
                >
                    {chatHistory.length === 0 && <div className="text-center text-gray-500 mt-10">How can I help?</div>}
                    {chatHistory.map(msg => (
                        <div key={msg.id}>
                            {msg.sender === 'bot' ? (
                                <div className="flex items-start space-x-3">
                                    <div className="w-6 h-6 rounded-full bg-gray-600 flex items-center justify-center mt-1">
                                        <FaRobot size={14} />
                                    </div>
                                    <div className="min-w-0">
                                        <p className="break-words text-gray-400 whitespace-pre-wrap">{msg.text}</p>
                                    </div>
                                </div>
                            ) : (
                                <div className="flex justify-end items-end space-x-2">
                                    <div className="max-w-lg px-4 py-4 rounded-lg shadow bg-gray-800 text-gray-100">
                                        <p className="whitespace-pre-wrap">{msg.text}</p>
                                    </div>
                                    <div className="w-8 h-8 rounded-full bg-teal-600 flex items-center justify-center text-white font-bold">U</div>
                                </div>
                            )}
                        </div>
                    ))}
                    <div ref={chatEndRef} />
                </div>

                {/* Input */}
                <div
                    ref={inputAreaRef}
                    className="absolute bottom-0 left-0 right-0 px-4 pb-4 pt-2 bg-gradient-to-t from-gray-900 via-gray-900 to-transparent"
                >
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
                                style={{ overflowY: 'hidden' }}
                            />
                            <button
                                onClick={handleSendMessage}
                                disabled={!message.trim() && uploadedFiles.length === 0}
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
                                {uploadedFiles.map(file => (
                                    <div key={file.name}>📎 {file.name}</div>
                                ))}
                            </div>
                        )}
                    </div>
                </div>
            </div>

            {/* File Panel */}
            <div className="w-1/3 h-full flex-shrink-0">
                {/* TODO: Still connected to FilePanel.tsx */}
            </div>
        </div>
    );
};

export default Assistant;