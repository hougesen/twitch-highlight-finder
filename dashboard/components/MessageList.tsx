import { ITwitchChatMessage } from '../types/models';

// TODO: Decide if i should make a standalone Table component
export function MessageList({ messages }: { messages: ITwitchChatMessage[] }) {
    return (
        <div className="flex flex-col">
            <div className="overflow-x-auto">
                <div className="inline-block min-w-full py-2">
                    <div className="overflow-hidden">
                        <table className="min-w-full">
                            <thead className="border-b">
                                <tr>
                                    <th className="px-6 py-4 text-left text-sm font-medium text-gray-900" scope="col">
                                        Timestamp
                                    </th>

                                    <th className="px-6 py-4 text-left text-sm font-medium text-gray-900" scope="col">
                                        Sender
                                    </th>

                                    <th className="px-6 py-4 text-left text-sm font-medium text-gray-900" scope="col">
                                        Message
                                    </th>
                                </tr>
                            </thead>

                            <tbody>
                                {messages.map((c, i) => (
                                    <tr key={i} className="border-b bg-white" data-id={c._id}>
                                        <td className="whitespace-nowrap px-6 py-4 text-sm font-medium text-gray-900">
                                            <p>{c?.timestamp ? new Date(c?.timestamp).toISOString() : 'N/A'}</p>
                                        </td>

                                        <td className="whitespace-nowrap px-6 py-4 text-sm font-medium text-gray-900">
                                            {c?.sender ?? ''}
                                        </td>

                                        <td className="whitespace-nowrap px-6 py-4 text-sm font-medium text-gray-900">
                                            {c?.message ?? ''}
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
