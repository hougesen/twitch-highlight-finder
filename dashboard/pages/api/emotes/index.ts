import { NextApiRequest, NextApiResponse } from 'next';

import getDbClient from '../../../lib/mongodb';
import { IEmote } from '../../../types/models';

async function fetchEmotes(): Promise<IEmote[]> {
    const db = await getDbClient();

    const emotes = await db.collection<IEmote>('emotes').find({}).toArray();

    return emotes ?? [];
}

export default async function handler(
    req: NextApiRequest,
    res: NextApiResponse<IEmote | IEmote[] | { error: unknown }>
) {
    switch (req.method) {
        case 'GET':
            return await fetchEmotes()
                .then((emotes) => res.status(200).send(emotes))
                .catch((error?: Error) => res.status(400).send({ error: error?.message ?? error }));

        case 'OPTIONS':
            return res.setHeader('Allow', ['GET']).status(200).end();

        default:
            return res.setHeader('Allow', ['GET']).status(405).send({ error: 'Method not allowed.' });
    }
}
