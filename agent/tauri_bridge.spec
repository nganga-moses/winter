# -*- mode: python ; coding: utf-8 -*-

block_cipher = None

a = Analysis(
    ['cli/tauri_bridge.py'],        # Entry script
    pathex=['.'],                   # Search root
    binaries=[],
    datas=[],
    hiddenimports=[
        'agents.builder_agent',
        'context.context_manager',
        'context.code_chunk_registry',
        'context.memory_store.memory_sqlite',
        'context.dependency_graph',
        'llm.llm_router',
        'llm.models.prompter'
    ],
    hookspath=[],
    runtime_hooks=[],
    excludes=[],
    win_no_prefer_redirects=False,
    win_private_assemblies=False,
    cipher=block_cipher,
)

pyz = PYZ(a.pure, a.zipped_data, cipher=block_cipher)

exe = EXE(
    pyz,
    a.scripts,
    [],
    exclude_binaries=True,
    name='winter-agent',                     # 🔥 Final binary name
    debug=False,
    bootloader_ignore_signals=False,
    strip=False,
    upx=True,
    console=True                         # ✅ Show stdout (for debugging)
)

coll = COLLECT(
    exe,
    a.binaries,
    a.zipfiles,
    a.datas,
    strip=False,
    upx=True,
    name='winter-agent'
)