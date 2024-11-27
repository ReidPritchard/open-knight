import Column from "primevue/column";
import PrimeVue from "primevue/config";
import DataTable from "primevue/datatable";
import FloatLabel from "primevue/floatlabel";
import InputText from "primevue/inputtext";
import Row from "primevue/row";
import Select from "primevue/select";

import DialogService from "primevue/dialogservice";
import { createPinia } from 'pinia'

import CustomTheme from "./theme";
import "./style.css";

import { createApp } from "vue";
import App from "./App.vue";

const app = createApp(App);
const pinia = createPinia()

app.use(pinia)
app.use(PrimeVue, {
  theme: {
    preset: CustomTheme,
    options: {
      prefix: "p",
      darkModeSelector: ".dark",
      cssLayer: false,
    },
  },
});
app.use(DialogService);

app.component("InputText", InputText);
app.component("FloatLabel", FloatLabel);
app.component("DataTable", DataTable);
app.component("Column", Column);
app.component("Row", Row);
app.component("Select", Select);

app.mount("#app");
