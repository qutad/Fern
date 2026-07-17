const currencyNames: Record<string, string> = {
	USD: '$',
	EUR: '€',
	GBP: '£',
	CAD: 'CA$',
	AUD: 'A$'
};

export function formatMoney(value: number, currency = 'USD', compact = false): string {
	return new Intl.NumberFormat('en-US', {
		style: 'currency',
		currency,
		notation: compact ? 'compact' : 'standard',
		maximumFractionDigits: compact ? 1 : 2
	}).format(value);
}

export function currencySymbol(currency: string): string {
	return currencyNames[currency] ?? currency;
}

export function formatDate(value: string, options: Intl.DateTimeFormatOptions = {}): string {
	const date = new Date(`${value}T12:00:00`);
	return new Intl.DateTimeFormat('en-US', { month: 'short', day: 'numeric', ...options }).format(
		date
	);
}

export function monthLabel(value: string): string {
	const [year, month] = value.split('-').map(Number);
	return new Intl.DateTimeFormat('en-US', { month: 'long', year: 'numeric' }).format(
		new Date(year, month - 1, 1)
	);
}

export function shiftMonth(value: string, amount: number): string {
	const [year, month] = value.split('-').map(Number);
	const date = new Date(year, month - 1 + amount, 1);
	return `${date.getFullYear()}-${String(date.getMonth() + 1).padStart(2, '0')}`;
}

export function todayMonth(): string {
	const now = new Date();
	return `${now.getFullYear()}-${String(now.getMonth() + 1).padStart(2, '0')}`;
}

export function percent(value: number, total: number): number {
	return total <= 0 ? 0 : Math.round((value / total) * 100);
}
