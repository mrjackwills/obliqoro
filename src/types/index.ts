export enum InvokeMessage {
	ShowSettings = 'show_settings',
	Minimize = 'minimize',
	Init = 'init',
	ResetSettings = 'reset_settings',
	TogglePause = 'toggle_pause',
	SetSettingFullscreen = 'set_setting_fullscreen',
	SetSettingLongBreak = 'set_setting_longbreak',
	SetSettingNumberSession = 'set_setting_number_sessions',
	SetSettingShortBreak = 'set_setting_shortbreak',
	SetSettingSession = 'set_setting_session',
	GetPackageInfo = 'get_package_info',
	GetAutoStart = 'get_autostart',
	SetAutoStart = 'set_autostart',
}

export enum FrontEndRoutes {
	Settings = '/',
	Timer = '/timer'
}

export enum ModuleName {
	Interval = 'interval',
	NextBreak = 'nextbreak',
	PackageInfo = 'packageinfo',
	Setting = 'setting',
	Snack = 'snack',
}

export enum ListenMessage {
	Autostart = 'autostart',
	Error = 'error',
	GetSettings = 'get::settings',
	GoToSettings = 'goto::settings',
	GoToTimer = 'goto::timer',
	NextBreak = 'next-break',
	NumberSessionsBeforeLong = 'sessions-before-long',
	OnBreak = 'on-break',
	PackageInfo = 'package-info',
	Paused = 'paused'
}

export enum BreakTypes {
	Short,
	Long
}

export type TSnack = {
	message?: string;
	icon?: string;
	timeout?: number;
	loading?: boolean;
};

export type ShowTimer = { interval: number, strategy: string }
export type PackageInfo = { [k in 'homepage' | 'version' | 'build_date']: string }
export type BreakSettings = { fullscreen: boolean } & { [k in 'session_as_sec' | 'short_break_as_sec' | 'long_break_as_sec' | 'number_session_before_break']: number }