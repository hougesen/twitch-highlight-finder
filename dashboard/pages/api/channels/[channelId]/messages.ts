import { ObjectId } from 'mongodb';
import { NextApiRequest, NextApiResponse } from 'next';

import getDbClient from '../../../../lib/mongodb';
import { IChannel, ITwitchChatMessage } from '../../../../types/models';

export default async function handler(
    req: NextApiRequest,
    res: NextApiResponse<ITwitchChatMessage[] | { error: string }>
) {
    if (req?.method !== 'GET') {
        return res
            .setHeader('Allow', ['GET'])
            .status(req?.method === 'OPTIONS' ? 200 : 405)
            .end();
    }

    const { channelId } = req?.query;

    if (!channelId || typeof channelId !== 'string') {
        return res.status(400).send({ error: 'channel_id is not a string ' + channelId });
    }

    const db = await getDbClient();

    const channel = await db.collection('channels').findOne<IChannel>({ _id: new ObjectId(channelId) });

    if (!channel?.channel_name) {
        return res.status(404).send({ error: 'Channel not found' });
    }

    const chatMessages = await db
        .collection<ITwitchChatMessage>('twitch_messages')
        .find({ channel: channel?.channel_name }, { sort: { timestamp: -1 } })
        .toArray();

    return res.status(200).send(chatMessages);
}
