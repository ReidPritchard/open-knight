import type { Component } from "vue";

export interface MenuItem {
	id: string;
	label: string;
	icon?: Component;
	type?: "normal" | "divider" | "destructive";
	disabled?: boolean;
}
