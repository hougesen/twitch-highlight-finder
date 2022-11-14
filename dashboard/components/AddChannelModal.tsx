import { ChangeEvent, FormEvent, useState } from 'react';
import axios, { AxiosError } from 'axios';
import Spinner from './Spinner';

export default function AddChannelModal({ closeModal }: { closeModal: () => void }) {
    const [channelName, setChannelName] = useState('');
    const [errorMessage, setErrorMessage] = useState('');
    const [processing, setProcessing] = useState(false);

    function handleChannelNameChange(e: ChangeEvent<HTMLInputElement>) {
        setChannelName(e?.target?.value?.toLowerCase()?.trim() ?? '');
    }

    async function handleSubmit(e: FormEvent<HTMLFormElement>) {
        e.preventDefault();
        setProcessing(true);
        axios
            .post('/api/channels', { channel_name: channelName })
            .then(() => closeModal())
            .catch((error: AxiosError<{ error?: string }>) =>
                setErrorMessage(error?.response?.data?.error ?? 'Something went wrong :(')
            )
            .finally(() => setProcessing(false));
    }

    return (
        <div className="max-w-screen fixed top-0 left-0 z-50 flex h-screen w-screen items-center justify-center bg-[rgba(169,169,169,0.5)]">
            <div className="min-w-fit bg-white p-8 rounded-lg flex flex-col gap-4">
                <div className="flex items-center gap-8">
                    <h2 className="text-xl font-bold">Add Channel</h2>

                    <button
                        type="button"
                        onClick={() => closeModal()}
                        className="ml-auto bg-black text-white h-8 w-8 rounded-full text-xl font-bold"
                    >
                        X
                    </button>
                </div>

                <form onSubmit={handleSubmit} className="flex flex-col gap-4">
                    <div>
                        <label htmlFor="channel_name">Channel Name:</label>

                        <input
                            type="text"
                            name="channel_name"
                            onChange={handleChannelNameChange}
                            className="block border-2 rounded mt-2"
                            required
                            value={channelName}
                        />
                    </div>

                    {errorMessage?.length ? <p className="text-red-700 text-center font-bold">{errorMessage}</p> : ''}

                    <button
                        type="submit"
                        className="border px-4 py-2 disabled:cursor-not-allowed flex items-center justify-center "
                        disabled={processing}
                    >
                        {processing ? <Spinner /> : ''}
                        Save
                    </button>
                </form>
            </div>
        </div>
    );
}
