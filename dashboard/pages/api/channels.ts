import { Document, WithId } from 'mongodb';
import type { NextApiRequest, NextApiResponse } from 'next';
import clientPromise from '../../lib/mongodb';

export declare interface IChannel extends WithId<Document> {
    channel_name: String;
}

async function fetchChannels(): Promise<IChannel[]> {
    const dbClient = await clientPromise;

    const db = dbClient.db('highlights');

    const collection = db.collection('channels');

    const channels = (await collection.find({}).toArray()) as IChannel[];

    return channels ?? [];
}

export default function handler<T>(
    req: NextApiRequest,
    res: NextApiResponse<IChannel | IChannel[] | { error: unknown }>
) {
    switch (req.method) {
        case 'GET':
            fetchChannels().then((channels) => res.status(200).send(channels));
            break;
        default:
            res.status(405).send({ error: 'Message not allowed' });
            break;
    }
}
