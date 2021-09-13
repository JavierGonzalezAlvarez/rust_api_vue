import Vue from 'vue'
import VueRouter from 'vue-router'

import Home from "./components/Home"
import User from "./components/usuario/Usuario"
import Listado from "./components/usuario/Listado"

Vue.use(VueRouter);

//constante para la ruta, y será un arreglo de objetos
const routes = [
    //declaramos la ruta raiz
    {
        path: "/",
        name: "home",
        component: Home
    },
    {
        path: "/user",
        name: "user",
        component: User
    },
    {
        path: "/userlistado",
        name: "listado",
        component: Listado
    }

]

//creamos una instancia de VueRouter
const router = new VueRouter({
    //en el constructor ponemos
    mode: 'history',
    routes
})

//como lo vamos a utilizarlo en otros componentes, lo exportamos
export default router