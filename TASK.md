# Test task

1) Подключиться к websocket api binance (Kline/Candlestick Stream). 
    1) валютная пара = BTCUSDT
    2) timeframe 5m
    3) Расчитать Relative Strength Index для BTCUSDT используя свечи 5минут (источник = close, length = 14)

2) Подключиться к websocket api bitfinex
    1) валютная пара = BTCUSDT(tBTCUSD)
    2) timeframe = 1m 
    3) Расчитать VWAP используя данные из свечей 1m (1 минута)

3) Отображать в цикле значения: 
    * Binance: Close price и RSI 
    * Bitfinex:Close price, VWAP 
    * Close price и другие значения отображать только на полностью сформированных (закрытых) свечах.

Примечание к П #3. Из за того что закрытые свечи будут приходить в разное время (binance: 5m BTCUSDT, Bitfinex:1m BTCUSDT) т.к. мы подписаны на разные таймфреймы(временные рамки) закрытые свечи(сформированные) будут приходить в разное время:
* 5m раз 5 минут например 15:00, 15:05, 15:10 
* 1m каждую минуту например в 15:00, 15.01, 15.02


Ссылки которы могут быть полезны при выполнении задачи:  
https://www.investopedia.com/terms/c/candlestick.asp - Что такое свечи (Kline/Candlestick)  
https://binance-docs.github.io/apidocs/spot/en/#websocket-market-streams - ссылка на док бинанс, внутри много статей можно найти различную инфу  
https://github.com/binance/binance-connector-python - одна изготовых библиотек для подключения к вебсокету, есть очень много других с похожим функционалом  
https://dev.binance.vision/ - тут можно задать вопрос по API, а лучше поискать по ответам  
https://www.investopedia.com/terms/r/rsi.asp - что такое Relative Strength Index  
https://www.investopedia.com/terms/v/vwap.asp - что такое VWAP  
https://github.com/twopirllc/pandas-ta - библиотека которая включает в себя выше перечисленные индикаторы (VWAP, RSI)  
https://docs.bitfinex.com/docs/introduction -  ссылка на док bitfinex, внутри много статей можно найти различную инфу  
https://github.com/bitfinexcom/bitfinex-api-py - одна из готовых библиотек для подключения к вебсокету Bitfinex  
https://github.com/websocket-client/websocket-client/ - websockets библиотека  

Лимит выполнения - особо нет. 

