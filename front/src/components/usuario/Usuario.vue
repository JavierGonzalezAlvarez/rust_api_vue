<template>
  <v-container fill-height fluid class="grey lighten-5">
    <v-row row class="text-xs-center" align="center" justify="center">
      <v-col cols="12" sm="4">
        <v-card flat>
          <v-card-title primary-title>
            <h4>Añadir Usuario</h4>
          </v-card-title>
          <v-form ref="form" v-model="valid" lazy-validation>
            <v-text-field
              prepend-icon="person"
              v-model="obj.name"
              label="Name"
              ref="name"
            ></v-text-field>
            <v-text-field
              prepend-icon="person"
              v-model="obj.adress"
              label="adress"
              ref="adress"
            ></v-text-field>
            <v-text-field
              prepend-icon="phone"
              v-model="obj.telephone"
              label="telephone"
              ref="telephone"
            ></v-text-field>
            <v-text-field
              prepend-icon="email"
              v-model="obj.email"
              label="email"
              ref="email"
            ></v-text-field>
            <v-text-field
              prepend-icon="lock"
              v-model="obj.password"
              label="password"
              type="password"
            ></v-text-field>
            <v-text-field
              prepend-icon="description"
              v-model="obj.comments"
              label="comments"
              type="comments"
            ></v-text-field>
            <v-card-actions>
              <v-btn primary large block @click="adduser">Añadir Usuario</v-btn>
            </v-card-actions>
          </v-form>
        </v-card>
        <!-- </v-container> -->
      </v-col>
    </v-row>
  </v-container>
</template>

<script>
import { ApiUser } from "./ApiUser";

export default {
  name: "User",
  data() {
    return {
      obj: {
        name: "",
        adress: "",
        telephone: "",
        email: "",
        password: "",
        comments: "",
      },
      api: new ApiUser(),
      valid: true,

      editedIdex: -1,
      editedItem: {
        id: -1,
        name: "",
        adress: "",
        email: "",
        password: "",
        comments: "",
      },
      defaultItem: {
        id: -1,
        name: "",
        adress: "",
        email: "",
        password: "",
        comments: "",
      },
    };
  },
  mounted() {
    this.$refs.name.focus();
  },
  methods: {
    async adduser() {
      let name = this.name;
      //let password = this.password;

      if (name == "") {
        this.alert("Requerido");
        return;
      }
      //if (password == "" || password.length <= 5) {
      //  this.alert("Requerido, mínimo 5 caracteres");
      //  return;
      //}

      console.log(this.obj);
      await this.api.postUser(this.obj);
    },
  },
};
</script>


<style>
</style>