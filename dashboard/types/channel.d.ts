import { Document, WithId } from 'mongodb';

export declare interface IChannel extends WithId<Document> {
    channel_name: String;
}
