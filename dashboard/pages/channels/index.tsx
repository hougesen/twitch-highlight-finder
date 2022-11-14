import axios, { AxiosError, AxiosResponse } from 'axios';
import { useEffect, useState } from 'react';
import AddChannelModal from '../../components/AddChannelModal';
import { ChannelList } from '../../components/ChannelList';
import { IChannel } from '../../types/models';

export default function ChannelsIndex() {
    const [channels, setChannels] = useState<IChannel[]>();
    const [loading, setLoading] = useState<boolean>(false);
    const [modalState, setModalState] = useState<boolean>(false);

    async function fetchChannels() {
        if (loading) return;

        setLoading(true);

        axios
            .get('/api/channels')
            .then((res: AxiosResponse<IChannel[]>) => setChannels(res?.data ?? []))
            .catch((error: AxiosError<{ error?: unknown }>) => console.error('fetchChannels', error))
            .finally(() => setLoading(false));
    }

    useEffect(() => {
        fetchChannels();
    }, []);

    function handleAddChanneClose() {
        setModalState(false);
        fetchChannels();
    }

    return (
        <>
            <div>
                <div className="flex mb-4">
                    <h1 className="font-bold text-3xl">Channels</h1>

                    <button
                        type="submit"
                        onClick={() => setModalState(true)}
                        className="border px-4 py-2 rounded ml-auto"
                    >
                        Add Channel
                    </button>
                </div>

                {channels?.length ? <ChannelList channels={channels} /> : <p>No channels found :(</p>}
            </div>

            {modalState ? <AddChannelModal closeModal={handleAddChanneClose} /> : ''}
        </>
    );
}
