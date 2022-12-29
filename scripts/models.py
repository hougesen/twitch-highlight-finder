from datetime import datetime
from typing import TypedDict


class TwitchVod(TypedDict):
    _id: str
    vod_id: str
    stream_id: str
    user_id: str
    channel_name: str
    language: str
    title: str
    url: str
    streamed_at: datetime
    ended_at: datetime
    video_duration: int
    analyzed: bool
