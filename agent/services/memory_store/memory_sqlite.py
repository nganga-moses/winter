import sqlite3
from pathlib import Path
from typing import List, Dict, Optional


class MemorySqlite:
    def __init__(self, db_path: str = "winter_memory.db"):
        self.db_path = Path(db_path)
        self.conn = sqlite3.connect(self.db_path)
        self._create_tables()

    def _create_tables(self):
        with self.conn:
            self.conn.executescript("""
            CREATE TABLE IF NOT EXISTS requirements (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                summary TEXT NOT NULL,
                source_file TEXT,
                confidence REAL DEFAULT 1.0,
                created_at TEXT DEFAULT CURRENT_TIMESTAMP
            );
            CREATE TABLE IF NOT EXISTS architecture(
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                module TEXT NOT NULL,
                layer TEXT,
                dependencies TEXT,
                created_at TEXT DEFAULT CURRENT_TIMESTAMP
            );
            CREATE TABLE IF NOT EXISTS rules(
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                rule_type TEXT NOT NULL,
                value TEXT NOT NULL,
                source TEXT,
                created_at TEXT DEFAULT CURRENT_TIMESTAMP
            );
            """)

    def add_requirement(self, summary: str, source_file: Optional[str] = None, confidence: float = 1.0):
        with self.conn:
            self.conn.execute(
                "INSERT INTO requirements (summary, source_file, confidence) VALUES (?,?,?)",
                (summary, source_file, confidence)
            )

    def get_requirements(self) -> List[Dict]:
        cursor = self.conn.execute("SELECT * FROM requirements ORDER BY created_at DESC")
        return [dict(zip([c[0] for c in cursor.description], row)) for row in cursor.fetchall()]

    def add_architecture(self, module: str, layer: Optional[str], dependencies: Optional[str]):
        with self.conn:
            self.conn.execute(
                "INSERT INTO architecture (module, layer, dependencies) VALUES (?,?,?)",
                (module, layer, dependencies)
            )

    def get_architecture(self) -> List[Dict]:
        cursor = self.conn.execute("SELECT * FROM architecture ORDER BY created_at DESC")
        return [dict(zip([c[0] for c in cursor.description], row)) for row in cursor.fetchall()]

    def add_rule(self, rule_type: str, value: str, source: Optional[str]):
        with self.conn:
            self.conn.execute(
                "INSERT INTO rules (rule_type, value, source) VALUES (?,?,?)",
                (rule_type, value, source)
            )

    def get_rules(self) -> List[Dict]:
        cursor = self.conn.execute("SELECT * FROM rules ORDER BY created_at DESC")
        return [dict(zip([c[0] for c in cursor.description], row)) for row in cursor.fetchall()]

    def close(self):
        self.conn.close()
