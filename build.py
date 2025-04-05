import shutil
import os
import platform

SOURCE = "agent/dist/winter-agent/winter-agent"
if platform.system() == "Windows":
    SOURCE += ".exe"

TARGET = "src-tauri/bundled/winter-agent"

# Ensure target folder exists
os.makedirs(os.path.dirname(TARGET), exist_ok=True)

# Copy binary
shutil.copy(SOURCE, TARGET)
print(f"✅ Copied {SOURCE} → {TARGET}")