from typing import Dict, List


class PromptAssembler:
    def __init__(self, role: str = "builder"):
        self.role = role

    def assemble(self, task: str, context: Dict = None, diff: str = "") -> str:
        context = context or {}

        if self.role == "reviewer":
            return self._assembler_review_prompt(task, diff, context)

        elif self.role == "requirements":
            return self.assemble_requirements_prompt(task)

        elif self.role == "architect":
            return self._assemble_architect_prompt(task, context)

        elif self.role == "scaffolder":
            return self._assemble_scaffolder_prompt(task, context)

        elif self.role == "assistant":
            return self._assemble_assistant_prompt(task, context)

        return self._assemble_build_prompt(task, context)

    def _assemble_build_prompt(self, task: str, context: Dict) -> str:
        return f"""
        ## Task
        {task}
        
        ## Requirements
        {self._section(context['requirements'])}
        
        ## Architecture
        {self._section(context['architecture'])}
        
        ## Coding Rules
        {self._section(context['rules'])}
        
        ## Related Context
        {self._related_section(context.get('related'))} 
        
        ## Relevant Code Chunks
        {self._code_chunks(context['code_chunks'])}
        
        ## Hotspots (Top called files/functions)
        {self._section(context.get("hotspots", []))}
        ___
        
        Please write production-quality code that fulfils the task, adheres to the rules, and fits within the existing architecture.
        
        Respond with **only code** unless instructed otherwise.
        """

    def _assembler_review_prompt(self, task: str, diff: str, context: Dict) -> str:
        return f"""
        ## Task
        {task}
        
        ## Code Diff
        {diff}
        
        ## Context (if available)
        {context}
        
        ## Related Context
        {self._related_section(context.get('related'))} 
        
        ## Hotspots (Top called files/functions)
        {self._section(context.get("hotspots", []))}
        
       ## ✅ Review Checklist
        - Does the code fulfill the task?
        - Is the logic sound?
        - Is it testable?
        - Are there security or performance concerns?
        - Is the code clean and well-structured?
        
        ---
        
        ## 📥 If any issues or improvements are needed:
        List specific actions the BuilderAgent should take to fix or improve the code.
        
        Use this exact format:
        
        - [ ] Fix: explanation of fix (file: path/to/file.py)
        - [ ] Improve: explanation of improvement (file: path/to/file.py)
        ---

        ## 🧑‍⚖️ Final Verdict:
        APPROVE / WARN / REJECT
        """

    def assemble_requirements_prompt(self, task: str) -> str:
        return f"""
        You are an expert software requirements engineer. Your job is to extract structured, unambiguous system requirements from the user's input.

        ## User Input
        {task}
        
        ## Instructions:
        - Identify key user needs and system features.
        - Break them down into bullet points with clear titles and descriptions.
        - Suggest common missing parts if any (e.g. login, admin panel).
        - Use JSON format: a list of objects with `title` and `description`.
        
        ## Output Format:
        [
          {{
            "title": "Feature A",
            "description": "Details about this feature"
          }},
          ...
        ]
        
        Use natural language where helpful. The goal is to **guide and inspire** the user to think more clearly.
        """

    def _assemble_architect_prompt(self, task: str, context: dict) -> str:
        return f"""
        ## Task
        {task}
    
        ## Requirements
        {context.get('requirements', [])}
    
        ---
    
        You are a senior software architect AI.
    
        Design a modular system architecture that fulfills the above requirements. Include:
    
        - Suggested stack (frontend, backend, DB)
        - Module breakdown (by service or feature)
        - Table names or key entities
        - Optional: Suggest APIs or responsibilities for each module
    
        Respond in valid JSON only.
        """

    def _assemble_scaffolder_prompt(self, task: str, context: dict) -> str:
        return f"""## 
        Task
        {task}
    
        ## Architecture
        {context.get("architecture", [])}
    
        Design a file/folder layout for this project using best practices.
    
        Respond in JSON like:
        ```json
        {{
          "src": {{
            "backend": {{
              "routes": ["auth.py", "form.py"],
              "services": ["auth_service.py"]
            }},
            "frontend": {{
              "components": ["LoginForm.jsx"]
            }}
          }}
        }}
        ```
        Keep it clean and simple.
        
        """

    def _assemble_assistant_prompt(self, task: str, context: dict) -> str:
        return f"""
        You are the Engineering Manager AI with access to all project knowledge.

        Uploaded Files
        
        {context.get('uploads', '[None]')}
        
        Recent Conversation
        
        {context.get('chat', '[No chat history]')}
        
        Task
        
        {task}
        
        Answer clearly and use technical best practices. Be helpful, accurate, and concise. You can include code examples, ideas, or links.
        
        Avoid guessing if unsure – say so and suggest how to clarify.
        """

    def _section(self, lines: List[str]) -> str:
        return "\n".join(f"- {line.strip()}" for line in lines) if lines else "[None]"

    def _code_chunks(self, chunks: List[Dict]) -> str:
        out = []
        for chunk in chunks:
            out.append(f"### {chunk['file']}::{chunk['name']} ({chunk['start_line']}-{chunk['end_line']})")
            out.append(f"\"\"\"\n{chunk['docstring'] or '[no docstring]'}\n\"\"")
            out.append("[code omitted]")
        return "\n".join(out)

    def _related_section(self, related: dict) -> str:
        if not related:
            return "[None]"
        out = []
        for key, items in related.items():
            out.append(f"### {key.capitalize()}")
            out.extend(f"- {i}" for i in items)
        return "\n".join(out)

    def _get_role_header(self) -> str:
        if self.role == "builder":
            return "You are an expert software engineer writing new features."
        elif self.role == "reviewer":
            return "You are an expert code reviewer assessing a code change."
        elif self.role == "planner":
            return "You are a technical project manager breaking down tasks."
        elif self.role == "assistant":
            return "You are a general-purpose Engineering Manager AI."
        return "You are an intelligent AI working on software systems."
