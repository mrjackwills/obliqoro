BEGIN;

-- Remove checks from settings & add auto_pause column
ALTER TABLE
    settings RENAME TO settings_old;

CREATE TABLE settings (
    settings_id INTEGER PRIMARY KEY AUTOINCREMENT CHECK (settings_id = 1),
    auto_pause BOOLEAN NOT NULL DEFAULT FALSE,
    auto_pause_threshold INTEGER NOT NULL,
    auto_pause_timespan_sec INTEGER NOT NULL,
    auto_resume BOOLEAN NOT NULL DEFAULT FALSE,
    auto_resume_threshold INTEGER NOT NULL,
    auto_resume_timespan_sec INTEGER NOT NULL,
    fullscreen BOOLEAN NOT NULL,
    long_break_as_sec INTEGER NOT NULL,
    number_session_before_break INTEGER NOT NULL,
    session_as_sec INTEGER NOT NULL,
    short_break_as_sec INTEGER NOT NULL
);

INSERT INTO
    settings (
        settings_id,
        auto_pause,
        auto_pause_threshold,
        auto_pause_timespan_sec,
        auto_resume,
        auto_resume_threshold,
        auto_resume_timespan_sec,
        fullscreen,
        long_break_as_sec,
        number_session_before_break,
        session_as_sec,
        short_break_as_sec
    )
SELECT
    settings_id,
    FALSE,
    5, 
    300,
    FALSE,
    5,
    300,
    fullscreen,
    long_break_as_sec,
    number_session_before_break,
    session_as_sec,
    short_break_as_sec
FROM
    settings_old;

-- Check for each column individually
CREATE TEMP TABLE temp_column_check AS
SELECT
    MAX(CASE WHEN name = 'auto_resume' THEN 1 ELSE 0 END) AS has_auto_resume,
    MAX(CASE WHEN name = 'auto_resume_threshold' THEN 1 ELSE 0 END) AS has_auto_resume_threshold,
    MAX(CASE WHEN name = 'auto_resume_timespan_sec' THEN 1 ELSE 0 END) AS has_auto_resume_timespan_sec,
    MAX(CASE WHEN name = 'auto_pause_threshold' THEN 1 ELSE 0 END) AS has_auto_pause_threshold,
    MAX(CASE WHEN name = 'auto_pause_timespan_sec' THEN 1 ELSE 0 END) AS has_auto_pause_timespan_sec
FROM pragma_table_info('settings_old');

-- Update for auto_resume
UPDATE settings
SET auto_resume = (
    SELECT auto_resume FROM settings_old
    WHERE settings_old.settings_id = settings.settings_id
)
WHERE (SELECT has_auto_resume FROM temp_column_check) = 1;

UPDATE settings
SET auto_resume = 0
WHERE (SELECT has_auto_resume FROM temp_column_check) = 0;

-- Update for auto_resume_threshold
UPDATE settings
SET auto_resume_threshold = (
    SELECT auto_resume_threshold FROM settings_old
    WHERE settings_old.settings_id = settings.settings_id
)
WHERE (SELECT has_auto_resume_threshold FROM temp_column_check) = 1;

UPDATE settings
SET auto_resume_threshold = 0
WHERE (SELECT has_auto_resume_threshold FROM temp_column_check) = 0;

-- Update for auto_resume_timespan_sec
UPDATE settings
SET auto_resume_timespan_sec = (
    SELECT auto_resume_timespan_sec FROM settings_old
    WHERE settings_old.settings_id = settings.settings_id
)
WHERE (SELECT has_auto_resume_timespan_sec FROM temp_column_check) = 1;

UPDATE settings
SET auto_resume_timespan_sec = 0
WHERE (SELECT has_auto_resume_timespan_sec FROM temp_column_check) = 0;

-- Update for auto_pause_threshold
UPDATE settings
SET auto_pause_threshold = (
    SELECT auto_pause_threshold FROM settings_old
    WHERE settings_old.settings_id = settings.settings_id
)
WHERE (SELECT has_auto_pause_threshold FROM temp_column_check) = 1;

UPDATE settings
SET auto_pause_threshold = 0
WHERE (SELECT has_auto_pause_threshold FROM temp_column_check) = 0;

-- Update for auto_pause_timespan_sec
UPDATE settings
SET auto_pause_timespan_sec = (
    SELECT auto_pause_timespan_sec FROM settings_old
    WHERE settings_old.settings_id = settings.settings_id
)
WHERE (SELECT has_auto_pause_timespan_sec FROM temp_column_check) = 1;

UPDATE settings
SET auto_pause_timespan_sec = 0
WHERE (SELECT has_auto_pause_timespan_sec FROM temp_column_check) = 0;


DROP TABLE temp_column_check;
DROP TABLE settings_old;

COMMIT;