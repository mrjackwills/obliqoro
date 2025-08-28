
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

// Work out the correct plural for a given unit
const suffix = (unit: number): string => unit === 1 ? '' : 's';

/*
 * Convert from seconds to H, M, S
 * With an option to hide seconds - used my the session length section
 */
export const secondsToText = (s: number, hide_second: boolean): string => {
	const second = Math.trunc(s % 60);
	const second_string = hide_second ? '' : `, ${zeroPad(second)} seconds`;
	const minute = Math.floor(s / 60 % 60);
	const minute_string = `${zeroPad(minute)} minute${suffix(minute)}`;
	const hour = Math.floor(s / 60 / 60 % 24);
	const hour_string = hour > 0 ? `${zeroPad(hour)} hour, ` : '';
	return `${hour_string}${minute_string}${second_string}`;
};


// Convert from a percentage to a zeroPadded 2.d.p string
export const formatPercentage = (value: number): string => {
	const rounded = value.toFixed(2);
	const parts = rounded.split('.');
	parts[0] = parts[0].padStart(2, '0');
	return `${parts[0]}.${parts[1]}`;
};
