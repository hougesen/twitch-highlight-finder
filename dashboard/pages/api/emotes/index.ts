import type { NextApiRequest, NextApiResponse } from 'next';
import getDbClient from '../../../lib/mongodb';
import type { IEmote } from '../../../types/models';

async function fetchEmotes(): Promise<IEmote[]> {
    const db = await getDbClient();

    const emotes = await db.collection('twitch_emotes').find({}).toArray();

    return emotes as IEmote[];
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

        default:
            return res.status(405).send({ error: 'Method not allowed.' });
    }
}
