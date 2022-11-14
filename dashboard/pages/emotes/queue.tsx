import axios from 'axios';
import { useEffect, useState } from 'react';
import EmoteImage from '../../components/EmoteImage';
import Spinner from '../../components/Spinner';
import { IEmote } from '../../types/models';

export default function EmotesQueue() {
    const [emote, setEmote] = useState<IEmote | null>(null);
    const [loading, setLoading] = useState(false);

    async function setEmoteScore(score: number) {
        axios.put(`/api/emotes/${emote?._id}`, { score }).then(() => fetchNextEmote());
    }

    async function fetchNextEmote() {
        if (loading) return;

        setLoading(true);

        axios
            .get('/api/emotes/next')
            .then((res) => setEmote(res?.data ?? null))
            .catch((error) => {
                console.error('fetchEmote error', error?.response?.data);
                setEmote(null);
            })
            .finally(() => setLoading(false));
    }

    useEffect(() => {
        fetchNextEmote();
    }, []);

    const maxScore = 10;

    if (emote) {
        return (
            <div className="flex flex-col items-center justify-center">
                <EmoteImage emote_id={emote.emote_id} />

                <h2 className="text-xl font-bold mb-8">{emote.name}</h2>

                <div className="flex gap-4">
                    {[...Array(maxScore + 1)].map((_, i) => (
                        <button
                            className="font-bold text-xl border w-16 h-16 disabled:cursor-not-allowed"
                            type="button"
                            disabled={loading}
                            key={i}
                            onClick={() => setEmoteScore(i)}
                        >
                            {i}
                        </button>
                    ))}
                </div>
            </div>
        );
    }

    if (loading) {
        return (
            <div className="w-full flex gap-2 items-center justify-center">
                <Spinner />

                <h1 className="text-xl font-bold">Loading</h1>
            </div>
        );
    }

    return (
        <div>
            <h1 className="text-xl font-bold">No pending emotes found</h1>
        </div>
    );
}
