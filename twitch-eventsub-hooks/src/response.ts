import { OutgoingHttpHeaders, ServerResponse } from 'http';
import {
    Request,
    TwitchNotificationChallenge,
    TwitchNotificationStreamOffline,
    TwitchNotificationStreamOnline,
} from './types';
import {
    HMAC_PREFIX,
    MESSAGE_SECRET,
    MESSAGE_TYPE,
    MESSAGE_TYPE_NOTIFICATION,
    MESSAGE_TYPE_REVOCATION,
    MESSAGE_TYPE_VERIFICATION,
    TWITCH_MESSAGE_SIGNATURE,
} from './config';
import { queueMessage } from './queue';
import { getHmacMessage, getHmac, verifyMessage } from './verification';

function sendResponse(res: ServerResponse, statusCode: number, headers?: OutgoingHttpHeaders, content?: string) {
    res.writeHead(statusCode, headers).end(content);
}

async function handleEvent(
    req: Request<TwitchNotificationStreamOnline | TwitchNotificationStreamOffline>,
    res: ServerResponse
) {
    sendResponse(res, 204);

    switch (req?.body?.subscription?.type) {
        case 'stream.online':
        case 'stream.offline':
            queueMessage(req?.body?.subscription?.type, req?.body?.event?.broadcaster_user_login);
            break;

        default:
            console.log('Unknown subscription type', req.body?.subscription?.type);
            break;
    }
}

export async function handleRequest(req: Request, res: ServerResponse) {
    const message = getHmacMessage(req?.headers, req?.body_str);

    if (!MESSAGE_SECRET) {
        sendResponse(res, 500);
        return;
    }

    const hmac = HMAC_PREFIX + getHmac(MESSAGE_SECRET, message);

    const validMessage = verifyMessage(hmac, req.headers[TWITCH_MESSAGE_SIGNATURE]?.toString() ?? '');

    if (!validMessage) {
        sendResponse(res, 403);
        return;
    }

    // Challenge
    if (MESSAGE_TYPE_VERIFICATION === req.headers[MESSAGE_TYPE]) {
        sendResponse(res, 200, { 'content-type': 'text/plain' }, (req?.body as TwitchNotificationChallenge)?.challenge);
        return;
    }

    if (MESSAGE_TYPE_REVOCATION === req.headers[MESSAGE_TYPE]) {
        sendResponse(res, 204);
        return;
    }

    if (MESSAGE_TYPE_NOTIFICATION === req.headers[MESSAGE_TYPE]) {
        await handleEvent(req as Request<TwitchNotificationStreamOnline | TwitchNotificationStreamOffline>, res);
        return;
    }

    // Unknown message

    sendResponse(res, 204);
}
