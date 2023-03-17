export function formatDeviceID(deviceID: number): string {
	const deviceIDStr = String(deviceID).padStart(10, '0');
	return `${deviceIDStr.substring(0, 2)}-${deviceIDStr.substring(2, 6)}-${deviceIDStr.substring(
		6,
		10
	)}`;
}

export function formatFileSize(size: number): string {
	const num = 1024.0; //byte

	if (size < num) return size + ' B';
	if (size < Math.pow(num, 2)) return (size / num).toFixed(2) + ' KB';
	if (size < Math.pow(num, 3)) return (size / Math.pow(num, 2)).toFixed(2) + ' MB';
	if (size < Math.pow(num, 4)) return (size / Math.pow(num, 3)).toFixed(2) + ' GB';
	return (size / Math.pow(num, 4)).toFixed(2) + ' TB';
}

export function formatTransferSpeed(size: number): string {
	const num = 1024.0; //byte

	if (size < num) return size + ' b/s';
	if (size < Math.pow(num, 2)) return (size / num).toFixed(2) + ' Kb/s';
	if (size < Math.pow(num, 3)) return (size / Math.pow(num, 2)).toFixed(2) + ' Mb/s';
	if (size < Math.pow(num, 4)) return (size / Math.pow(num, 3)).toFixed(2) + ' Gb/s';
	return (size / Math.pow(num, 4)).toFixed(2) + ' Tb/s';
}

export function formatSecondsDuration(total_seconds: number): string {
	console.log('total seconds: ' + total_seconds);
	const hours = Math.floor(total_seconds / 3600);
	let minutes = Math.floor((total_seconds - hours * 3600) / 60);
	let seconds = total_seconds - hours * 3600 - hours * 60;

	if (minutes < 0) minutes = 0;
	if (seconds < 0) seconds = 0;

	return `${hours.toString().padStart(2, '0')}:${minutes.toString().padStart(2, '0')}:${seconds
		.toString()
		.padStart(2, '0')}`;
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export function deepCopy(target: any, hash = new WeakMap()) {
	if (typeof target !== 'object' || target === null) {
		throw new TypeError('data is not an object');
	}

	if (hash.has(target)) {
		return hash.get(target);
	}

	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	const newTarget: any = {};
	const dataKeys = Object.keys(target);
	dataKeys.forEach((value) => {
		const currentDataValue = target[value];
		if (typeof currentDataValue !== 'object' || currentDataValue === null) {
			newTarget[value] = currentDataValue;
		} else if (Array.isArray(currentDataValue)) {
			newTarget[value] = [...currentDataValue];
		} else if (currentDataValue instanceof Set) {
			newTarget[value] = new Set([...currentDataValue]);
		} else if (currentDataValue instanceof Map) {
			newTarget[value] = new Map([...currentDataValue]);
		} else {
			hash.set(target, target);

			newTarget[value] = deepCopy(currentDataValue, hash);
		}
	});
	return newTarget;
}
