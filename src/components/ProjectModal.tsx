import { useState, ChangeEvent, FormEvent } from "react";
import { XMarkIcon } from "@heroicons/react/24/outline";
import { apiRequest } from "../api/api";

interface Project {
    id?: string;
    name: string;
    description: string;
    instructions: string;
    privacy: "public" | "private";
}

interface ProjectModalProps {
    onClose: () => void;
    onSave: () => void;
    project?: Project | null;
}

const ProjectModal = ({ onClose, onSave, project }: ProjectModalProps) => {
    const [formData, setFormData] = useState<Project>({
        name: project?.name || "",
        description: project?.description || "",
        instructions: project?.instructions || "",
        privacy: project?.privacy || "public",
        id: project?.id,
    });

    const [errors, setErrors] = useState<Record<string, string>>({});
    const [isSaving, setIsSaving] = useState(false);
    const [feedbackMessage, setFeedbackMessage] = useState<string | null>(null);
    const apiUrl = (window as any).__API_URL__ || 'http://127.0.0.1:6144';
    const token = localStorage.getItem("token");

    const handleChange = (e: ChangeEvent<HTMLInputElement | HTMLTextAreaElement | HTMLSelectElement>) => {
        const { name, value } = e.target;
        setFormData({ ...formData, [name]: value });
        setErrors({ ...errors, [name]: "" });
    };

    const validateForm = () => {
        const newErrors: Record<string, string> = {};
        if (!formData.name.trim()) newErrors.name = "Project Name is required.";
        if (!formData.description.trim()) newErrors.description = "Project Description is required.";
        if (!formData.instructions.trim()) newErrors.instructions = "Instructions are required.";
        setErrors(newErrors);
        return Object.keys(newErrors).length === 0;
    };

    const handleSubmit = async (e: FormEvent) => {
        e.preventDefault();
        if (!validateForm()) return;

        setIsSaving(true);
        setFeedbackMessage(null);

        const payload = {
            name: formData.name,
            description: formData.description,
            instructions: formData.instructions,
            privacy: formData.privacy,
        };

        const url = project ? `${apiUrl}/projects/${project.id}` : `${apiUrl}/projects`;
        const method = project ? "PUT" : "POST";

        try {
            const response = await apiRequest(url, method, payload, token);
            console.log(`Project ${project ? "updated" : "created"} successfully:`, response);
            setFeedbackMessage(`✅ Project ${project ? "updated" : "created"} successfully!`);
            setTimeout(() => {
                onSave();
                onClose();
            }, 1500);
        } catch (error) {
            console.error(`Error ${project ? "updating" : "creating"} project:`, error);
            setFeedbackMessage(`❌ Failed to ${project ? "update" : "create"} project. Please try again.`);
        } finally {
            setIsSaving(false);
        }
    };

    return (
        <div className="fixed inset-0 flex items-center justify-center bg-black bg-opacity-30 backdrop-blur-sm">
            <div className="bg-[#1E1E1E] text-white p-6 rounded-lg shadow-lg w-1/3">
                <div className="flex justify-between items-center border-b border-gray-700 pb-3">
                    <h2 className="text-lg font-semibold">{project ? "Edit Project" : "Create a New Project"}</h2>
                    <button onClick={onClose} aria-label="Close Modal" className="hover:text-white">
                        <XMarkIcon className="w-6 h-6 text-gray-400" />
                    </button>
                </div>

                <form className="mt-4 space-y-4" onSubmit={handleSubmit}>
                    {/* Name */}
                    <div>
                        <label className="block text-sm mb-1">
                            Project Name <span className="text-red-500">*</span>
                        </label>
                        <input
                            type="text"
                            name="name"
                            value={formData.name}
                            onChange={handleChange}
                            className="w-full bg-gray-800 text-white p-2 rounded-md border border-gray-600 focus:ring-1 focus:ring-teal-500"
                            placeholder="Enter project name"
                        />
                        {errors.name && <p className="text-red-500 text-xs mt-1">{errors.name}</p>}
                    </div>

                    {/* Description */}
                    <div>
                        <label className="block text-sm mb-1">
                            Project Description <span className="text-red-500">*</span>
                        </label>
                        <textarea
                            name="description"
                            value={formData.description}
                            onChange={handleChange}
                            className="w-full bg-gray-800 text-white p-2 rounded-md border border-gray-600 focus:ring-1 focus:ring-teal-500"
                            placeholder="Enter project description"
                            rows={3}
                        />
                        {errors.description && <p className="text-red-500 text-xs mt-1">{errors.description}</p>}
                    </div>

                    {/* Instructions */}
                    <div>
                        <label className="block text-sm mb-1">
                            Project Instructions <span className="text-red-500">*</span>
                        </label>
                        <textarea
                            name="instructions"
                            value={formData.instructions}
                            onChange={handleChange}
                            className="w-full bg-gray-800 text-white p-2 rounded-md border border-gray-600 focus:ring-1 focus:ring-teal-500"
                            placeholder="Instructions that Leo should follow for this project."
                            rows={3}
                        />
                        {errors.instructions && <p className="text-red-500 text-xs mt-1">{errors.instructions}</p>}
                    </div>

                    {/* Privacy */}
                    <div>
                        <label className="block text-sm mb-1">Privacy</label>
                        <select
                            name="privacy"
                            value={formData.privacy}
                            onChange={handleChange}
                            className="w-full bg-gray-800 text-white p-2 rounded-md border border-gray-600 focus:ring-1 focus:ring-teal-500"
                        >
                            <option value="public">Public</option>
                            <option value="private">Private</option>
                        </select>
                    </div>

                    {/* Submit */}
                    <div className="flex justify-end mt-4">
                        <button
                            type="submit"
                            className={`px-4 py-2 rounded-lg transition ${
                                isSaving ? "bg-gray-500 cursor-not-allowed" : "bg-teal-600 hover:bg-teal-700"
                            } text-white`}
                            disabled={isSaving}
                        >
                            {isSaving ? "Saving..." : project ? "Update Project" : "Create Project"}
                        </button>
                    </div>

                    {/* Feedback */}
                    {feedbackMessage && (
                        <div className={`text-sm mt-3 text-center ${feedbackMessage.includes("❌") ? "text-red-500" : "text-green-500"}`}>
                            {feedbackMessage}
                        </div>
                    )}
                </form>
            </div>
        </div>
    );
};

export default ProjectModal;