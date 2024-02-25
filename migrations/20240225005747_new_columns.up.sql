-- Add up migration script here
CREATE TABLE IF NOT EXISTS Journal_new (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    day DATE NOT NULL,
    comment TEXT NOT NULL,
    status TEXT NOT NULL CHECK(status IN ('Completo', 'Pendente', 'Congelado')),
    dev_id INTEGER,
    project_id INTEGER,
    FOREIGN KEY (dev_id) REFERENCES Dev(id) ON DELETE CASCADE,
    FOREIGN KEY (project_id) REFERENCES Project(id) ON DELETE CASCADE
);

INSERT INTO Journal_new (id, day, comment, status, dev_id, project_id)
SELECT id, day, comment, 'Pendente', dev_id, project_id FROM Journal;

DROP TABLE Journal;

ALTER TABLE Journal_new RENAME TO Journal;
