from dataclasses import dataclass
from typing import Final, Optional, Any
import hashlib
from datetime import datetime

@dataclass(frozen=True)  # PEP 557: immutable by construction
class AkashicCache:
    """Immutable cache entry; global state is verboten."""

    session_id: str
    last_blockhash: str
    theta_power: float  # [0.0, 1.0] - trusted input from Rust VAD
    is_coherent: bool   # Derived: theta_power >= THETA_THRESHOLD

    THETA_THRESHOLD: Final[float] = 0.7  # Mathematical constant

    @classmethod
    def from_ipc_payload(cls, payload: dict[str, Any]) -> "AkashicCache":
        """Pure function: dict -> immutable cache."""
        theta = float(payload["theta_power"])
        return cls(
            session_id=str(payload["session_id"]),
            last_blockhash=str(payload["blockhash"]),
            theta_power=theta,
            is_coherent=(theta >= cls.THETA_THRESHOLD),
        )

    def compute_record_hash(self) -> str:
        """BLAKE3-like hash of immutable state."""
        data = f"{self.session_id}{self.last_blockhash}{self.theta_power:.6f}"
        # Using sha256 as placeholder for blake3 if strict dependency not enforced or to keep stdlib
        return hashlib.sha256(data.encode()).hexdigest()
