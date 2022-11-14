import axios, { AxiosResponse } from 'axios';
import { useRouter } from 'next/router';
import { useEffect, useState } from 'react';
import { MessageList } from '../../../components/MessageList';
import Spinner from '../../../components/Spinner';
import { IChannel, ITwitchChatMessage } from '../../../types/models';

export default function ChannelPage() {
    const [channel, setChannel] = useState<IChannel | null>(null);
    const [loadingChannel, setLoadingChannel] = useState(false);

    const [channelMessages, setChannelMessages] = useState<ITwitchChatMessage[]>([]);
    const [loadingChannelMessages, setLoadingChannelMessages] = useState(false);

    const router = useRouter();

    const { channelId } = router.query;

    async function fetchChannel() {
        console.log('fetchChannel', { channelId });

        if (!channelId) return;

        if (loadingChannel) return;

        setLoadingChannel(true);

        axios
            .get(`/api/channels/${channelId}`)
            .then((res: AxiosResponse<IChannel>) => setChannel(res?.data))
            .finally(() => setLoadingChannel(false));
    }

    async function fetchChannelMessages() {
        console.log('fetchChannelMessages', { channelId });

        if (!channelId) return;

        if (loadingChannelMessages) return;

        axios
            .get(`/api/channels/${channelId}/messages`)
            .then((res: AxiosResponse<ITwitchChatMessage[]>) => setChannelMessages(res?.data ?? []))
            .finally(() => setLoadingChannelMessages(false));
    }

    useEffect(() => {
        if (!channelId) return;

        fetchChannel();
        fetchChannelMessages();
    }, [channelId]);

    if (loadingChannel) {
        return (
            <div className="w-full flex gap-2 items-center justify-center">
                <Spinner />

                <h1 className="text-xl font-bold">Loading</h1>
            </div>
        );
    }

    if (channel) {
        return (
            <div>
                <h1 className="text-xl font-bold">{channel?.channel_name ?? 'N/A'}</h1>

                <MessageList messages={channelMessages} />
            </div>
        );
    }

    return <h1 className="text-xl font-bold text-center">Channel not found :(</h1>;
}
