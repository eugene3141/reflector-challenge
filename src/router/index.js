import { createRouter, createWebHistory } from "vue-router";

import routes from "./route";

const router = createRouter({
    history: createWebHistory(import.meta.BASE_URL),
    base: import.meta.BASE_URL,
    routes,
    scrollBehavior(to, from, savedPosition) {
        if (savedPosition) {
            return savedPosition;
        } else {
            return { top: 0 };
        }
    },
});
router.beforeEach((to, from, next) => {
    const titleText = to.name;
    const words = titleText.split(" ");
    const wordslength = words.length;
    for (let i = 0; i < wordslength; i++) {
        words[i] = words[i][0].toUpperCase() + words[i].substr(1);
    }

    document.title = words.join(' ') + " - P2P Lending";

    next();
});

export default router;
