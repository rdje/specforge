#!/usr/bin/env python3
"""Focused contract tests for the reviewed-population replay orchestrator."""

from __future__ import annotations

import os
from pathlib import Path
import sys
import unittest
from unittest.mock import patch


sys.path.insert(0, str(Path(__file__).resolve().parent))
import replay_source_to_intent_population as replay  # noqa: E402


class SourceToIntentReplayCommandTest(unittest.TestCase):
    def test_replay_command_has_exactly_four_positionals_after_separator(self) -> None:
        environment = {
            key: f"test-{index}"
            for index, key in enumerate(replay.REPLAY_ENVIRONMENT_KEYS, 1)
        }
        with patch.dict(os.environ, environment, clear=True):
            command = replay.source_to_intent_replay_command(
                Path(".project-data/tmp/population/sources/example.pdf"),
                Path(".project-data/tmp/population/replays/example"),
            )

        separator = command.index("--")
        self.assertEqual(
            command[separator + 1 :],
            [
                ".project-data/tmp/population/sources/example.pdf",
                ".project-data/tmp/population/replays/example",
                "generated/prior_memory/corpus_memory.json",
                "-",
            ],
        )
        self.assertEqual(
            command[:separator],
            [
                "env",
                "DOCLING_DEVICE=test-1",
                "SPECFORGE_INGEST_BATCH_THRESHOLD=test-2",
                "SPECFORGE_INGEST_BATCH_PAGES=test-3",
                "cargo",
                "run",
                "--quiet",
                "-p",
                "specforge",
                "--example",
                "source_to_intent_replay",
            ],
        )


if __name__ == "__main__":
    unittest.main()
