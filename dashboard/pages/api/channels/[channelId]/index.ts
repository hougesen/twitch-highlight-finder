import { ObjectId } from 'mongodb';
import type { NextApiRequest, NextApiResponse } from 'next';
import { MissingFieldError } from '../../../../lib/errors';
import getDbClient from '../../../../lib/mongodb';
import { IChannel } from '../../../../types/models';

async function getChannelById(channelId: string): Promise<IChannel | null> {
    const db = await getDbClient();

    const channel = await db.collection<IChannel>('channels').findOne({ _id: new ObjectId(channelId) });

    return channel;
}

async function updateChannel(channelId: string, channelName: string): Promise<IChannel | null> {
    const formattedChannelName = channelName?.trim()?.toLowerCase();

    if (!formattedChannelName?.length) {
        throw new MissingFieldError('channel_name');
    }

    const db = await getDbClient();

    const channel = await db.collection<IChannel>('channels').findOneAndUpdate(
        {
            _id: new ObjectId(channelId),
        },
        {
            $set: {
                channel_name: formattedChannelName,
            },
        }
    );

    return channel?.value;
}

async function deleteChannelById(channelId: string) {
    const db = await getDbClient();

    await db.collection('channels').deleteOne({ _id: new ObjectId(channelId) });
}

export default function handler(
    req: NextApiRequest,
    res: NextApiResponse<IChannel | null | void | { error: unknown }>
) {
    const channelId = req?.query?.channelId;

    if (!channelId || typeof channelId !== 'string') {
        return res.status(400).send({ error: 'channel_id is not a string' });
    }

    switch (req.method) {
        case 'GET':
            return getChannelById(channelId)
                .then((channel) => res.status(channel ? 200 : 404).send(channel))
                .catch((error?: Error) => res.status(400).send({ error: error?.message ?? error }));

        case 'PUT':
            return updateChannel(channelId, req?.body?.channel_name)
                .then((channel) => res.status(200).send(channel))
                .catch((error?: Error) => res.status(400).send({ error: error?.message ?? error }));

        case 'DELETE':
            return deleteChannelById(channelId)
                .then(() => res.status(200).send())
                .catch((error?: Error) => res.status(400).send({ error: error?.message ?? error }));

        default:
            return res
                .setHeader('Allow', ['GET', 'PUT', 'DELETE'])
                .status(req?.method === 'OPTIONS' ? 200 : 405)
                .send({ error: 'Method not allowed.' });
    }
}
