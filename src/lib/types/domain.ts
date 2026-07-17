export type TransactionType = 'expense' | 'income' | 'transfer';
export type Theme = 'light' | 'dark' | 'system';

export interface Transaction {
	id: string;
	date: string;
	description: string;
	category: string;
	account: string;
	type: TransactionType;
	amount: number;
	note?: string;
}

export interface Budget {
	id: string;
	category: string;
	limit: number;
	spent: number;
	month: string;
}

export interface RecurringPayment {
	id: string;
	name: string;
	category: string;
	amount: number;
	frequency: 'weekly' | 'monthly' | 'quarterly' | 'yearly';
	nextDate: string;
	account: string;
	type: 'expense' | 'income';
	active: boolean;
}

export interface Settings {
	currency: string;
	theme: Theme;
	weekStartsOn: 'monday' | 'sunday';
}

export interface DashboardData {
	balance: number;
	income: number;
	expenses: number;
	savingsRate: number;
	monthlyTrend: { month: string; income: number; expenses: number }[];
	categorySpend: { category: string; amount: number; color: string }[];
	recentTransactions: Transaction[];
	upcoming: RecurringPayment[];
}

export interface AnalyticsData extends DashboardData {
	dailySpend: { date: string; amount: number }[];
	averageDailySpend: number;
	largestExpense: Transaction | null;
}

export interface BootstrapData {
	settings: Settings;
	accounts: string[];
	categories: string[];
}

export interface CsvPreview {
	headers: string[];
	rows: string[][];
	totalRows: number;
}

export interface CsvMapping {
	date: string;
	description: string;
	amount: string;
	category?: string;
	type?: string;
}
