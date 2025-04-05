import {BrowserRouter as Router, Route, Routes} from "react-router-dom";
import ShellLayout from "./layouts/ShellLayout.tsx";
import CodeEditor from "./pages/CodeEditor.tsx";
import Dashboard from "./pages/Dashboard.tsx";
import Assistant from "./pages/Assistant.tsx";

function App() {
    return (
        <Router>
            <Routes>
                <Route element={<ShellLayout/>}>
                    <Route path="/" element={<Dashboard/>}/>
                    <Route path="/assistant" element={<Assistant/>}/>
                    <Route path="/projects/:id/editor" element={<CodeEditor/>}/>

                </Route>
            </Routes>
            
        </Router>
    );
}

export default App;