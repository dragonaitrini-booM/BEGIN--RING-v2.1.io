from dataclasses import dataclass
from datetime import datetime

@dataclass(frozen=True)
class AkashicRecord:
    hash: str
    phase: int
    timestamp: datetime
    theta_coherence: float
