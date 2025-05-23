import { defineStore } from "pinia";

// Enhanced hotkey type with better key combination support
export interface Hotkey {
	id: string;
	description: string;
	key: string;
	ctrl?: boolean;
	shift?: boolean;
	alt?: boolean;
	meta?: boolean;
	callback: () => void;
}

// Default hotkeys configuration
const DEFAULT_HOTKEYS: Omit<Hotkey, "callback">[] = [
	{
		id: "open_settings",
		description: "Open settings",
		key: ".",
		ctrl: false, // NOTE: Maybe want another one for "ctrl + ."?
		shift: false,
		alt: false,
		meta: true,
	},
	{
		id: "next_move",
		description: "Go to next move",
		key: "ArrowRight",
		ctrl: false,
		shift: false,
		alt: false,
		meta: false,
	},
	{
		id: "prev_move",
		description: "Go to previous move",
		key: "ArrowLeft",
		ctrl: false,
		shift: false,
		alt: false,
		meta: false,
	},
	{
		id: "toggle_game_library",
		description: "Toggle game library",
		key: "g",
		ctrl: false,
		shift: false,
		alt: false,
		meta: true,
	},
];

export const useSettingsStore = defineStore("settings", {
	state: () => ({
		hotkeys: [] as Hotkey[],
		defaultCallbacks: {} as Record<string, () => void>,
	}),

	actions: {
		initializeHotkeys(defaultCallbacks: Record<string, () => void>) {
			// Load saved hotkeys from localStorage or use defaults
			const savedHotkeys = localStorage.getItem("hotkeys");
			const baseHotkeys = savedHotkeys
				? JSON.parse(savedHotkeys)
				: DEFAULT_HOTKEYS;

			// Merge saved configuration with callbacks
			this.hotkeys = baseHotkeys.map((hotkey: Omit<Hotkey, "callback">) => ({
				...hotkey,
				callback: defaultCallbacks[hotkey.id],
			}));

			// Setup global event listener
			document.removeEventListener("keydown", this.handleKeyPress);
			document.addEventListener("keydown", this.handleKeyPress);
		},

		handleKeyPress(event: KeyboardEvent) {
			// Ignore if typing in an input
			if (
				event.target instanceof HTMLInputElement ||
				event.target instanceof HTMLTextAreaElement
			) {
				return;
			}

			const matchingHotkey = this.hotkeys.find(
				(hotkey) =>
					hotkey.key === event.key &&
					!!hotkey.ctrl === event.ctrlKey &&
					!!hotkey.shift === event.shiftKey &&
					!!hotkey.alt === event.altKey &&
					!!hotkey.meta === event.metaKey,
			);

			if (matchingHotkey) {
				event.preventDefault();
				matchingHotkey.callback();
			}
		},

		updateHotkey(
			id: string,
			updates: Partial<Omit<Hotkey, "id" | "callback">>,
		) {
			const index = this.hotkeys.findIndex((h) => h.id === id);
			if (index !== -1) {
				this.hotkeys[index] = { ...this.hotkeys[index], ...updates };
				this.saveHotkeys();
			}
		},

		saveHotkeys() {
			// Save hotkey configuration without the callbacks
			const hotkeyConfig = this.hotkeys.map(
				({ callback, ...config }) => config,
			);
			localStorage.setItem("hotkeys", JSON.stringify(hotkeyConfig));
		},

		resetToDefaults() {
			localStorage.removeItem("hotkeys");
			this.initializeHotkeys(this.defaultCallbacks);
		},
	},
});
