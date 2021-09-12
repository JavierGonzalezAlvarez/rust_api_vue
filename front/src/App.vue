<template>
  <v-app>
    <v-app-bar app color="primary" dense dark>
      <v-app-bar-nav-icon></v-app-bar-nav-icon>
      <v-toolbar-title class="color">Menu Principal</v-toolbar-title>

      <v-divider></v-divider>

      <!-- <v-list color="primary" dense dark> -->
      <div v-for="m in menu" :key="m.nombre" color="primary" dense dark>
        <router-link :to="m.ruta">
          <v-list-item link>
            <v-list-item-icon
              ><v-icon>{{ m.icono }}</v-icon></v-list-item-icon
            >
            <v-list-item-title class="color">{{ m.nombre }}</v-list-item-title>
          </v-list-item>
        </router-link>
      </div>
      <!-- </v-list> -->
    </v-app-bar>

    <v-main>
      <!-- ver router en main-->
      <router-view></router-view>
    </v-main>
  </v-app>
</template>

<script>
//import HelloWorld from './components/HelloWorld';
//import Home from "./components/Home";

export default {
  name: "App",

  components: {
    //HelloWorld,
    //Home,
  },

  data() {
    return {
      menu: [
        { ruta: "/", nombre: "Inicio", icono: "home" },
        { ruta: "/user", nombre: "Usuarios", icono: "group" },
      ],
    };
  },
};
</script>

<style>
.color {
  color: white;
}
</style>