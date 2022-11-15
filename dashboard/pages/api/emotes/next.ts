import { NextApiRequest, NextApiResponse } from 'next';

import getDbClient from '../../../lib/mongodb';
import { IEmote } from '../../../types/models';

async function getNextEmote(): Promise<IEmote | null> {
    let db = await getDbClient();

    const emote = await db.collection<IEmote>('emotes').findOne(
        { score: { $exists: false } },
        {
            sort: { channel_id: 1 },
        }
    );

    return emote;
}

export default async function handler(req: NextApiRequest, res: NextApiResponse<IEmote | null | { error: unknown }>) {
    switch (req.method) {
        case 'GET':
            return await getNextEmote()
                .then((emote) => res.status(200).send(emote))
                .catch((error?: Error) => res.status(400).send({ error: error?.message ?? error }));

        case 'OPTIONS':
            return res.setHeader('Allow', ['GET']).status(200).end();

        default:
            return res.setHeader('Allow', ['GET']).status(405).send({ error: 'Method not allowed.' });
    }
}
