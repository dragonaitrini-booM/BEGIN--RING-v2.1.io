import pytest
from scarab.dragon_v2.akashic_cache import AkashicCache
from scarab.dragon_v2.dragon_system import DragonSystem
from scarab.dragon_v2.exceptions import ConsciousnessVetoError

class TestConsciousnessVeto:
    """Mathematical validation of theta-coherence gate."""

    def test_veto_blocks_low_theta(self):
        """theta = 0.65 < 0.7 -> VETO"""
        cache = AkashicCache.from_ipc_payload({
            "session_id": "test_session",
            "blockhash": "0xabc123",
            "theta_power": 0.65
        })
        system = DragonSystem(cache)

        with pytest.raises(ConsciousnessVetoError) as exc:
            system.prepare_manifestation()

        assert exc.value.theta_power == 0.65
        assert "0.7" in str(exc.value)  # Threshold in message

    def test_passes_high_theta(self):
        """theta = 0.85 >= 0.7 -> MANIFEST"""
        cache = AkashicCache.from_ipc_payload({
            "session_id": "test_session",
            "blockhash": "0xabc123",
            "theta_power": 0.85
        })
        system = DragonSystem(cache)

        record = system.prepare_manifestation()
        assert record.theta_coherence == 0.85
        assert record.phase == 18000

    def test_immutability_enforced(self):
        """Frozen dataclass prevents mutation"""
        cache = AkashicCache.from_ipc_payload({
            "session_id": "test",
            "blockhash": "0x123",
            "theta_power": 0.8
        })

        with pytest.raises(AttributeError):
            cache.theta_power = 0.9  # Cannot mutate frozen dataclass
