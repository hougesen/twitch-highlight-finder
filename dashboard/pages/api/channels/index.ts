import type { NextApiRequest, NextApiResponse } from 'next';
import { MissingFieldError } from '../../../lib/errors';
import getDbClient from '../../../lib/mongodb';
import type { IChannel } from '../../../types/models';

async function fetchChannels(): Promise<IChannel[]> {
    const db = await getDbClient();

    const collection = db.collection('channels');

    const channels = (await collection.find({}).toArray()) ?? [];

    return channels as IChannel[];
}

async function insertChannel(channelName: string): Promise<IChannel> {
    const formattedChannelName = channelName?.trim()?.toLowerCase();

    if (!formattedChannelName?.length) {
        throw new MissingFieldError('channel_name');
    }

    const db = await getDbClient();

    const collection = db.collection('channels');

    const upsertItem = {
        channel_name: formattedChannelName,
    };

    const channel = await collection.findOneAndUpdate(
        { channel_name: formattedChannelName },
        {
            $set: upsertItem,
        },
        { upsert: true }
    );

    return channel?.value as IChannel;
}

export default function handler(req: NextApiRequest, res: NextApiResponse<IChannel | IChannel[] | { error: unknown }>) {
    switch (req.method) {
        case 'GET':
            return fetchChannels()
                .then((channels) => res.status(200).send(channels))
                .catch((error?: Error) => res.status(400).send({ error: error?.message ?? error }));

        case 'POST':
            return insertChannel(req?.body?.channel_name)
                .then((channel) => res.status(201).send(channel))
                .catch((error?: Error) => res.status(400).send({ error: error?.message ?? error }));

        default:
            return res.status(405).send({ error: 'Method not allowed.' });
    }
}
