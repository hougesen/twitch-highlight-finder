import { IncomingMessage } from 'http';

export declare type TwitchNotificationStreamOnline = {
    subscription: {
        id: string;
        status: string;
        type: 'stream.online';
        version: string;
        condition: {
            broadcaster_user_id: string;
        };
        transport: {
            method: string;
            callback: string;
        };
        created_at: string;
        cost: number;
    };
    event: {
        id: string;
        broadcaster_user_id: string;
        broadcaster_user_login: string;
        broadcaster_user_name: string;
        type: string;
        started_at: string;
    };
};

export declare type TwitchNotificationStreamOffline = {
    subscription: {
        id: string;
        status: 'stream.offline';
        type: string;
        version: string;
        condition: {
            broadcaster_user_id: string;
        };
        transport: {
            method: string;
            callback: string;
        };
        created_at: string;
        cost: number;
    };
    event: {
        broadcaster_user_id: string;
        broadcaster_user_login: string;
        broadcaster_user_name: string;
    };
};

export declare type TwitchNotificationChallenge = {
    challenge: string;
    subscription: {
        id: string;
        status: string;
        type: string;
        version: string;
        cost: number;
        condition: {
            broadcaster_user_id: string;
        };
        transport: {
            method: string;
            callback: string;
        };
        created_at: string;
    };
};

export declare type TwitchNotificationRevoked = {
    subscription: {
        id: string;
        status: string;
        type: string;
        cost: number;
        version: string;
        condition: {
            broadcaster_user_id?: string;
        };
        transport: {
            method: string;
            callback: string;
        };
        created_at: string;
    };
};

export declare type TwitchNotification =
    | TwitchNotificationChallenge
    | TwitchNotificationRevoked
    | TwitchNotificationStreamOnline
    | TwitchNotificationStreamOffline;

export declare interface Request<Body = unknown> extends IncomingMessage {
    body_str?: string;
    body?: Body;
}
