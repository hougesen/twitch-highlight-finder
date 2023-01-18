import 'dotenv/config';

import http from 'http';

import { handleRequest } from './response';
import { Request, TwitchNotification } from './types';

function parseRequestBody(bodyStr: string): TwitchNotification | undefined {
    try {
        return JSON.parse(bodyStr);
    } catch {
        return undefined;
    }
}

async function handleIncomingRequest(req: Request, res: http.ServerResponse) {
    const chunks: Uint8Array[] = [];

    req.on('data', (chunk: Uint8Array) => {
        chunks.push(chunk);
    }).on('end', () => {
        req.body_str = Buffer.concat(chunks)?.toString() ?? '';
        req.body = parseRequestBody(req?.body_str);

        handleRequest(req, res);
    });
}

const server = http.createServer();

server.on('request', handleIncomingRequest);

const PORT = process.env.PORT ?? 1234;

server.listen(PORT);

server.on('listening', () => console.info(`Listening on :${PORT}`));
