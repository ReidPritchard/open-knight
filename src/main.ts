import Button from "primevue/button";
import Column from "primevue/column";
import PrimeVue from "primevue/config";
import DataTable from "primevue/datatable";
import FloatLabel from "primevue/floatlabel";
import InputText from "primevue/inputtext";
import Row from "primevue/row";
import Select from "primevue/select";
import Panel from "primevue/panel";
import Tabs from "primevue/tabs";
import TabPanel from "primevue/tabpanel";

import CustomTheme from "./theme";

import { createApp } from "vue";
import App from "./App.vue";

const app = createApp(App);

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

app.component("Button", Button);
app.component("InputText", InputText);
app.component("FloatLabel", FloatLabel);
app.component("DataTable", DataTable);
app.component("Column", Column);
app.component("Row", Row);
app.component("Select", Select);
app.component("Panel", Panel);
app.component("Tabs", Tabs);
app.component("TabPanel", TabPanel);

app.mount("#app");
