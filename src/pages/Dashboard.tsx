import { useEffect, useState } from "react";
import { Link } from "react-router-dom";
import ProjectModal from "../components/ProjectModal";
import { apiRequest } from "../api/api";

interface Project {
    id: string;
    name: string;
    description?: string;
}

const Dashboard = () => {
    const [chatOpen, setChatOpen] = useState(false);
    const [projects, setProjects] = useState<Project[]>([]);
    const [loading, setLoading] = useState(true);
    const [selectedProject, setSelectedProject] = useState<Project | null>(null);
    const [showProjectModal, setShowProjectModal] = useState(false);
    const [error, setError] = useState(null);

    // Fetch projects when the page loads
    useEffect(() => {
        fetchProjects();
    }, []);

    const fetchProjects = async () => {
        setLoading(true);
        setError(null);

        try {
            const token = localStorage.getItem("token"); // Get token for authentication
            const response = await apiRequest(`${import.meta.env.VITE_API_URL}/projects`, "GET", null, token);
            setProjects(response); // Assuming API returns a list of projects
        } catch (err) {
            setError("Failed to load projects. Please try again.");
        } finally {
            setLoading(false);
        }
    };




    const handleAddProject = () => {
        setSelectedProject(null);
        setShowProjectModal(true);
    };

    const handleSaveProject = () => {
        setShowProjectModal(false);

    };

    return (
        <div className="flex h-screen transition-all">
            <div className={`flex-1 transition-all w-1/2 p-4`}>
                <div className="flex flex-col space-y-8">
                    <div className="flex flex-col space-y-6">
                        <h1 className="text-2xl font-semibold text-white pt-5">My Workspace</h1>
                        <div>
                            <h2 className="text-lg font-medium text-white">
                                Get started by building out your work structure
                            </h2>
                            <p className="text-gray-400 text-sm pt-4 ">
                                Start building your engineering workflow by creating projects to organize your shipped
                                software.
                            </p>
                            <div className="flex space-x-2 mt-2">
                                <button onClick={() => setChatOpen(!chatOpen)}
                                        className="px-4 py-2 bg-gray-700 text-white rounded">
                                    Project Assistant
                                </button>
                                <button onClick={handleAddProject} className="px-4 py-2 bg-gray-700 text-white rounded">
                                    Create a Project
                                </button>
                                <button className="px-4 py-2 bg-gray-700 text-white rounded">
                                    Build Knowledgebase
                                </button>
                            </div>
                        </div>

                        <div>

                            <div>
                                <h2 className="text-lg font-medium text-white">My Projects</h2>
                                <p className="text-gray-400 text-sm">
                                    Manage and access your software projects easily. Invite your team or keep projects
                                    private.
                                </p>
                                <div className="grid grid-cols-2 gap-4 mt-4">
                                    {!loading && projects.length === 0 && (
                                        <div className="flex justify-center items-center h-64">
                                            <div
                                                className="bg-gray-900 p-6 rounded-lg border border-gray-700 text-center shadow-lg">
                                                <h2 className="text-white text-lg font-semibold">No Projects
                                                    Available</h2>
                                                <p className="text-gray-400 text-sm mt-2">Create a new project to get
                                                    started.</p>
                                                <button onClick={handleAddProject}
                                                        className="mt-4 px-4 py-2 bg-green-600 hover:bg-green-700 text-white rounded">
                                                    Create a New Project
                                                </button>
                                            </div>
                                        </div>
                                    )}
                                    {projects.map((project) => (
                                        <div key={project.id}
                                             className="bg-gray-800 p-4 rounded border border-gray-700">
                                            <h3 className="text-white font-semibold">{project.name}</h3>
                                            <p className="text-gray-400 text-sm">{project.description || "No description available."}</p>
                                            <Link to={`/projects/${project.id}`} className="mt-2 text-blue-400 text-sm">
                                                Open Project
                                            </Link>
                                        </div>
                                    ))}
                                </div>
                            </div>

                        </div>


                        {showProjectModal && (
                            <ProjectModal
                                onClose={() => setShowProjectModal(false)}
                                onSave={handleSaveProject}
                            />
                        )}
                    </div>
                </div>
            </div>
            <div className={`flex-1 transition-all w-1/2 p-12 pl-4`}>
                <h2 className="text-lg font-medium text-white">Connecting Your Projects to the World</h2>
                <p className="text-gray-400 text-sm">
                    We provide multiple <b>cloud integrations</b> so you can ship your software
                    effortlessly. <br/>

                </p>

                <div className="bg-gray-800 p-4 rounded border border-gray-700 mt-4">
                    <h3 className="text-white font-semibold">Integrations</h3>
                    <p className="text-gray-400 text-sm">Configure integrations at the <b>workspace
                        level</b> or tailor them per project.</p>
                    <div className="flex flex-row space-x-4 mt-2">
                        <div
                            className="flex flex-1 items-center justify-between text-sm bg-gray-900 px-3 py-2 rounded border border-gray-700">
                            <span className="text-gray-400">GitHub</span>
                            <img alt="GitHub" className="h-10" src="/icons8-github-384.png"/>
                        </div>
                        <div
                            className="flex flex-1 items-center justify-between text-sm bg-gray-900 px-3 py-2 rounded border border-gray-700">
                            <span className="text-gray-400">Google Cloud Platform</span>
                            <img alt="GCP" className="h-10" src="/icons8-google-cloud-platform-480.png"/>
                        </div>
                        <div
                            className="flex flex-1 items-center justify-between text-sm bg-gray-900 px-3 py-2 rounded border border-gray-700">
                            <span className="text-gray-400">Amazon Web Services</span>
                            <img alt="AWS" className="h-10" src="/icons8-aws-logo-384.png"/>
                        </div>

                    </div>
                </div>
            </div>
        </div>
    );
};

export default Dashboard;