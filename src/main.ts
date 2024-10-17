import { createApp } from "vue";
import App from "./App.vue";
import PrimeVue from 'primevue/config';
import Aura from '@primevue/themes/aura';
import Button from "primevue/button";
import InputText from "primevue/inputtext";
import FloatLabel from "primevue/floatlabel";
import DataTable from "primevue/datatable";
import Column from "primevue/column";
import Row from "primevue/row";

const app = createApp(App);
app.use(PrimeVue, {
    theme: {
        preset: Aura,
        options: {
            prefix: 'p',
            darkModeSelector: 'system',
            cssLayer: false
        }
    }
});
app.component('Button', Button);
app.component('InputText', InputText);
app.component('FloatLabel', FloatLabel);
app.component('DataTable', DataTable);
app.component('Column', Column);
app.component('Row', Row);


app.mount("#app");
