import Image from 'next/image';

export default function EmoteImage({ emote_id, width }: { emote_id: string; width?: number }) {
    return (
        <Image
            src={`https://static-cdn.jtvnw.net/emoticons/v2/${emote_id}/default/light/3.0`}
            alt={emote_id}
            width={width ?? 112}
            height={width ?? 112}
        />
    );
}
