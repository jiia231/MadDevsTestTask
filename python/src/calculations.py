class RsiCalculator:
    def __init__(self, period: int):
        self.period = period
        self.close_prices = []

    def update(self, close_price: float):
        self.close_prices.append(close_price)

        if len(self.close_prices) >= self.period:
            gain_loss_values = self.calculate_gain_loss_values()
            average_gain = self.calculate_average(gain_loss_values["gains"])
            average_loss = self.calculate_average(gain_loss_values["losses"])

            if average_loss != 0.0:
                relative_strength = average_gain / average_loss
                rsi = 100.0 - (100.0 / (1.0 + relative_strength))
                return rsi
            else:
                return 100.0
        else:
            return None

    def calculate_gain_loss_values(self):
        gains = []
        losses = []

        for i in range(1, len(self.close_prices)):
            price_diff = self.close_prices[i] - self.close_prices[i - 1]
            if price_diff >= 0.0:
                gains.append(price_diff)
                losses.append(0.0)
            else:
                gains.append(0.0)
                losses.append(abs(price_diff))

        return {"gains": gains, "losses": losses}

    def calculate_average(self, values: list[float]):
        return sum(values) / len(values) if len(values) > 0 else 0.0


class VwapCalculator:
    def __init__(self):
        self.total_volume = 0.0
        self.cumulative_price_volume = 0.0

    def update(self, price: float, volume: float):
        self.cumulative_price_volume += price * volume
        self.total_volume += volume

        if self.total_volume > 0.0:
            return self.cumulative_price_volume / self.total_volume
        else:
            return 0.0
