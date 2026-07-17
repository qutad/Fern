import { describe, expect, it } from 'vitest';
import { formatMoney, percent, shiftMonth } from './format';

describe('financial format utilities', () => {
	it('formats currency with cents', () => {
		expect(formatMoney(1234.5, 'USD')).toBe('$1,234.50');
	});

	it('handles month boundaries', () => {
		expect(shiftMonth('2026-01', -1)).toBe('2025-12');
		expect(shiftMonth('2026-12', 1)).toBe('2027-01');
	});

	it('guards percentage calculations against zero totals', () => {
		expect(percent(12, 0)).toBe(0);
		expect(percent(30, 120)).toBe(25);
	});
});
