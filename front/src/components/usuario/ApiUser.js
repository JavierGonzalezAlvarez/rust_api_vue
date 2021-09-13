import Api from "./Api"

export class ApiUser extends Api {
    constructor() {
        super()
    }

    async getUser(id = -1) {
        return await super.get("users", id)
    }
}