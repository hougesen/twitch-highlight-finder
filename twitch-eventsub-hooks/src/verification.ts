import crypto from 'crypto';
import { IncomingHttpHeaders } from 'http';

import { TWITCH_MESSAGE_ID, TWITCH_MESSAGE_TIMESTAMP } from './config';

export function getHmacMessage(headers: IncomingHttpHeaders, body?: string) {
    // @ts-expect-error allow undefined on purpose
    return headers[TWITCH_MESSAGE_ID] + headers[TWITCH_MESSAGE_TIMESTAMP] + body;
}

export function getHmac(secret: string, message: string) {
    return crypto.createHmac('sha256', secret).update(message).digest('hex');
}

export function verifyMessage(hmac: string, verifySignature: string) {
    return crypto.timingSafeEqual(Buffer.from(hmac), Buffer.from(verifySignature));
}
