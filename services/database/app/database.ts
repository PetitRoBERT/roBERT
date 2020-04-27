import mongoose from "mongoose";


const server = '127.0.0.1:27017';
const database = 'devpetitrobert';
class Database {
  constructor() {
    this._connect()
  }
  _connect() {
    mongoose.connect(`mongodb://${server}/${database}`, { useNewUrlParser: true, useUnifiedTopology: true })
      .then((mes: any) => {
        console.log(`Database connection successful ${Object.keys(mes)}`)
      })
      .catch((err: any) => {
        console.error('Database connection error', err);
      })
  }
}
export default Database;