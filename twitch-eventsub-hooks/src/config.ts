import 'dotenv/config';

export const MESSAGE_SECRET = process.env.MESSAGE_SECRET;

if (!MESSAGE_SECRET) {
    throw new Error('Missing message secret');
}

export const HMAC_PREFIX = 'sha256=';

// Notification request headers
export const TWITCH_MESSAGE_ID = 'twitch-eventsub-message-id';
export const TWITCH_MESSAGE_TIMESTAMP = 'twitch-eventsub-message-timestamp';
export const TWITCH_MESSAGE_SIGNATURE = 'twitch-eventsub-message-signature';
export const MESSAGE_TYPE = 'twitch-eventsub-message-type';

// Notification message types
export const MESSAGE_TYPE_VERIFICATION = 'webhook_callback_verification';
export const MESSAGE_TYPE_NOTIFICATION = 'notification';
export const MESSAGE_TYPE_REVOCATION = 'revocation';
