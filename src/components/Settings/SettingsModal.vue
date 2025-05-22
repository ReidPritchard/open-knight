<script setup lang="ts">
import { ref } from "vue";
import { useGlobalStore } from "../../stores";
import { useSettingsStore } from "../../stores/settings";
import type { Hotkey } from "../../stores/settings";

defineProps<{
  isOpen: boolean;
}>();

const emit = defineEmits(["close"]);

const settingsStore = useSettingsStore();
const globalStore = useGlobalStore();
const editingHotkey = ref<string | null>(null);
const newKey = ref<Partial<Omit<Hotkey, "id" | "callback">> | null>(null);

const startEditing = (hotkeyId: string) => {
  editingHotkey.value = hotkeyId;
  newKey.value = {};
  // Start listening for key combinations
  document.addEventListener("keydown", handleKeyPress);
};

const stopEditing = () => {
  editingHotkey.value = null;
  newKey.value = null;
  document.removeEventListener("keydown", handleKeyPress);
};

const handleKeyPress = (e: KeyboardEvent) => {
  e.preventDefault();
  if (!editingHotkey.value || !newKey.value) return;

  // If the key is a modifier key, don't update the hotkey yet
  // that way we can capture the key+modifier combination
  if (
    e.key === "Control" ||
    e.key === "Alt" ||
    e.key === "Shift" ||
    e.key === "Meta"
  )
    return;

  newKey.value = {
    key: e.key,
    ctrl: e.ctrlKey,
    shift: e.shiftKey,
    alt: e.altKey,
    meta: e.metaKey,
  };

  // Update the hotkey
  settingsStore.updateHotkey(editingHotkey.value, newKey.value);
  stopEditing();
};
</script>

<template>
  <dialog :class="{ modal: true, 'modal-open': isOpen }">
    <div class="modal-box">
      <div role="tablist" class="tabs tabs-border">
        <input type="radio" name="settings" class="tab" aria-label="UI" />
        <div class="tab-content border-base-300 bg-base-100 p-10">
          <h3 class="font-bold text-lg mb-4">UI</h3>
        </div>

        <input
          type="radio"
          name="settings"
          class="tab"
          aria-label="Shortcuts"
        />
        <div class="tab-content border-base-300 bg-base-100 p-10">
          <h3 class="font-bold text-lg mb-4">Keyboard Shortcuts</h3>

          <div class="overflow-x-auto">
            <table class="table">
              <thead>
                <tr>
                  <th>Action</th>
                  <th>Shortcut</th>
                  <th>Edit</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="hotkey in settingsStore.hotkeys" :key="hotkey.id">
                  <td>{{ hotkey.description }}</td>
                  <td>
                    <div class="flex gap-1 items-center">
                      <template v-if="editingHotkey === hotkey.id">
                        <span class="text-sm text-accent"
                          >Press new key combination...</span
                        >
                      </template>
                      <template v-else>
                        <template v-if="hotkey.ctrl"
                          ><kbd class="kbd kbd-sm">ctrl</kbd> +</template
                        >
                        <template v-if="hotkey.alt"
                          ><kbd class="kbd kbd-sm">alt</kbd> +</template
                        >
                        <template v-if="hotkey.shift"
                          ><kbd class="kbd kbd-sm">shift</kbd> +</template
                        >
                        <template v-if="hotkey.meta"
                          ><kbd class="kbd kbd-sm">⌘</kbd> +</template
                        >
                        <kbd class="kbd kbd-sm">{{ hotkey.key }}</kbd>
                      </template>
                    </div>
                  </td>
                  <td>
                    <button
                      class="btn btn-ghost btn-sm"
                      @click="
                        editingHotkey === hotkey.id
                          ? stopEditing()
                          : startEditing(hotkey.id)
                      "
                    >
                      <span class="material-symbols-outlined">
                        {{ editingHotkey === hotkey.id ? "close" : "edit" }}
                      </span>
                    </button>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
      </div>

      <!-- Modal footer actions -->
      <div class="modal-action">
        <button class="btn btn-ghost" @click="settingsStore.resetToDefaults()">
          Reset to Defaults
        </button>
        <button class="btn" @click="emit('close')">Close</button>
      </div>
    </div>
    <form method="dialog" class="modal-backdrop" @click="emit('close')">
      <button>close</button>
    </form>
  </dialog>
</template>
