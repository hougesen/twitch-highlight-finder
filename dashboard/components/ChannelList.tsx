import { IChannel } from '../types/models';

// TODO: Decide if i should make a standalone Table component
export function ChannelList({ channels }: { channels: IChannel[] }) {
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
                                        Name
                                    </th>

                                    <th className="px-6 py-4 text-left text-sm font-medium text-gray-900" scope="col">
                                        Channel id
                                    </th>
                                </tr>
                            </thead>

                            <tbody>
                                {channels.map((c, i) => (
                                    <tr key={i} className="border-b bg-white">
                                        <td className="whitespace-nowrap px-6 py-4 text-sm flex gap-4 items-center font-medium text-gray-900">
                                            <p>{c?._id?.toString()}</p>
                                        </td>

                                        <td className="whitespace-nowrap px-6 py-4 text-sm font-medium text-gray-900">
                                            {c?.channel_name}
                                        </td>

                                        <td className="whitespace-nowrap px-6 py-4 text-sm font-medium text-gray-900">
                                            {c?.channel_id ?? 'N/A'}
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
