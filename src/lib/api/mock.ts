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

const month = new Date().toISOString().slice(0, 7);
const day = (offset: number) => {
	const date = new Date();
	date.setDate(date.getDate() + offset);
	return date.toISOString().slice(0, 10);
};

let transactions: Transaction[] = [
	{
		id: 't1',
		date: day(-1),
		description: 'Juniper Market',
		category: 'Groceries',
		account: 'Everyday card',
		type: 'expense',
		amount: 86.42
	},
	{
		id: 't2',
		date: day(-2),
		description: 'Northwind Studio',
		category: 'Salary',
		account: 'Checking',
		type: 'income',
		amount: 4850
	},
	{
		id: 't3',
		date: day(-3),
		description: 'Field Notes Coffee',
		category: 'Dining',
		account: 'Everyday card',
		type: 'expense',
		amount: 12.8
	},
	{
		id: 't4',
		date: day(-5),
		description: 'Cityline Energy',
		category: 'Utilities',
		account: 'Checking',
		type: 'expense',
		amount: 142.18
	},
	{
		id: 't5',
		date: day(-7),
		description: 'Aperture Books',
		category: 'Shopping',
		account: 'Everyday card',
		type: 'expense',
		amount: 38.5
	},
	{
		id: 't6',
		date: day(-9),
		description: 'Savings transfer',
		category: 'Transfer',
		account: 'Checking',
		type: 'transfer',
		amount: 800
	},
	{
		id: 't7',
		date: day(-12),
		description: 'Paper & Grain',
		category: 'Freelance',
		account: 'Checking',
		type: 'income',
		amount: 720
	},
	{
		id: 't8',
		date: day(-14),
		description: 'Metro pass',
		category: 'Transport',
		account: 'Everyday card',
		type: 'expense',
		amount: 78
	}
];

let budgets: Budget[] = [
	{ id: 'b1', category: 'Groceries', limit: 600, spent: 386, month },
	{ id: 'b2', category: 'Dining', limit: 280, spent: 194, month },
	{ id: 'b3', category: 'Transport', limit: 220, spent: 112, month },
	{ id: 'b4', category: 'Shopping', limit: 350, spent: 321, month },
	{ id: 'b5', category: 'Entertainment', limit: 180, spent: 76, month },
	{ id: 'b6', category: 'Utilities', limit: 300, spent: 227, month }
];

let recurring: RecurringPayment[] = [
	{
		id: 'r1',
		name: 'Apartment rent',
		category: 'Housing',
		amount: 1650,
		frequency: 'monthly',
		nextDate: day(3),
		account: 'Checking',
		type: 'expense',
		active: true
	},
	{
		id: 'r2',
		name: 'Atlas Internet',
		category: 'Utilities',
		amount: 64,
		frequency: 'monthly',
		nextDate: day(7),
		account: 'Everyday card',
		type: 'expense',
		active: true
	},
	{
		id: 'r3',
		name: 'Music subscription',
		category: 'Entertainment',
		amount: 12,
		frequency: 'monthly',
		nextDate: day(9),
		account: 'Everyday card',
		type: 'expense',
		active: true
	},
	{
		id: 'r4',
		name: 'Studio retainer',
		category: 'Freelance',
		amount: 900,
		frequency: 'monthly',
		nextDate: day(12),
		account: 'Checking',
		type: 'income',
		active: true
	}
];

let settings: Settings = { currency: 'USD', theme: 'light', weekStartsOn: 'monday' };
const categories = [
	'Groceries',
	'Dining',
	'Housing',
	'Utilities',
	'Transport',
	'Shopping',
	'Entertainment',
	'Health',
	'Salary',
	'Freelance',
	'Transfer',
	'Other'
];
const colors = ['#d96548', '#e2a64f', '#789b7b', '#7d8fbd', '#a17e9f', '#5d8d91'];

function dashboard(): DashboardData {
	const expenses = transactions
		.filter((t) => t.type === 'expense')
		.reduce((sum, t) => sum + t.amount, 0);
	const income = transactions
		.filter((t) => t.type === 'income')
		.reduce((sum, t) => sum + t.amount, 0);
	const grouped = new Map<string, number>();
	transactions
		.filter((t) => t.type === 'expense')
		.forEach((t) => grouped.set(t.category, (grouped.get(t.category) ?? 0) + t.amount));
	return {
		balance: 18420.65,
		income,
		expenses,
		savingsRate: Math.round(((income - expenses) / income) * 100),
		monthlyTrend: ['Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul'].map((label, i) => ({
			month: label,
			income: 4700 + i * 165 + (i % 2) * 350,
			expenses: 3150 + i * 95 + (i % 3) * 210
		})),
		categorySpend: [...grouped.entries()]
			.map(([category, amount], i) => ({ category, amount, color: colors[i % colors.length] }))
			.sort((a, b) => b.amount - a.amount),
		recentTransactions: transactions.slice(0, 5),
		upcoming: recurring.filter((r) => r.active).slice(0, 4)
	};
}

export const mockApi = {
	async getBootstrapData(): Promise<BootstrapData> {
		return { settings, accounts: ['Checking', 'Everyday card', 'Savings'], categories };
	},
	async listTransactions(): Promise<Transaction[]> {
		return structuredClone(transactions);
	},
	async createTransaction(input: Omit<Transaction, 'id'>): Promise<Transaction> {
		const item = { ...input, id: crypto.randomUUID() };
		transactions = [item, ...transactions];
		return item;
	},
	async updateTransaction(input: Transaction): Promise<Transaction> {
		transactions = transactions.map((t) => (t.id === input.id ? input : t));
		return input;
	},
	async deleteTransaction(id: string): Promise<void> {
		transactions = transactions.filter((t) => t.id !== id);
	},
	async listBudgets(requestedMonth: string): Promise<Budget[]> {
		return budgets
			.filter((b) => b.month === requestedMonth || b.month === month)
			.map((b) => ({ ...b, month: requestedMonth }));
	},
	async upsertBudget(input: Budget): Promise<Budget> {
		const item = { ...input, id: input.id || crypto.randomUUID() };
		budgets = [...budgets.filter((b) => b.id !== item.id), item];
		return item;
	},
	async deleteBudget(id: string): Promise<void> {
		budgets = budgets.filter((b) => b.id !== id);
	},
	async listRecurring(): Promise<RecurringPayment[]> {
		return structuredClone(recurring);
	},
	async createRecurring(input: Omit<RecurringPayment, 'id'>): Promise<RecurringPayment> {
		const item = { ...input, id: crypto.randomUUID() };
		recurring = [item, ...recurring];
		return item;
	},
	async updateRecurring(input: RecurringPayment): Promise<RecurringPayment> {
		recurring = recurring.map((r) => (r.id === input.id ? input : r));
		return input;
	},
	async deleteRecurring(id: string): Promise<void> {
		recurring = recurring.filter((r) => r.id !== id);
	},
	async getDashboard(): Promise<DashboardData> {
		return dashboard();
	},
	async getAnalytics(): Promise<AnalyticsData> {
		const base = dashboard();
		return {
			...base,
			dailySpend: Array.from({ length: 14 }, (_, i) => ({
				date: day(i - 13),
				amount: 22 + ((i * 37) % 94)
			})),
			averageDailySpend: 94.2,
			largestExpense:
				transactions.filter((t) => t.type === 'expense').sort((a, b) => b.amount - a.amount)[0] ??
				null
		};
	},
	async previewCsv(filePath: string): Promise<CsvPreview> {
		void filePath;
		return {
			headers: ['Date', 'Description', 'Amount', 'Category'],
			rows: [
				['2026-07-02', 'Example market', '-42.10', 'Groceries'],
				['2026-07-03', 'Client deposit', '800.00', 'Income']
			],
			totalRows: 2
		};
	},
	async importCsv(filePath: string, mapping: CsvMapping): Promise<{ imported: number }> {
		void filePath;
		void mapping;
		return { imported: 2 };
	},
	async exportCsv(filePath: string): Promise<void> {
		void filePath;
	},
	async createBackup(filePath: string): Promise<void> {
		void filePath;
	},
	async restoreBackup(filePath: string): Promise<void> {
		void filePath;
	},
	async updateSettings(input: Settings): Promise<Settings> {
		settings = input;
		return settings;
	}
};
