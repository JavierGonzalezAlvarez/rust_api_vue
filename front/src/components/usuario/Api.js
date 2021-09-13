export default class Api {
  constructor() {
    this.SERVER_URL = 'http://127.0.0.1:8080/'
  }

  async get(nombre, id = -1) {

    let url = this.SERVER_URL + nombre; // + "/";
    if (id !== -1) {
      url += id + "/"
    }

    console.log("url solicitada => ", url)

    const res = await fetch(url, {
      method: "GET",
      //mode: 'no-cors',
      headers: {
        'Content-Type': 'application/json',
        'Access-Control-Allow-Origin': '*',
      }

    });

    const items = await res.json();
    console.log("respuesta 0 =>", items);

    if (items.results === undefined) {
      console.log("respuesta 2 =>", items);
      return items
    }

    console.log("respuesta 3 =>", items);
    return items.results;
  }


}
