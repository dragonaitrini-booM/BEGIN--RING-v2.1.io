from typing import Final

class ConsciousnessVetoError(Exception):
    """Raised when theta-band coherence is insufficient for manifestation."""

    THETA_THRESHOLD: Final[float] = 0.7  # PEP 484: immutable constant

    def __init__(self, theta_power: float) -> None:
        self.theta_power: float = theta_power
        super().__init__(
            f"Consciousness Veto: theta-power ({theta_power:.3f}) < threshold ({self.THETA_THRESHOLD})"
        )
