import { createRouter, createWebHashHistory } from "vue-router";
import AccountView from "./views/AccountView.vue";
import GameView from "./views/GameView.vue";
import GamesView from "./views/GamesView.vue";
import SettingsView from "./views/SettingsView.vue";

export const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    { path: "/", redirect: "/games" },
    { path: "/games", name: "games", component: GamesView },
    { path: "/games/:id", name: "game", component: GameView, props: true },
    { path: "/account", name: "account", component: AccountView },
    { path: "/settings", name: "settings", component: SettingsView },
  ],
});
