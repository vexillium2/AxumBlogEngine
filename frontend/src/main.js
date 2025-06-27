// import "./assets/main.css"

import { createApp } from "vue"
import App from "./App.vue"
import router from "./router/index"
import ElementPlus from "element-plus" //引入组件库
import "element-plus/dist/index.css"

import * as ElementPlusIconsVue from "@element-plus/icons-vue" //引入全局图标

const app = createApp(App)
app.use(ElementPlus)
app.use(router)
app.mount("#app")
