import { IEmote } from '../types/models';
import EmoteImage from './EmoteImage';

// TODO: Decide if i should make a standalone Table component
export default function EmoteList({ emotes }: { emotes: IEmote[] }) {
    return (
        <div className="flex flex-col">
            <div className="overflow-x-auto">
                <div className="inline-block min-w-full py-2">
                    <div className="overflow-hidden">
                        <table className="min-w-full">
                            <thead className="border-b">
                                <tr>
                                    <th className="px-6 py-4 text-left text-sm font-medium text-gray-900" scope="col">
                                        #
                                    </th>

                                    <th className="px-6 py-4 text-left text-sm font-medium text-gray-900" scope="col">
                                        Broadcaster ID
                                    </th>

                                    <th className="px-6 py-4 text-left text-sm font-medium text-gray-900" scope="col">
                                        Score
                                    </th>
                                </tr>
                            </thead>

                            <tbody>
                                {emotes.map((e, i) => (
                                    <tr key={i} className="border-b bg-white">
                                        <td className="whitespace-nowrap px-6 py-4 text-sm flex gap-4 items-center font-medium text-gray-900">
                                            <EmoteImage width={64} emote_id={e.emote_id} />
                                            <p>{e?.name}</p>
                                        </td>

                                        <td className="whitespace-nowrap px-6 py-4 text-sm font-medium text-gray-900">
                                            {e?.channel_id ?? 'Global'}
                                        </td>

                                        <td className="whitespace-nowrap px-6 py-4 text-sm font-medium text-gray-900">
                                            {e?.score ?? 'N/A'}
                                        </td>
                                    </tr>
                                ))}
                            </tbody>
                        </table>
                    </div>
                </div>
            </div>
        </div>
    );
}
