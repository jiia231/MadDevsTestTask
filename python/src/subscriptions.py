import asyncio
import json

import websockets

from candlesticks import (
    BitfinexCandlestick,
    BitfinexCandlestickSnapshotData,
    BinanceCandlestickData,
    BitfinexCandlestickUpdateData,
)


async def subscribe_binance_candles(queue: asyncio.Queue):
    uri = "wss://stream.binance.com:9443/ws/btcusdt@kline_1m"
    async with websockets.connect(uri) as websocket:
        while True:
            msg: str = await websocket.recv()
            candle_obj = BinanceCandlestickData.model_validate_json(msg)
            await queue.put(candle_obj.k)


async def subscribe_bitfinex_candles(queue: asyncio.Queue):
    uri = "wss://api-pub.bitfinex.com/ws/2"
    async with websockets.connect(uri) as websocket:
        connection_params = (
            '{"event":"subscribe","channel":"candles","key":"trade:1m:tBTCUSD"}'
        )
        await websocket.send(connection_params)
        _ = await websocket.recv()
        _ = await websocket.recv()

        msg: str = await websocket.recv()

        obj = json.loads(msg)
        candles = BitfinexCandlestickSnapshotData(
            channel_id=obj[0],
            data=[
                BitfinexCandlestick(
                    mts=x[0],
                    open=x[1],
                    close=x[2],
                    high=x[3],
                    low=x[4],
                    volume=x[5],
                )
                for x in obj[1]
            ],
        )
        for candle in candles.data:
            await queue.put(candle)

        while True:
            msg: str = await websocket.recv()
            if "hb" in msg:
                continue
            obj = json.loads(msg)
            candle_obj = BitfinexCandlestickUpdateData(
                channel_id=obj[0],
                data=BitfinexCandlestick(
                    mts=obj[1][0],
                    open=obj[1][1],
                    close=obj[1][2],
                    high=obj[1][3],
                    low=obj[1][4],
                    volume=obj[1][5],
                ),
            )
            await queue.put(candle_obj)
