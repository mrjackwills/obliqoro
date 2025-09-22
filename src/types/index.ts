export type ConstT<T> = T[keyof T];

export const InvokeMessage = {
	GetPackageInfo: 'get_package_info',
	Init: 'init',
	Minimize: 'minimize',
	OpenLocation: 'open_location',
	PauseAfterBreak: 'pause_after_break',
	ResetSettings: 'reset_settings',
	SetSettings: 'set_settings',
	ShowSettings: 'show_settings',
	TogglePause: 'toggle_pause'
} as const;
export type InvokeMessage = ConstT<typeof InvokeMessage>;

export const FrontEndRoutes = {
	Settings: '/',
	Timer: '/timer'
} as const;

export const FrontEndNames = {
	Settings: 'settings',
	Timer: 'timer'
} as const;

export type FrontEndRoutes = ConstT<typeof FrontEndRoutes>;

export const ModuleName = {
	Interval: 'interval',
	NextBreak: 'nextbreak',
	PackageInfo: 'packageinfo',
	Setting: 'setting',
	Snack: 'snack',
	CpuUsage: 'cpu_usage'
} as const;
export type ModuleName = ConstT<typeof ModuleName>;

// / These need to match the enum FrontEnd as_str()
export const ListenMessage = {
	Cpu: 'cpu',
	Error: 'error',
	Fullscreen: 'fullscreen',
	GetSettings: 'get::settings',
	GoToSettings: 'goto::settings',
	GoToTimer: 'goto::timer',
	NextBreak: 'next-break',
	NumberSessionsBeforeLong: 'sessions-before-long',
	OnBreak: 'on-break',
	PackageInfo: 'package-info',
	Paused: 'paused'
} as const;
export type ListenMessage = ConstT<typeof ListenMessage>;

export const BreakTypes = {
	Short: 0,
	Long: 1
} as const;
export type BreakTypes = ConstT<typeof BreakTypes>;

export type TSnack = {
	message?: string;
	icon?: string;
	timeout?: number;
	loading?: boolean;
};

export type ShowTimer = {
	interval: number;
	strategy: string;
};

export type CpuMeasure = {
	current: number;
	pause?: number;
	resume?: number;
};
export type BuildInfo = Record<'homepage' | 'version' | 'build_date', string> & { github_version?: string };
export type FrontEndState = Record<'fullscreen' | 'auto_pause' | 'paused' | 'start_on_boot' | 'auto_resume', boolean> &
  Record<
    'auto_pause_threshold' |
    'auto_pause_timespan_sec' |
    'auto_resume_threshold' |
    'auto_resume_timespan_sec' |
    'long_break_as_sec' |
    'number_session_before_break' |
    'session_as_sec' |
    'short_break_as_sec', number>;

