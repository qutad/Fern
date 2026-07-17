import type { BootstrapData, Settings } from '$lib/types/domain';
import { api } from '$lib/api';
import { todayMonth } from '$lib/utils/format';

class AppState {
	month = $state(todayMonth());
	bootstrap = $state<BootstrapData | null>(null);
	loading = $state(true);
	error = $state('');

	async init() {
		if (this.bootstrap) return;
		this.loading = true;
		try {
			this.bootstrap = await api.getBootstrapData();
			this.applyTheme(this.bootstrap.settings.theme);
		} catch (error) {
			this.error = error instanceof Error ? error.message : 'Could not load Fern';
		} finally {
			this.loading = false;
		}
	}

	async saveSettings(settings: Settings) {
		this.bootstrap = this.bootstrap
			? { ...this.bootstrap, settings: await api.updateSettings(settings) }
			: this.bootstrap;
		this.applyTheme(settings.theme);
	}

	applyTheme(theme: Settings['theme']) {
		if (typeof document === 'undefined') return;
		const dark =
			theme === 'dark' ||
			(theme === 'system' && matchMedia('(prefers-color-scheme: dark)').matches);
		document.documentElement.dataset.theme = dark ? 'dark' : 'light';
	}
}

export const appState = new AppState();
