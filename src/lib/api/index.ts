import { invoke } from '@tauri-apps/api/core';
import type {
	AnalyticsData,
	BootstrapData,
	Budget,
	CsvMapping,
	CsvPreview,
	DashboardData,
	RecurringPayment,
	Settings,
	Transaction
} from '$lib/types/domain';
import { mockApi } from './mock';

interface RawAccount {
	id: string;
	name: string;
}
interface RawCategory {
	id: string;
	name: string;
	color: string;
}
interface RawTransaction {
	id: string;
	accountId: string;
	categoryId?: string;
	transactionType: Transaction['type'];
	amountCents: number;
	date: string;
	description: string;
	notes: string;
}
interface RawBudget {
	id: string;
	categoryId: string;
	month: string;
	amountCents: number;
	spentCents: number;
}
interface RawRecurring {
	id: string;
	accountId: string;
	categoryId?: string;
	transactionType: 'expense' | 'income';
	amountCents: number;
	description: string;
	frequency: 'daily' | 'weekly' | 'monthly' | 'yearly';
	intervalCount: number;
	startDate: string;
	endDate?: string;
	active: boolean;
}
interface RawCategoryTotal {
	categoryName: string;
	color: string;
	amountCents: number;
}
interface RawPeriod {
	period: string;
	incomeCents: number;
	expenseCents: number;
}
interface RawDashboard {
	incomeCents: number;
	expenseCents: number;
	netCents: number;
	byCategory: RawCategoryTotal[];
	recentTransactions: RawTransaction[];
}
interface RawAnalytics {
	incomeCents: number;
	expenseCents: number;
	netCents: number;
	byCategory: RawCategoryTotal[];
	periods: RawPeriod[];
}
interface RawBootstrap {
	accounts: RawAccount[];
	categories: RawCategory[];
	settings: Record<string, string>;
	dashboard: RawDashboard;
}

let accounts: RawAccount[] = [];
let categories: RawCategory[] = [];

const inTauri = () => typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;
const command = <T>(name: string, args?: Record<string, unknown>) => invoke<T>(name, args);
const cents = (amount: number) => Math.round(amount * 100);
const amount = (value: number) => value / 100;
const accountName = (id: string) =>
	accounts.find((item) => item.id === id)?.name ?? 'Unknown account';
const accountId = (name: string) =>
	accounts.find((item) => item.name === name)?.id ?? accounts[0]?.id ?? 'account-default';
const categoryName = (id?: string) => categories.find((item) => item.id === id)?.name ?? 'Other';
const categoryId = (name: string) => categories.find((item) => item.name === name)?.id;

function transactionFromRaw(item: RawTransaction): Transaction {
	return {
		id: item.id,
		date: item.date,
		description: item.description,
		category: categoryName(item.categoryId),
		account: accountName(item.accountId),
		type: item.transactionType,
		amount: amount(item.amountCents),
		note: item.notes
	};
}

function transactionInput(item: Omit<Transaction, 'id'>) {
	return {
		accountId: accountId(item.account),
		categoryId: item.type === 'transfer' ? null : (categoryId(item.category) ?? null),
		transactionType: item.type,
		amountCents: cents(item.amount),
		date: item.date,
		description: item.description,
		notes: item.note ?? ''
	};
}

function nextOccurrence(item: RawRecurring) {
	const today = new Date();
	today.setUTCHours(0, 0, 0, 0);
	let date = new Date(`${item.startDate}T00:00:00Z`);
	const originalDay = date.getUTCDate();
	while (date < today) {
		if (item.frequency === 'daily') date.setUTCDate(date.getUTCDate() + item.intervalCount);
		else if (item.frequency === 'weekly')
			date.setUTCDate(date.getUTCDate() + item.intervalCount * 7);
		else {
			const months = item.frequency === 'yearly' ? item.intervalCount * 12 : item.intervalCount;
			const target = new Date(Date.UTC(date.getUTCFullYear(), date.getUTCMonth() + months, 1));
			const lastDay = new Date(
				Date.UTC(target.getUTCFullYear(), target.getUTCMonth() + 1, 0)
			).getUTCDate();
			date = new Date(
				Date.UTC(target.getUTCFullYear(), target.getUTCMonth(), Math.min(originalDay, lastDay))
			);
		}
	}
	return date.toISOString().slice(0, 10);
}

function recurringFromRaw(item: RawRecurring): RecurringPayment {
	return {
		id: item.id,
		name: item.description,
		category: categoryName(item.categoryId),
		amount: amount(item.amountCents),
		frequency:
			item.frequency === 'monthly' && item.intervalCount === 3
				? 'quarterly'
				: (item.frequency as RecurringPayment['frequency']),
		nextDate: nextOccurrence(item),
		account: accountName(item.accountId),
		type: item.transactionType,
		active: item.active
	};
}

function recurringInput(item: Omit<RecurringPayment, 'id'>) {
	return {
		accountId: accountId(item.account),
		categoryId: categoryId(item.category) ?? null,
		transactionType: item.type,
		amountCents: cents(item.amount),
		description: item.name,
		notes: '',
		frequency: item.frequency === 'quarterly' ? 'monthly' : item.frequency,
		intervalCount: item.frequency === 'quarterly' ? 3 : 1,
		startDate: item.nextDate,
		endDate: null,
		active: item.active
	};
}

function settingsFromRaw(values: Record<string, string>): Settings {
	return {
		currency: values.currency ?? 'USD',
		theme: (values.theme as Settings['theme']) ?? 'system',
		weekStartsOn:
			((values.weekStartsOn ?? values.weekStart) as Settings['weekStartsOn']) ?? 'monday'
	};
}

function monthBounds(month: string) {
	const [year, number] = month.split('-').map(Number);
	const startDate = `${month}-01`;
	const endDate = new Date(Date.UTC(year, number, 0)).toISOString().slice(0, 10);
	return { startDate, endDate };
}

function trendBounds(month: string) {
	const [year, number] = month.split('-').map(Number);
	const start = new Date(Date.UTC(year, number - 6, 1));
	return { startDate: start.toISOString().slice(0, 10), endDate: monthBounds(month).endDate };
}

function fillMonthlyPeriods(selectedMonth: string, periods: RawPeriod[]): RawPeriod[] {
	const [year, month] = selectedMonth.split('-').map(Number);
	const periodsByMonth = new Map(periods.map((item) => [item.period, item]));

	return Array.from({ length: 6 }, (_, index) => {
		const date = new Date(Date.UTC(year, month - 1 - (5 - index), 1));
		const key = date.toISOString().slice(0, 7);
		const existing = periodsByMonth.get(key);

		return {
			period: date.toLocaleString('en', {
				month: 'short',
				timeZone: 'UTC'
			}),
			incomeCents: existing?.incomeCents ?? 0,
			expenseCents: existing?.expenseCents ?? 0
		};
	});
}

function dashboardFromRaw(
	summary: RawDashboard,
	trend: RawPeriod[],
	recurring: RawRecurring[],
	selectedMonth: string,
	balanceCents = summary.netCents
): DashboardData {
	const income = amount(summary.incomeCents);
	const expenses = amount(summary.expenseCents);
	return {
		balance: amount(balanceCents),
		income,
		expenses,
		savingsRate: income ? Math.round(((income - expenses) / income) * 100) : 0,
		monthlyTrend: fillMonthlyPeriods(selectedMonth, trend).map((item) => ({
			month: item.period,
			income: amount(item.incomeCents),
			expenses: amount(item.expenseCents)
		})),
		categorySpend: summary.byCategory.map((item) => ({
			category: item.categoryName,
			amount: amount(item.amountCents),
			color: item.color
		})),
		recentTransactions: summary.recentTransactions.map(transactionFromRaw),
		upcoming: recurring
			.filter((item) => item.active)
			.map(recurringFromRaw)
			.slice(0, 4)
	};
}

export const api = {
	async getBootstrapData(): Promise<BootstrapData> {
		if (!inTauri()) return mockApi.getBootstrapData();
		const raw = await command<RawBootstrap>('get_bootstrap_data');
		accounts = raw.accounts;
		categories = raw.categories;
		return {
			settings: settingsFromRaw(raw.settings),
			accounts: accounts.map((item) => item.name),
			categories: categories.map((item) => item.name)
		};
	},
	async listTransactions(month?: string): Promise<Transaction[]> {
		if (!inTauri()) return mockApi.listTransactions();
		const filter = month ? { ...monthBounds(month), limit: 500 } : { limit: 500 };
		return (await command<RawTransaction[]>('list_transactions', { filter })).map(
			transactionFromRaw
		);
	},
	async createTransaction(item: Omit<Transaction, 'id'>): Promise<Transaction> {
		if (!inTauri()) return mockApi.createTransaction(item);
		return transactionFromRaw(
			await command('create_transaction', { input: transactionInput(item) })
		);
	},
	async updateTransaction(item: Transaction): Promise<Transaction> {
		if (!inTauri()) return mockApi.updateTransaction(item);
		return transactionFromRaw(
			await command('update_transaction', { input: { id: item.id, ...transactionInput(item) } })
		);
	},
	async deleteTransaction(id: string): Promise<void> {
		if (!inTauri()) return mockApi.deleteTransaction(id);
		await command('delete_transaction', { input: { id } });
	},
	async listBudgets(month: string): Promise<Budget[]> {
		if (!inTauri()) return mockApi.listBudgets(month);
		return (await command<RawBudget[]>('list_budgets', { input: { month } })).map((item) => ({
			id: item.id,
			category: categoryName(item.categoryId),
			limit: amount(item.amountCents),
			spent: amount(item.spentCents),
			month: item.month
		}));
	},
	async upsertBudget(item: Budget): Promise<Budget> {
		if (!inTauri()) return mockApi.upsertBudget(item);
		const raw = await command<RawBudget>('upsert_budget', {
			input: {
				categoryId: categoryId(item.category),
				month: item.month,
				amountCents: cents(item.limit)
			}
		});
		return {
			id: raw.id,
			category: categoryName(raw.categoryId),
			limit: amount(raw.amountCents),
			spent: amount(raw.spentCents),
			month: raw.month
		};
	},
	async deleteBudget(id: string): Promise<void> {
		if (!inTauri()) return mockApi.deleteBudget(id);
		await command('delete_budget', { input: { id } });
	},
	async listRecurring(): Promise<RecurringPayment[]> {
		if (!inTauri()) return mockApi.listRecurring();
		return (await command<RawRecurring[]>('list_recurring_rules')).map(recurringFromRaw);
	},
	async createRecurring(item: Omit<RecurringPayment, 'id'>): Promise<RecurringPayment> {
		if (!inTauri()) return mockApi.createRecurring(item);
		return recurringFromRaw(
			await command('create_recurring_rule', { input: recurringInput(item) })
		);
	},
	async updateRecurring(item: RecurringPayment): Promise<RecurringPayment> {
		if (!inTauri()) return mockApi.updateRecurring(item);
		return recurringFromRaw(
			await command('update_recurring_rule', { input: { id: item.id, ...recurringInput(item) } })
		);
	},
	async deleteRecurring(id: string): Promise<void> {
		if (!inTauri()) return mockApi.deleteRecurring(id);
		await command('delete_recurring_rule', { input: { id } });
	},
	async getDashboard(month: string): Promise<DashboardData> {
		if (!inTauri()) return mockApi.getDashboard();
		const [summary, analytics, recurring, allTransactions] = await Promise.all([
			command<RawDashboard>('get_dashboard_summary', { input: { month } }),
			command<RawAnalytics>('get_analytics_summary', {
				input: { ...trendBounds(month), groupBy: 'month' }
			}),
			command<RawRecurring[]>('list_recurring_rules'),
			command<RawTransaction[]>('list_transactions', { filter: { limit: 5000 } })
		]);
		const balance = allTransactions.reduce(
			(total, item) =>
				total +
				(item.transactionType === 'income'
					? item.amountCents
					: item.transactionType === 'expense'
						? -item.amountCents
						: 0),
			0
		);
		return dashboardFromRaw(summary, analytics.periods, recurring, month, balance);
	},
	async getAnalytics(month: string): Promise<AnalyticsData> {
		if (!inTauri()) return mockApi.getAnalytics();
		const bounds = monthBounds(month);
		const [summary, raw, daily, recurring, transactions] = await Promise.all([
			command<RawDashboard>('get_dashboard_summary', { input: { month } }),
			command<RawAnalytics>('get_analytics_summary', {
				input: { ...trendBounds(month), groupBy: 'month' }
			}),
			command<RawAnalytics>('get_analytics_summary', { input: { ...bounds, groupBy: 'day' } }),
			command<RawRecurring[]>('list_recurring_rules'),
			command<RawTransaction[]>('list_transactions', {
				filter: { ...bounds, transactionType: 'expense', limit: 500 }
			})
		]);
		const base = dashboardFromRaw(summary, raw.periods, recurring, month);
		const expenses = transactions.map(transactionFromRaw).sort((a, b) => b.amount - a.amount);
		const days = new Date(`${bounds.endDate}T00:00:00Z`).getUTCDate();
		return {
			...base,
			dailySpend: daily.periods.map((item) => ({
				date: item.period,
				amount: amount(item.expenseCents)
			})),
			averageDailySpend: amount(daily.expenseCents) / days,
			largestExpense: expenses[0] ?? null
		};
	},
	async previewCsv(path: string): Promise<CsvPreview> {
		if (!inTauri()) return mockApi.previewCsv(path);
		const raw = await command<{
			headers: string[];
			rows: Record<string, string>[];
			totalRows: number;
		}>('preview_csv', { input: { path, limit: 20 } });
		return { ...raw, rows: raw.rows.map((row) => raw.headers.map((header) => row[header] ?? '')) };
	},
	async importCsv(path: string, mapping: CsvMapping): Promise<{ imported: number }> {
		if (!inTauri()) return mockApi.importCsv(path, mapping);
		return command('import_csv', {
			input: {
				path,
				mapping: { ...mapping, transactionType: mapping.type },
				accountId: accounts[0]?.id ?? 'account-default',
				defaultCategoryId: null,
				defaultTransactionType: 'expense',
				dateFormat: '%Y-%m-%d',
				invertAmount: false
			}
		});
	},
	async exportCsv(path: string): Promise<void> {
		if (!inTauri()) return mockApi.exportCsv(path);
		await command('export_csv', { input: { path, filter: null } });
	},
	async createBackup(path: string): Promise<void> {
		if (!inTauri()) return mockApi.createBackup(path);
		await command('create_backup', { input: { path } });
	},
	async restoreBackup(path: string): Promise<void> {
		if (!inTauri()) return mockApi.restoreBackup(path);
		await command('restore_backup', { input: { path } });
	},
	async updateSettings(settings: Settings): Promise<Settings> {
		if (!inTauri()) return mockApi.updateSettings(settings);
		const raw = await command<Record<string, string>>('update_settings', {
			input: { values: settings }
		});
		return settingsFromRaw(raw);
	}
};
