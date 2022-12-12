CREATE TABLE IF NOT EXISTS error (
	error_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL UNIQUE,
	error_stack TEXT,
	error_message TEXT NOT NULL,
	error_timestamp DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL
	-- error_timestamp DATETIME DEFAULT (datetime('now', 'localtime')) NOT NULL
);

CREATE TABLE IF NOT EXISTS settings (
	settings_id INTEGER PRIMARY KEY AUTOINCREMENT CHECK (settings_id = 1),
	short_break_as_sec INTEGER NOT NULL,
	long_break_as_sec INTEGER NOT NULL,
	session_as_sec INTEGER NOT NULL,
	number_session_before_break INTEGER NOT NULL,
	fullscreen BOOLEAN NOT NULL
);


CREATE TABLE IF NOT EXISTS stats (
	stats_id INTEGER PRIMARY KEY AUTOINCREMENT,
	date DATE NOT NULL,
	number_session_completed INTEGER
);
 
-- sessions today? date, then session_count +=1 on each?
-- CREATE TABLE IF NOT EXISTS settings (
-- 	settings_id INTEGER PRIMARY KEY AUTOINCREMENT CHECK (settings_id = 1),
-- 	short_break_length INTEGER NOT NULL DEFAULT 5,
-- 	long_break_length INTEGER NOT NULL DEFAULT 15,
-- 	session_length INTEGER NOT NULL DEFAULT 25,
-- 	number_session INTEGER NOT NULL DEFAULT 4,
-- ) STRICT;
