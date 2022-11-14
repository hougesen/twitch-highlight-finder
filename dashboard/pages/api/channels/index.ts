import axios, { AxiosResponse } from 'axios';
import type { NextApiRequest, NextApiResponse } from 'next';
import { MissingFieldError } from '../../../lib/errors';
import getDbClient from '../../../lib/mongodb';
import type { IChannel } from '../../../types/models';

async function getTwitchAuthHeader(): Promise<{
    headers: {
        'Client-ID': string;
        Accept: string;
        Authorization: string;
    };
}> {
    const { CLIENT_ID, CLIENT_SECRET } = process?.env;

    if (!CLIENT_ID?.length) {
        throw new Error('Invalid/Missing environment variable: "CLIENT_ID"');
    }

    if (!CLIENT_SECRET?.length) {
        throw new Error('Invalid/Missing environment variable: "CLIENT_SECRET"');
    }

    const twitchAccessToken = await axios
        .post(
            `https://id.twitch.tv/oauth2/token?client_id=${CLIENT_ID}&client_secret=${CLIENT_SECRET}&grant_type=client_credentials&state=def`
        )
        .then((res: AxiosResponse<{ access_token: string }>) => res?.data?.access_token)
        .catch((error) => {
            console.error('getTwitchToken error getting oauth token', error?.response?.data);
            return null;
        });

    if (!twitchAccessToken) {
        throw new Error('getTwitchAuthHeader: Error getting Twitch Access Token');
    }

    const axiosConfig = {
        headers: {
            'Client-ID': CLIENT_ID,
            Accept: 'application/vnd.twitchtv.v5+json',
            Authorization: `Bearer ${twitchAccessToken}`,
        },
    };

    return axiosConfig;
}

async function fetchChannels(): Promise<IChannel[]> {
    const db = await getDbClient();

    const collection = db.collection('channels');

    const channels = (await collection.find({}).toArray()) ?? [];

    return channels as IChannel[];
}

async function getChannelId(channelName: string): Promise<string | null> {
    return await axios
        .get(`https://api.twitch.tv/helix/users?login=${channelName}`, await getTwitchAuthHeader())
        .then((res: AxiosResponse<{ data: Array<{ id: string }> }>) => {
            if (res?.data?.data?.length) {
                return res?.data?.data[0]?.id;
            }

            return null;
        })
        .catch((error) => {
            console.error('getChannelId error', error?.response?.data);
            return null;
        });
}

async function insertChannel(channelName: string): Promise<IChannel> {
    const formattedChannelName = channelName?.trim()?.toLowerCase();

    if (!formattedChannelName?.length) {
        throw new MissingFieldError('channel_name');
    }

    const db = await getDbClient();

    const collection = db.collection('channels');

    const channelId = await getChannelId(channelName);

    if (!channelId) {
        throw new Error('Error: Not a valid Twitch Channel');
    }

    const upsertItem: Partial<IChannel> = {
        channel_name: formattedChannelName,
        channel_id: channelId,
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
            return res.setHeader('Allow', ['GET', 'POST']).status(405).send({ error: 'Method not allowed.' });
    }
}
