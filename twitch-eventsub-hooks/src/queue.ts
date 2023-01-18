import { CreateQueueCommand, SendMessageCommand, SQSClient } from '@aws-sdk/client-sqs';
import { fromEnv } from '@aws-sdk/credential-providers';

const SQS_CLIENT = new SQSClient({ credentials: fromEnv() });

async function getQueueUrl() {
    return SQS_CLIENT.send(new CreateQueueCommand({ QueueName: 'live-tracker' })).then((q) => q?.QueueUrl);
}

export async function queueMessage(type: string, username: string) {
    const queueUrl = await getQueueUrl();

    await SQS_CLIENT.send(
        new SendMessageCommand({
            QueueUrl: queueUrl,
            MessageBody: JSON.stringify({ event: type, username }),
        })
    );
}
