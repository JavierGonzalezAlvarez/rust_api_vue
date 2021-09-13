<template>
  <div>
    <v-data-table
      :headers="headers"
      :items="items"
      item-key="name"
      class="elevation-1"
      :search="search"
      :custom-filter="filterOnlyCapsText"
    >
      <template v-slot:top>
        <v-text-field
          v-model="search"
          label="Buscar (EN MAYUSCULA)"
          class="mx-4"
        ></v-text-field>
      </template>
      <template v-slot:body.append>
        <tr>
          <td></td>
          <td>
            <v-text-field
              v-model="search"
              type="number"
              label="Less than"
            ></v-text-field>
          </td>
          <td colspan="4"></td>
        </tr>
      </template>
    </v-data-table>
  </div>
</template>

<script>
import { ApiUser } from "./ApiUser";

export default {
  name: "Listado",
  data() {
    return {
      items: [],
      api: new ApiUser(),
      loading: false,
      search: "",
      headers: [
        { text: "ID", value: "id" },
        {
          text: "Name",
          align: "start",
          sortable: true,
          value: "name",
        },
        {
          text: "Adress",
          align: "start",
          sortable: true,
          value: "adress",
        },
        { text: "E-Mail", value: "email", sortable: false },
        { text: "Password", value: "password", sortable: false },
        { text: "Comments", value: "comments", sortable: false },
        { text: "Created at", value: "created_at", sortable: false },
      ],

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

  watch: {
    dialog(val) {
      val || this.close();
    },
  },

  methods: {
    async iniciar() {
      try {
        this.loading = true;
        let r = await this.api.getUser();
        this.items = r;
        console.log(this.items);
      } catch (error) {
        console.log(error);
      } finally {
        this.loading = false;
      }
    },

    filterOnlyCapsText(value, search) {
      return (
        value != null &&
        search != null &&
        typeof value === "string" &&
        value.toString().toLocaleUpperCase().indexOf(search) !== -1
      );
    },

    editItem(item) {
      this.editedIdex = this.items.indexOf(item);
      this.editedItem = Object.assign({}, item);
      this.dialog = true;
    },
  },

  created() {
    this.iniciar();
  },
};
</script>

<style>
</style>