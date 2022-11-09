import { Document, WithId } from 'mongodb';

export declare interface IChannel extends WithId<Document> {
    channel_name: String;
}

export declare interface IEmote extends WithId<Document> {
    emote_id: string;
    name: string;
    broadcaster_id: string;
}
