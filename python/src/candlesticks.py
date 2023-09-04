from datetime import datetime

from pydantic import BaseModel


class BinanceCandlestick(BaseModel):
    t: datetime  # Kline start time
    T: datetime  # Kline close time
    s: str  # Symbol
    i: str  # Interval
    f: int  # First trade ID
    L: int  # Last trade ID
    o: str  # Open price
    c: str  # Close price
    h: str  # High price
    l: str  # Low price
    v: str  # Base asset volume
    n: int  # Number of trades
    x: bool  # Is this kline closed?
    q: str  # Quote asset volume
    V: str  # Taker buy base asset volume
    Q: str  # Taker buy quote asset volume
    B: str  # Ignore


class BinanceCandlestickData(BaseModel):
    e: str  # Event type
    E: int  # Event time
    s: str  # Symbol
    k: BinanceCandlestick


class BitfinexCandlestick(BaseModel):
    mts: datetime
    open: float
    close: float
    high: float
    low: float
    volume: float


class BitfinexCandlestickUpdateData(BaseModel):
    channel_id: int
    data: BitfinexCandlestick


class BitfinexCandlestickSnapshotData(BaseModel):
    channel_id: int
    data: list[BitfinexCandlestick]
