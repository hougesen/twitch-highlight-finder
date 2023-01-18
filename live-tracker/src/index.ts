import { Context, APIGatewayProxyResult, APIGatewayEvent } from 'aws-lambda';
import {
    MESSAGE_SECRET,
    HMAC_PREFIX,
    TWITCH_MESSAGE_SIGNATURE,
    MESSAGE_TYPE_VERIFICATION,
    MESSAGE_TYPE,
    MESSAGE_TYPE_REVOCATION,
    MESSAGE_TYPE_NOTIFICATION,
} from './config';
import { queueMessage } from './queue';
import { TwitchNotificationChallenge, TwitchNotificationStreamOnline, TwitchNotificationStreamOffline } from './types';
import { getHmacMessage, getHmac, verifyMessage } from './verification';

export const handler = async (event: APIGatewayEvent, context: Context): Promise<APIGatewayProxyResult> => {
    console.log(`Event: ${JSON.stringify(event, null, 2)}`);

    console.log(`Context: ${JSON.stringify(context, null, 2)}`);

    try {
        const message = getHmacMessage(event?.headers, event?.body);

        if (!MESSAGE_SECRET) {
            return { statusCode: 500, body: '' };
        }

        const hmac = HMAC_PREFIX + getHmac(MESSAGE_SECRET, message);

        const validMessage = verifyMessage(hmac, event?.headers?.[TWITCH_MESSAGE_SIGNATURE] ?? '');

        if (!validMessage) {
            return { statusCode: 403, body: '' };
        }

        if (MESSAGE_TYPE_REVOCATION === event?.headers?.[MESSAGE_TYPE]) {
            return { statusCode: 204, body: '' };
        }

        // Challenge
        if (MESSAGE_TYPE_VERIFICATION === event?.headers?.[MESSAGE_TYPE]) {
            return {
                statusCode: 200,
                body: (JSON.parse(event?.body ?? '{}') as TwitchNotificationChallenge)?.challenge,
            };
        }

        if (MESSAGE_TYPE_NOTIFICATION === event?.headers?.[MESSAGE_TYPE]) {
            if (event?.body) {
                const body = JSON.parse(event.body) as TwitchNotificationStreamOnline | TwitchNotificationStreamOffline;

                switch (body?.subscription?.type) {
                    case 'stream.online':
                    case 'stream.offline':
                        await queueMessage(body?.subscription?.type, body?.event?.broadcaster_user_login);
                        break;

                    default:
                        console.info('Unknown subscription type', body?.subscription?.type);
                        break;
                }
            }
        }

        return { statusCode: 204, body: '' };
    } catch (error) {
        return { statusCode: 400, body: JSON.stringify(error) };
    }
};
