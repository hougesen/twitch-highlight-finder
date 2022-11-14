import { NextApiRequest, NextApiResponse } from 'next';
import getDbClient from '../../../lib/mongodb';

export default async function handler(_req: NextApiRequest, res: NextApiResponse<{ count: number }>) {
    const db = await getDbClient();

    const count = await db.collection('emotes').countDocuments({ score: { $exists: false } });

    return res.status(200).send({ count });
}
