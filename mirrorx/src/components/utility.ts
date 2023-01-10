export function formatDeviceID(deviceID: number): string {
	const deviceIDStr = String(deviceID).padStart(10, '0');
	return `${deviceIDStr.substring(0, 2)}-${deviceIDStr.substring(2, 6)}-${deviceIDStr.substring(6, 10)}`;
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
