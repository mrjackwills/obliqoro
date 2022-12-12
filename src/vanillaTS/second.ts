
export const zeroPad = (unit: number): string => String(unit).padStart(2, '0');

export const sec_to_minutes = (s: number): string => {
	const second = Math.trunc(s % 60);
	const minute = Math.floor(s / 60 % 60);
	if (minute > 0) {
		return `${zeroPad(minute)} minute${minute > 1 ? 's' : ''}, ${zeroPad(second)} seconds`;
	} else {
		return `${zeroPad(second)} second${second > 1 ? 's' : ''}`;
	}
};

export const sec_to_minutes_only = (s: number): string => {
	const minute = Math.floor(s / 60 % 60);
	if (minute > 0) {
		return `${zeroPad(minute)} minute${minute > 1 ? 's' : ''}`;
	} else {
		return `0`;
	}
};