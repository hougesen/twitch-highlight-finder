import { MongoClient } from 'mongodb';

if (!process.env.MONGO_CONNECTION_URI) {
    throw new Error('Invalid/Missing environment variable: "MONGO_CONNECTION_URI"');
}

const MONGO_URI = process.env.MONGO_CONNECTION_URI;
const options = {};

let client;
let clientPromise: Promise<MongoClient>;

if (process.env.NODE_ENV === 'development') {
    if (!global._mongoClientPromise) {
        client = new MongoClient(MONGO_URI, options);
        global._mongoClientPromise = client.connect();
    }
    clientPromise = global._mongoClientPromise;
} else {
    client = new MongoClient(MONGO_URI, options);
    clientPromise = client.connect();
}

export default async function getDbClient(databaseName = 'highlights') {
    return (await clientPromise).db(databaseName);
}
