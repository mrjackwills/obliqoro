CREATE TABLE IF NOT EXISTS error (
	error_id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL UNIQUE,
	error_stack TEXT,
	error_message TEXT NOT NULL,
	error_timestamp DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL
	-- error_timestamp DATETIME DEFAULT (datetime('now', 'localtime')) NOT NULL
);

CREATE TABLE IF NOT EXISTS settings (
	settings_id INTEGER PRIMARY KEY AUTOINCREMENT CHECK (settings_id = 1),
	short_break_as_sec INTEGER CHECK(short_break_as_sec >= 10 AND short_break_as_sec <= 120) NOT NULL,
	long_break_as_sec INTEGER CHECK(long_break_as_sec>= 60 AND long_break_as_sec <= 600) NOT NULL,
	session_as_sec INTEGER CHECK(session_as_sec >= 60 AND session_as_sec <= 3540) NOT NULL,
	number_session_before_break INTEGER CHECK(number_session_before_break >= 2 AND number_session_before_break <= 10) NOT NULL,
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
