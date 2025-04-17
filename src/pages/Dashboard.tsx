import { useEffect, useState } from "react";
import { Link, useNavigate } from "react-router-dom";
import ProjectModal from "../components/ProjectModal";
import { internalRequest } from "../api/api";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import { useAuthStore } from '../hooks/useAuthStore';
import { useInjectedPort } from "../hooks/useInjectedPort";

interface Project {
    id: string;
    name: string;
    description?: string;
}

const Dashboard = () => {
    const [projects, setProjects] = useState<Project[]>([]);
    const [loading, setLoading] = useState(true);
    const [showProjectModal, setShowProjectModal] = useState(false);
    const [error, setError] = useState<string | null>(null);
    const navigate = useNavigate();
    const portReady = useInjectedPort();
    const { supabaseId } = useAuthStore();
    const GITHUB_CLIENT_ID = import.meta.env.VITE_GITHUB_URL;
    const apiUrl = import.meta.env.VITE_API_URL;

    useEffect(() => {
        if (portReady) fetchProjects();
    }, [portReady]);

    useEffect(() => {
        const unlisten = Promise.all([
            listen("menu-new-project", () => handleAddProject()),
            listen("menu-import-sources", () => alert("Import from sources coming soon.")),
            listen("menu-import-git", () => alert("Import from Git coming soon."))
        ]);

        return () => {
            unlisten.then(listeners => listeners.forEach(unsub => unsub()));
        };
    }, []);

    const fetchProjects = async () => {
        setLoading(true);
        try {
            const token = localStorage.getItem("token");
            const response = await internalRequest<Project[]>(`projects`, "GET", null, token);
            setProjects(response);
        } catch (err) {
            setError("Failed to load projects. Please try again.");
        } finally {
            setLoading(false);
        }
    };

    const handleGitHubConnect = () => {
        if (!supabaseId) return;
        const authUrl = `https://github.com/login/oauth/authorize?client_id=${GITHUB_CLIENT_ID}&redirect_uri=${apiUrl}?supabaseId=${supabaseId}&scope=repo%20user`;
        window.open(authUrl, '_blank');
    };

    const handleAddProject = () => {
        setShowProjectModal(true);
    };

    const handleOpenProject = async (projectId: string) => {
        await invoke("set_last_opened_project", { projectPath: projectId });
        navigate(`/projects/${projectId}`);
    };

    return (
        <div className="flex transition-all">
            <div className="flex-1 w-1/2 p-4">
                <div className="space-y-6 pt-5">
                    <h1 className="text-2xl font-semibold text-white">My Workspace</h1>
                    <p className="text-gray-400 text-sm">
                        Start building your engineering workflow by creating projects to organize your shipped software.
                    </p>
                    <div className="flex space-x-2">
                        <button onClick={handleAddProject} className="px-4 py-2 bg-gray-700 text-white rounded">
                            Create a new Project
                        </button>
                        <button className="px-4 py-2 bg-gray-700 text-white rounded">
                            Import an Existing Project
                        </button>
                    </div>

                    <div>
                        <h2 className="text-lg font-medium text-white">My Projects</h2>
                        <p className="text-gray-400 text-sm">Manage and access your software projects easily.</p>
                        <div className="grid grid-cols-2 gap-4 mt-2">
                            {!loading && projects.length === 0 ? (
                                <div className="flex justify-center items-center h-64">
                                    <div
                                        className="bg-blue-gray-400 p-6 rounded-lg border border-gray-900 text-center shadow-lg">
                                        <h2 className="text-white text-lg font-semibold">No Projects Available</h2>
                                        <p className="text-gray-400 text-sm mt-2">Create or import a project to get
                                            started.</p>

                                    </div>
                                </div>                            ) : (
                                projects.map(project => (
                                    <div key={project.id} className="bg-gray-800 p-4 rounded border border-gray-700">
                                        <h3 className="text-white font-semibold">{project.name}</h3>
                                        <p className="text-gray-400 text-sm">{project.description || "No description available."}</p>
                                        <button
                                            onClick={() => handleOpenProject(project.id)}
                                            className="mt-2 text-blue-400 text-sm underline"
                                        >
                                            Open Project
                                        </button>
                                    </div>
                                ))
                            )}
                        </div>
                    </div>

                    {showProjectModal && (
                        <ProjectModal
                            onClose={() => setShowProjectModal(false)}
                            onSave={() => setShowProjectModal(false)}
                        />
                    )}
                </div>
            </div>

            <div className="flex-1 w-1/2 p-12 pl-4">
                <h2 className="text-lg font-medium text-white">Cloud Integrations</h2>
                <p className="text-gray-400 text-sm">Link your stack to deploy effortlessly.</p>

                <div className="space-y-4 mt-4">
                    {[
                        { name: "Supabase", logo: "/icons8-supabase.png", desc: "Supabase Auth & Postgres DB" },
                        { name: "GitHub", logo: "/icons8-github-384.png", desc: "Push code & manage repos" },
                        { name: "GCP", logo: "/icons8-google-cloud-platform-480.png", desc: "Deploy to Google Cloud" },
                        { name: "AWS", logo: "/icons8-aws-logo-384.png", desc: "Deploy to AWS" }
                    ].map(({ name, logo, desc }) => (
                        <div key={name} className="bg-gray-900 p-3 rounded border border-gray-700 flex items-center space-x-4">
                            <img alt={name} src={logo} className="w-10 h-10" />
                            <div className="flex-1 text-gray-400 text-sm">{desc}</div>
                            <button onClick={handleGitHubConnect} className="px-4 py-1 bg-gray-700 text-white rounded">Connect</button>
                        </div>
                    ))}
                </div>
            </div>
        </div>
    );
};

export default Dashboard;