import Link from 'next/link';

export default function TopNavigation() {
    return (
        <div className="flex bg-gray-200 mt-4 gap-8 w-full rounded-lg p-4">
            <Link href={'/'} className="text-xl">
                Channels
            </Link>

            <Link href={'/emotes'} className="text-xl">
                Emotes
            </Link>
        </div>
    );
}
