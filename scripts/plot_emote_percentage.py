import os
import sys
from pathlib import Path
from typing import TypedDict

import matplotlib.pyplot
from dotenv import load_dotenv
from pymongo import MongoClient
from pymongo.collection import Collection
from pymongo.command_cursor import CommandCursor

from models import TwitchVod

load_dotenv()


db_client = MongoClient(os.environ["MONGO_CONNECTION_URI"]).highlights


def get_vod(stream_id: str) -> TwitchVod | None:
    twitch_vod_collection: Collection[TwitchVod] = db_client.get_collection(
        "twitch_vods"
    )

    return twitch_vod_collection.find_one(filter={"stream_id": stream_id})


class GetTokenAggregateResult(TypedDict):
    message: str


def get_messages(vod: TwitchVod) -> list[GetTokenAggregateResult]:
    pipeline = [
        {
            "$match": {
                "channel": vod["channel_name"],
                "timestamp": {"$gte": vod["streamed_at"], "$lte": vod["ended_at"]},
            }
        },
        {"$project": {"_id": False, "message": "$message"}},
    ]

    cursor: CommandCursor[GetTokenAggregateResult] = db_client.get_collection(
        "twitch_messages"
    ).aggregate(pipeline=pipeline)

    return list(cursor)


def get_emotes():
    e: list[str] = db_client.get_collection("emotes").distinct("name")

    return set(e)


def calculate_emote_count(messages: list[GetTokenAggregateResult]) -> dict[str, int]:
    emotes = get_emotes()

    emote_count: dict[str, int] = {}

    for item in messages:
        for token in item["message"].strip().split(" "):
            if len(token) == 0 or token not in emotes:
                continue

            if token in emote_count:
                emote_count[token] += 1
            else:
                emote_count[token] = 1

    return emote_count


def plot_emotes(
    emote_count: dict[str, int],
    channel_name: str,
    stream_id: str,
    minimum_percentage=1,
):
    x = []
    y = []

    total = 0

    for (_, v) in emote_count.items():
        total += v

    dataset: list[tuple[str, float]] = list(
        (k, (v / total) * 100)
        for (
            k,
            v,
        ) in emote_count.items()
        if (v / total) * 100 >= minimum_percentage
    )

    dataset.sort(key=lambda x: x[1], reverse=True)

    dataset_total = 0

    for i in range(len(dataset)):
        (e, c) = dataset[i]

        x.append(e)
        y.append(c)

        dataset_total += c

    if dataset_total < total:
        x.append("other")
        y.append((dataset_total / total) * 100)

    fig, ax = matplotlib.pyplot.subplots()

    ax.barh(x, y)

    ax.grid(visible=True)

    ax.set_xlabel("percentage")

    ax.set_title(
        label=f"{channel_name} - stream id {stream_id}", loc="left", fontsize=10
    )

    ax.set_title(label="emote occurrence percentage", loc="center")

    ax.set_title(label=f"minimum {minimum_percentage}%", loc="right", fontsize=10)

    (w, h) = fig.get_size_inches()

    fig.set_size_inches(w * 2, h * 2)

    fig.align_xlabels()
    fig.tight_layout()

    folder_path = os.path.realpath(
        os.path.join(os.path.dirname(__file__), "..", "graphs")
    )

    Path(folder_path).mkdir(parents=True, exist_ok=True)

    fig.savefig(
        f"{folder_path}/emote-occurence-{channel_name}-{stream_id}.jpg", dpi=100
    )


if __name__ == "__main__":
    if len(sys.argv) == 1:
        print("Error: Missing stream_id argument!")
        sys.exit(2)

    stream_id = sys.argv[1]

    vod = get_vod(stream_id)

    if vod is None:
        print(f"Error: no vod with stream_id {stream_id} found!")
        sys.exit(1)

    messages = get_messages(vod)

    emote_count = calculate_emote_count(messages)

    plot_emotes(emote_count, vod["channel_name"], stream_id)
