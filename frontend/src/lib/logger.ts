const c = {
	reset: '\x1b[0m',
	red: '\x1b[31m',
	green: '\x1b[32m',
	yellow: '\x1b[33m',
	blue: '\x1b[34m',
	dim: '\x1b[2m'
};

/**
 * Logs an outgoing HTTP request with status code and URL.
 * Used in +page.server.ts files to track backend API calls.
 */
export function logRequest(status: number, url: string): void {
	const timestamp = `${c.dim}${new Date().toISOString()}${c.reset}`;
	const statusColor = status >= 400 ? c.red : status >= 300 ? c.yellow : c.green;
	const statusStr = `${statusColor}[${status}]${c.reset}`;
	console.log(`${timestamp} ${statusStr} GET ${url}`);
}

/**
 * Logs an error with optional cause.
 * Strips stack traces – only the message is logged to avoid information disclosure.
 */
export function logError(message: string, error?: unknown): void {
	const timestamp = `${c.dim}${new Date().toISOString()}${c.reset}`;
	let msg = message;
	if (error instanceof Error) {
		msg = `${message}: ${error.message}`;
	} else if (error !== undefined) {
		msg = `${message}: ${error}`;
	}
	console.error(`${timestamp} ${c.red}ERROR${c.reset} ${msg}`);
}
