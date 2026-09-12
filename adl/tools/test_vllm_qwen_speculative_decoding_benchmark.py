#!/usr/bin/env python3
"""PVF: deterministic local CPU accounting/correctness negatives; required #905
contract proof, no engine import, model load, hardware or Runtime execution.
"""
import unittest
import io
import json
import tempfile
from pathlib import Path
from unittest.mock import patch, Mock
from types import SimpleNamespace

import vllm_qwen_speculative_decoding_benchmark as bench


class AccountingTests(unittest.TestCase):
    def test_raw_engine_request_does_not_claim_speculative_activation(self):
        scratch = Path(__file__).resolve().parents[2] / '.adl/runs/905'
        scratch.mkdir(parents=True, exist_ok=True)
        llm = Mock()
        llm.generate.return_value = [SimpleNamespace(outputs=[SimpleNamespace(token_ids=[1, 2])])]
        llm.get_metrics.return_value = []
        engine = SimpleNamespace(__version__='fixture', LLM=Mock(return_value=llm), SamplingParams=lambda **kw: kw)
        torch = SimpleNamespace(__version__='fixture', cuda=SimpleNamespace(is_available=lambda: False))
        with tempfile.TemporaryDirectory(dir=scratch) as directory:
            output = Path(directory) / 'result.json'
            with patch('sys.argv', ['bench', '--mode', 'speculative', '--out', str(output)]), \
                    patch.dict('sys.modules', {'torch': torch, 'vllm': engine}), patch('sys.stdout', io.StringIO()):
                self.assertEqual(bench.main(), 0)
            result = json.loads(output.read_text())
            self.assertTrue(result['claims']['speculative_mode_requested'])
            self.assertFalse(result['claims']['proves_vllm_speculative_mode'])
            self.assertFalse(result['claims']['proves_current_runtime_route'])
            self.assertIsNone(result['runtime']['container_image'])
            self.assertFalse(result['provenance']['model_revisions_verified'])
            self.assertEqual(result['summary']['runs'], 6)
            self.assertTrue(all(len(r['output_sha256']) == 64 for r in result['runs']))

    def test_missing_dependency_retains_failed_setup(self):
        scratch = Path(__file__).resolve().parents[2] / '.adl/runs/905'
        scratch.mkdir(parents=True, exist_ok=True)
        with tempfile.TemporaryDirectory(dir=scratch) as directory:
            output = Path(directory) / 'result.json'
            with patch('sys.argv', ['bench', '--mode', 'target_only', '--out', str(output)]), \
                    patch.dict('sys.modules', {'torch': None}):
                with self.assertRaises(ModuleNotFoundError):
                    bench.main()
            records = [json.loads(line) for line in output.with_suffix('.json.attempts.jsonl').read_text().splitlines()]
            self.assertEqual([r['status'] for r in records], ['started', 'failed'])
            self.assertEqual(records[-1]['phase'], 'dependency_setup')
            self.assertEqual(output.read_text(), '')

    def test_existing_result_or_journal_refuses_before_engine_initialization(self):
        scratch = Path(__file__).resolve().parents[2] / '.adl/runs/905'
        scratch.mkdir(parents=True, exist_ok=True)
        with tempfile.TemporaryDirectory(dir=scratch) as directory:
            for name in ['result.json', 'result.json.attempts.jsonl']:
                output = Path(directory) / 'result.json'
                retained = Path(directory) / name
                retained.write_text('immutable prior evidence')
                engine = Mock()
                with patch('sys.argv', ['bench', '--mode', 'target_only', '--out', str(output)]), \
                        patch.dict('sys.modules', {'torch': Mock(), 'vllm': engine}):
                    with self.assertRaises(FileExistsError):
                        bench.main()
                engine.LLM.assert_not_called()
                self.assertEqual(retained.read_text(), 'immutable prior evidence')
                retained.unlink()

    def test_attempts_retain_failure_without_secret_error_text(self):
        journal = io.StringIO()
        def fail():
            raise RuntimeError('private-secret-prompt')
        with self.assertRaises(RuntimeError):
            bench.measured_attempt(fail, journal, phase='measured', prompt_index=0, repeat_index=0)
        self.assertNotIn('private-secret-prompt', journal.getvalue())
        records = [json.loads(line) for line in journal.getvalue().splitlines()]
        self.assertEqual([r['status'] for r in records], ['started', 'failed'])
        self.assertEqual(records[1]['error_class'], 'RuntimeError')
        value, elapsed = bench.measured_attempt(lambda: 42, journal, phase='initialization')
        self.assertEqual(value, 42)
        self.assertGreaterEqual(elapsed, 0)

    def run_record(self, elapsed=2.0, identity=None):
        return bench.RunResult(0, 0, elapsed, 4, 4 / elapsed,
                               identity or bench.output_identity([1, 2, 3, 4]))

    def test_empty_and_duplicate_denominators_rejected(self):
        for runs in [[], [self.run_record(), self.run_record()]]:
            with self.assertRaises(ValueError):
                bench.summarize(runs)

    def test_bad_metrics_rejected(self):
        for value in [-1, float('nan'), float('inf'), 1.5, True, '3']:
            with self.subTest(value=value), self.assertRaises(ValueError):
                bench.nonnegative_counter(value)

    def test_invalid_duration_and_contradictory_throughput(self):
        for elapsed, rate in [(0, 0), (-1, 4), (float('inf'), 0), (1, float('nan')), (1, 2)]:
            with self.subTest(elapsed=elapsed, rate=rate), self.assertRaises(ValueError):
                bench.summarize([bench.RunResult(0, 0, elapsed, 4, rate)])

    def test_counter_reset_and_shape_change_rejected(self):
        with self.assertRaises(ValueError):
            bench.subtract_optional_int(3, 4)
        with self.assertRaises(ValueError):
            bench.subtract_optional_vector([1], [0, 0])
        self.assertIsNone(bench.subtract_optional_int(None, 0))

    def test_acceptance_cannot_exceed_proposal(self):
        metrics = [SimpleNamespace(name='vllm:spec_decode_num_draft_tokens', value=2),
                   SimpleNamespace(name='vllm:spec_decode_num_accepted_tokens', value=3)]
        with self.assertRaises(ValueError):
            bench.extract_spec_metrics(metrics)

    def test_both_censored_and_duplicate_metrics_rejected(self):
        with self.assertRaises(ValueError):
            bench.compare_runs([self.run_record()], [self.run_record()], prompt_count=2, repeats=1)
        metric = SimpleNamespace(name='vllm:spec_decode_num_drafts', value=1)
        with self.assertRaises(ValueError):
            bench.extract_spec_metrics([metric, metric])

    def test_output_identity_nonempty_and_stable(self):
        self.assertEqual(bench.output_identity([1, 2]), bench.output_identity([1, 2]))
        self.assertNotEqual(bench.output_identity([1, 2]), bench.output_identity([2, 1]))
        with self.assertRaises(ValueError):
            bench.output_identity([])

    def test_paired_correctness_and_null_or_negative_benefit(self):
        for elapsed, ratio in [(2.0, 1.0), (4.0, .5), (1.0, 2.0)]:
            result = bench.compare_runs([self.run_record()], [self.run_record(elapsed)], prompt_count=1, repeats=1)
            self.assertEqual(result['wall_time_speedup'], ratio)
            self.assertFalse(result['proves_current_runtime_route'])

    def test_mismatch_missing_identity_and_censored_run_rejected(self):
        good = self.run_record()
        bad = self.run_record(identity='f' * 64)
        missing = bench.RunResult(0, 0, 2, 4, 2)
        other = self.run_record()
        other.prompt_index = 1
        for record in [bad, missing, other]:
            with self.subTest(record=record), self.assertRaises(ValueError):
                bench.compare_runs([good], [record], prompt_count=1, repeats=1)


if __name__ == '__main__':
    unittest.main()
