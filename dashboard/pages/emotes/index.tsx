import axios, { AxiosError, AxiosResponse } from 'axios';
import { useEffect, useState } from 'react';

import EmoteList from '../../components/EmoteList';
import { IEmote } from '../../types/models';

export default function EmotesIndex() {
    const [emotes, setEmotes] = useState<IEmote[]>([]);

    const [loading, setLoading] = useState<boolean>(false);

    async function fetchEmotes() {
        if (loading) return;

        setLoading(true);

        axios
            .get('/api/emotes')
            .then((res: AxiosResponse<IEmote[]>) => setEmotes(res?.data ?? []))
            .catch((error: AxiosError<{ error?: unknown }>) => console.error('fetchEmotes', error?.response?.data))
            .finally(() => setLoading(false));
    }

    useEffect(() => {
        fetchEmotes();
    }, []);

    return (
        <div>
            <h1>Emotes</h1>

            {emotes?.length ? <EmoteList emotes={emotes} /> : <div>No emotes found :(</div>}
        </div>
    );
}
