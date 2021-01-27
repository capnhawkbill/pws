import { createApp } from 'vue'
import { VueCookieNext } from 'vue-cookie-next'
import App from './App.vue'
import router from './router'
import axios from 'axios'

const app = createApp(App)
app.use(VueCookieNext)
app.use(router)
app.mount('#app')

app.config.globalProperties.axios=axios
VueCookieNext.config({ expire: 0 })
