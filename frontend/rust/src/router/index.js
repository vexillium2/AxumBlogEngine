import { createRouter, createWebHashHistory } from "vue-router"

const routes = [
  {
    path: "/",
    name: "Main",
    component: () => import("../view/Main.vue"),
  },
]

const router = createRouter({
  history: createWebHashHistory(),
  routes,
})

export default router
