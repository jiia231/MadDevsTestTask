#!/usr/bin/env python3

import asyncio

from calculations import RsiCalculator, VwapCalculator
from candlesticks import BinanceCandlestick, BitfinexCandlestick
from subscriptions import subscribe_bitfinex_candles, subscribe_binance_candles


async def main():
    binance_queue = asyncio.Queue(1024)
    bitfinex_queue = asyncio.Queue(1024)

    rsi_calculator = RsiCalculator(period=14)
    vwap_calculator = VwapCalculator()

    asyncio.create_task(subscribe_binance_candles(binance_queue))
    asyncio.create_task(subscribe_bitfinex_candles(bitfinex_queue))

    while True:
        if not binance_queue.empty():
            binance_candle: BinanceCandlestick = binance_queue.get_nowait()
            # print(binance_candle)
            if binance_candle.x:  # if it is closing record
                rsi = rsi_calculator.update(float(binance_candle.c))
                if rsi is not None:
                    print(f"Binance\tClose Price: {float(binance_candle.c)}\tRSI: {rsi}")
        if not bitfinex_queue.empty():
            bitfinex_candle: BitfinexCandlestick = bitfinex_queue.get_nowait()
            # print(bitfinex_candle)
            vwap = vwap_calculator.update(
                (bitfinex_candle.close + bitfinex_candle.open) / 2,
                bitfinex_candle.volume,
            )
            print(f"Bitfinex\tClose Price: {bitfinex_candle.close}\tVWAP: {vwap}")
        await asyncio.sleep(0.1)


if __name__ == "__main__":
    try:
        asyncio.run(main())
    except KeyboardInterrupt:
        print("Exiting...")
