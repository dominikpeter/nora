Implement a generic `Interval<T>` with fields `lower: T` and `upper: T`,
representing a half-open interval. Derive Debug and PartialEq. Implement
PartialOrd for T: PartialOrd. Equal endpoints mean Equal; if self.lower >=
other.upper return Greater; otherwise if self.upper <= other.lower return
Less; otherwise return None (overlap). Apply checks in that order.
Support integer and floating-point endpoints. Do not include tests or main.
