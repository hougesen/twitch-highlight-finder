import { MongoClient } from 'mongodb';

if (!process.env.MONGO_CONNECTION_STRING) {
    throw new Error('Invalid/Missing environment variable: "MONGO_CONNECTION_STRING"');
}

const uri = process.env.MONGO_CONNECTION_STRING;
const options = {};

let client;
let clientPromise: Promise<MongoClient>;

if (process.env.NODE_ENV === 'development') {
    if (!global._mongoClientPromise) {
        client = new MongoClient(uri, options);
        global._mongoClientPromise = client.connect();
    }
    clientPromise = global._mongoClientPromise;
} else {
    client = new MongoClient(uri, options);
    clientPromise = client.connect();
}

export default async function getDbClient(databaseName = 'highlights') {
    return (await clientPromise).db(databaseName);
}
