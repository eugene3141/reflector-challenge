import "./assets/scss/tailwind.scss";
import 'vue3-toastify/dist/index.css';

import { createApp } from 'vue'
import App from './App.vue'
import router from "./router";
import {createPinia} from 'pinia'
import Vue3Toasity from 'vue3-toastify';
import HelperPlugin from './plugins/helpers'

const pinia = createPinia()

createApp(App)
    .use(pinia)
    .use(router)
    .use(Vue3Toasity, {
        theme: "dark",
        autoClose: 3000,
        position: "bottom-center",
        dangerouslyHTMLString: true
    })
    .use(HelperPlugin)
    .mount('#app')
