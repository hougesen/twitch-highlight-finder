import os
import sys
from datetime import datetime
from pathlib import Path
from typing import TypedDict

from dotenv import load_dotenv
from matplotlib import pyplot
from pymongo import MongoClient
from pymongo.collection import Collection
from pymongo.command_cursor import CommandCursor

from models import TwitchVod

load_dotenv()


db_client = MongoClient(os.environ["MONGO_CONNECTION_URI"]).highlights


def get_vod(stream_id: str):
    twitch_vod_collection: Collection[TwitchVod] = db_client.get_collection(
        "twitch_vods"
    )

    return twitch_vod_collection.find_one(filter={"stream_id": stream_id})


class AggregateMessage(TypedDict):
    timestamp: datetime
    count: int


def get_messages_per_minute(vod: TwitchVod):
    pipeline = [
        {
            "$match": {
                "channel": vod["channel_name"],
                "timestamp": {"$gte": vod["streamed_at"], "$lte": vod["ended_at"]},
            }
        },
        {
            "$group": {
                "_id": {
                    "$dateToString": {
                        "format": "%Y-%m-%dT%H:%M",
                        "date": "$timestamp",
                    }
                },
                "count": {"$sum": 1},
            }
        },
        {
            "$addFields": {"timestamp": {"$dateFromString": {"dateString": "$_id"}}},
        },
        {"$unset": "_id"},
        {"$sort": {"timestamp": 1}},
    ]

    message_cursor: CommandCursor[AggregateMessage] = db_client.get_collection(
        "twitch_messages"
    ).aggregate(pipeline=pipeline)

    return list(message_cursor)


def plot_messages(
    messages: list[AggregateMessage],
    stream_start: datetime,
    channel_name: str,
    stream_id: str,
):
    x: list[int] = []
    y: list[int] = []

    stream_start_timestamp = stream_start.timestamp()
    for message in messages:
        x.append(int((message["timestamp"].timestamp() - stream_start_timestamp) / 60))
        y.append(message["count"])

    fig, ax = pyplot.subplots()

    ax.plot(x, y)

    ax.set_title(label="messages per minute", fontsize=10, loc="left")
    ax.set_title(
        label=f"{channel_name} - stream id {stream_id}", fontsize=10, loc="center"
    )

    ax.set_xlabel("minute")
    ax.set_ylabel("message count")

    (w, h) = fig.get_size_inches()

    fig.set_size_inches(w * 2, h * 2)

    fig.align_xlabels()
    fig.tight_layout()

    folder_path = os.path.realpath(
        os.path.join(os.path.dirname(__file__), "..", "graphs")
    )

    Path(folder_path).mkdir(parents=True, exist_ok=True)

    fig.savefig(f"{folder_path}/messages-per-minute-{channel_name}-{stream_id}.jpg")


if __name__ == "__main__":
    if len(sys.argv) == 1:
        print("Error: Missing stream_id argument!")
        sys.exit(2)

    stream_id = sys.argv[1]

    vod = get_vod(stream_id)

    if vod is None:
        print(f"Error: no vod with stream_id {stream_id} found!")
        sys.exit(1)

    messages = get_messages_per_minute(vod)

    plot_messages(messages, vod["streamed_at"], vod["channel_name"], stream_id)
