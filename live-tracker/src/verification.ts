import { APIGatewayProxyEventHeaders } from 'aws-lambda';
import { createHmac, timingSafeEqual } from 'crypto';
import { TWITCH_MESSAGE_ID, TWITCH_MESSAGE_TIMESTAMP } from './config';

export function getHmacMessage(headers: APIGatewayProxyEventHeaders, body?: string | null) {
    const messageId = headers?.[TWITCH_MESSAGE_ID] ?? '';

    const messageTimestamp = headers?.[TWITCH_MESSAGE_TIMESTAMP] ?? '';

    return messageId + messageTimestamp + body;
}

export function getHmac(secret: string, message: string) {
    return createHmac('sha256', secret).update(message).digest('hex');
}

export function verifyMessage(hmac: string, verifySignature: string) {
    return timingSafeEqual(Buffer.from(hmac), Buffer.from(verifySignature));
}
