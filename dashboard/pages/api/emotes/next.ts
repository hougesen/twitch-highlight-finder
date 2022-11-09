import type { NextApiRequest, NextApiResponse } from 'next';
import getDbClient from '../../../lib/mongodb';
import type { IEmote } from '../../../types/models';

async function getNextEmote(): Promise<IEmote | null> {
    let db = await getDbClient();
    console.log('next');

    const emote = await db.collection('emotes').findOne({ score: { $exists: false } });

    return emote as IEmote | null;
}

export default async function handler(req: NextApiRequest, res: NextApiResponse<IEmote | null | { error: unknown }>) {
    switch (req.method) {
        case 'GET':
            return await getNextEmote()
                .then((emote) => res.status(200).send(emote))
                .catch((error?: Error) => res.status(400).send({ error: error?.message ?? error }));

        default:
            return res.status(405).send({ error: 'Method not allowed.' });
    }
}