import {invoke} from "@tauri-apps/api/core";

const PROJECT_ROOT ="Users/maxx/winter/projects/project-001"; //will be replaced with dynamic path

export const listFiles = async (): Promise<string[]> =>{
    return await invoke("list_project_files", { projectPath: PROJECT_ROOT});
};

export const readFile = async (relativePath: string): Promise<string> => {
    return await invoke("read_file",{
        projectPath: PROJECT_ROOT,
        relativePath,
    });
};

export const writeFile = async (relativePath: string, content: string) : Promise<void>=>{
    await invoke("write_file",{
        projectPath: PROJECT_ROOT,
        relativePath,
        content,
    });
};