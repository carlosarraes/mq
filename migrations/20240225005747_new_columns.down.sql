CREATE TABLE IF NOT EXISTS Journal_old (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    day DATE NOT NULL,
    title TEXT NOT NULL,
    comment TEXT NOT NULL,
    dev_id INTEGER,
    project_id INTEGER,
    FOREIGN KEY (dev_id) REFERENCES Dev(id) ON DELETE CASCADE,
    FOREIGN KEY (project_id) REFERENCES Project(id) ON DELETE CASCADE
);

INSERT INTO Journal_old (id, day, title, comment, dev_id, project_id)
SELECT id, day, 'N/A' AS title, comment, dev_id, project_id FROM Journal;

DROP TABLE Journal;

ALTER TABLE Journal_old RENAME TO Journal;
