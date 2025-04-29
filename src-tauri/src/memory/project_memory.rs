use std::fs;
use std::path::{Path, PathBuf};
use anyhow::{Result, Context};

pub struct ProjectMemory{
    pub root: PathBuf, // e.g ~/WinterData/projects/{project_id}/
}

impl ProjectMemory{
    /// Read a memory file as string
    pub fn read(&self, relative_path: &str) -> Result<String> {
        let full_path = self.root.join(relative_path);
        fs::read_to_string(&full_path)
            .with_context(|| format!("Failed to read memory file: {}", full_path.display()))
    }
    /// Return a stubbed summary of a memory file
    pub fn summarize(&self,relative_path: &str,_scope: &str)-> Result<String>{
        let full_path = self.root.join(relative_path);
        let content = fs::read_to_string(&full_path)
            .with_context(|| format!("Failed to summarize file: {}",full_path.display()))?;

        // ✨ In future: Use SummarizeTool (LLM). For now, return first N lines.
        let summary: String = content.lines().take(10).collect::<Vec<_>>().join("\n");
        Ok(format!("(Stub Summary)\n{}", summary))
    }

    /// Chunk a memory file into parts by line count(approx)
    pub fn chunk(&self,relative_path: &str, lines_per_chunk: usize) -> Result<Vec<String>> {
        let full_path = self.root.join(relative_path);
        let content = fs::read_to_string(&full_path)
            .with_context(|| format!("Failed to chunk file: {}", full_path.display()))?;

        let lines: Vec<_> = content.lines().collect();
        let mut chunks = vec![];

        for chunk in lines.chunks(lines_per_chunk){
            chunks.push(chunk.join("\n"));
        }
        Ok(chunks)
    }

}