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

class MissingFieldError extends Error {
    constructor(field: string) {
        super(`ERROR: Item is missing field ${field}`);
    }
}

async function insertChannel(channelName: string): Promise<IChannel> {
    const formattedChannelName = channelName?.trim()?.toLowerCase();

    if (!formattedChannelName?.length) {
        throw new MissingFieldError('channel_name');
    }

    const dbClient = await clientPromise;

    const db = dbClient.db('highlights');

    const collection = db.collection('channels');

    const upsertItem = {
        channel_name: formattedChannelName,
    };

    return await collection
        .findOneAndUpdate(
            { channel_name: formattedChannelName },
            {
                $set: upsertItem,
            },
            { upsert: true }
        )
        .then((c) => c.value as IChannel)
        .catch((error: Error) => {
            throw error;
        });
}

export default function handler<T>(
    req: NextApiRequest,
    res: NextApiResponse<IChannel | IChannel[] | { error: unknown }>
) {
    switch (req.method) {
        case 'GET':
            fetchChannels()
                .then((channels) => res.status(200).send(channels))
                .catch((error: Error) => res.status(400).send({ error: error?.message ?? error }));
            break;

        case 'POST':
            insertChannel(req?.body?.channel_name)
                .then((channel) => res.status(201).send(channel))
                .catch((error: Error) => res.status(400).send({ error: error?.message ?? error }));
            break;

        default:
            res.status(405).send({ error: 'Method not allowed.' });
            break;
    }
}
