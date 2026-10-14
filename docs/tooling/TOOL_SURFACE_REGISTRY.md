# ADL Source Tool Surface Registry

Historical baseline: issue #3734; its v0.91.5 snapshot remains unchanged.
Status: generated_live_registry

## Summary

This registry records the current visible ADL toolkit surface for the
selected source checkout. It is a live source inventory intended to
drive later simplification cuts without confusing primary workflow
entrypoints, supported compatibility shims, internal helpers, and
historical evidence artifacts.

This registry distinguishes active workflow spines from already-retired
wrapper residue so later cuts can stay evidence-bound.

## Generation Command

```bash
python3 adl/tools/generate_tool_surface_registry.py
```

## Freshness Check

```bash
python3 adl/tools/generate_tool_surface_registry.py --check
```

## Classification Counts

- `primary`: 7
- `supported shim`: 0
- `internal helper`: 261
- `historical evidence`: 21
- `archive candidate`: 0
- `remove candidate`: 0

## Locally Declared Binaries

| Surface | Kind | Classification | Notes |
| --- | --- | --- | --- |
| `adl` | owner binary | primary | Public ADL planning and installed-product CLI declared by this repository. |
| `adl-process` | owner binary | primary | Public permission-safe process-status helper declared by this repository. |

## Owner Navigation

| Surface | Kind | Classification | Notes |
| --- | --- | --- | --- |
| `https://github.com/agent-logic/agent-logic-infrastructure` | owner documentation | primary | Infrastructure manual proof owner; retained source and resolver checks do not qualify live behavior. |
| `https://github.com/agent-logic/cognitive-sdlc` | owner documentation | primary | C-SDLC v3 lifecycle owner. Use its installed operator manual and authenticated authority; source presence grants no authority. |
| `https://github.com/agent-logic/agent-logic-runtime` | owner documentation | primary | Runtime product owner; installation and product qualification are separate from this source inventory. |
| `https://github.com/agent-logic/codefriend` | owner documentation | primary | CodeFriend review product owner; this registry does not invoke or qualify it. |

## Tool Surfaces Under `adl/tools`

| Surface | Kind | Classification | Notes |
| --- | --- | --- | --- |
| `adl/tools/BURST_FINAL_SUMMARY_TEMPLATE.md` | tool surface | internal helper | Visible tool surface retained for bounded support work. |
| `adl/tools/README.md` | tool surface | internal helper | Visible tool surface retained for bounded support work. |
| `adl/tools/archive_run_artifacts.sh` | tool surface | internal helper | Shell helper retained for bounded workflow support. |
| `adl/tools/batched_checks.sh` | tool surface | primary | Local validation entrypoint; use the current declared public validation contract. |
| `adl/tools/build_issue509_runtime_bundle_on_relay.sh` | tool surface | internal helper | Shell helper retained for bounded workflow support. |
| `adl/tools/build_v0917_execution_outlier_analysis.py` | tool surface | internal helper | Python helper or generator used by a bounded tooling surface. |
| `adl/tools/check_no_new_legacy_swarm_refs.sh` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/check_no_tracked_adl_issue_record_residue.sh` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/check_release_notes_commands.sh` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/check_release_version_surfaces.sh` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/check_repo_quality_staleness.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/ci_path_policy.sh` | tool surface | internal helper | Shell helper retained for bounded workflow support. |
| `adl/tools/clean_coverage_artifacts.sh` | tool surface | internal helper | Shell helper retained for bounded workflow support. |
| `adl/tools/compute_ccc_v0.py` | tool surface | internal helper | Python helper or generator used by a bounded tooling surface. |
| `adl/tools/csdlc_obs_ready_canary.sh` | tool surface | internal helper | Shell helper retained for bounded workflow support. |
| `adl/tools/demo_d11_signed_remote.sh` | tool surface | historical evidence | Historical signed-remote contract; current public ADL does not supply its Runtime binary or test interfaces. |
| `adl/tools/demo_five_command_run.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v087_operational_skills.sh` | tool surface | historical evidence | Historical bundle-install and inventory contract; original recursive acceptance remains separate from owner resource proof. |
| `adl/tools/demo_v087_trace_truth.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v0891_arxiv_manuscript_workflow.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v090_architecture_document_generation.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v090_codebuddy_review_showcase.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v0911_multiagent_podcast_audio.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v0911_multiagent_podcast_pilot.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v0912_quality_gate.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v0912_workflow_guardrails_showcase.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v0913_first_proof_demo.py` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v0913_five_minute_sprint_console.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v0913_podcast_studio_v2.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v0913_quality_gate.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v0913_starharvest_html_game.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v0914_codex_only_complete_issue_workcell.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/demo_v0918_podcast_launch.sh` | tool surface | internal helper | Demo/proof helper rather than a primary workflow entrypoint. |
| `adl/tools/diagnose_browser_routes.py` | tool surface | internal helper | Python helper or generator used by a bounded tooling surface. |
| `adl/tools/editor_action.sh` | tool surface | internal helper | Shell helper retained for bounded workflow support. |
| `adl/tools/ensure_csm_binary.sh` | tool surface | internal helper | Shell helper retained for bounded workflow support. |
| `adl/tools/ensure_final_merge_gate.sh` | tool surface | internal helper | Shell helper retained for bounded workflow support. |
| `adl/tools/fill_planning_template.py` | tool surface | internal helper | Python helper or generator used by a bounded tooling surface. |
| `adl/tools/fix_git_main_sync.sh` | tool surface | internal helper | Shell helper retained for bounded workflow support. |
| `adl/tools/fixtures` | tool surface | internal helper | Directory containing bounded helper assets, scripts, or proof inputs. |
| `adl/tools/generate_active_command_reference_scan.py` | tool surface | internal helper | Python helper or generator used by a bounded tooling surface. |
| `adl/tools/generate_podcast_launch_packet.py` | tool surface | internal helper | Python helper or generator used by a bounded tooling surface. |
| `adl/tools/generate_podcast_studio_v2_packet.py` | tool surface | internal helper | Python helper or generator used by a bounded tooling surface. |
| `adl/tools/generate_tool_surface_registry.py` | tool surface | internal helper | Python helper or generator used by a bounded tooling surface. |
| `adl/tools/install_adl_operational_skills.sh` | tool surface | internal helper | Shell helper retained for bounded workflow support. |
| `adl/tools/install_aws_remote_validation_tool.sh` | tool surface | historical evidence | Historical Infrastructure source route retired by #1223; command refuses execution. Original assertions remain in Git and retirement custody; run the reviewed owner route. |
| `adl/tools/install_csdlc_man_pages.sh` | tool surface | internal helper | Shell helper retained for bounded workflow support. |
| `adl/tools/install_issue268_runtime_volume.py` | tool surface | internal helper | Python helper or generator used by a bounded tooling surface. |
| `adl/tools/install_owner_binaries.sh` | tool surface | internal helper | Shell helper retained for bounded workflow support. |
| `adl/tools/install_vector_component.ps1` | tool surface | internal helper | Visible tool surface retained for bounded support work. |
| `adl/tools/install_vector_component.sh` | tool surface | internal helper | Shell helper retained for bounded workflow support. |
| `adl/tools/issue345_aws_gpu_prerequisites.cloudformation.yaml` | tool surface | internal helper | Visible tool surface retained for bounded support work. |
| `adl/tools/issue607_guardian_recovery_proof.sh` | tool surface | internal helper | Shell helper retained for bounded workflow support. |
| `adl/tools/issue607_probe_runtime.py` | tool surface | internal helper | Python helper or generator used by a bounded tooling surface. |
| `adl/tools/issue607_qualify_warm_polis.sh` | tool surface | internal helper | Shell helper retained for bounded workflow support. |
| `adl/tools/lib` | tool surface | internal helper | Directory containing bounded helper assets, scripts, or proof inputs. |
| `adl/tools/mcp` | tool surface | internal helper | Directory containing bounded helper assets, scripts, or proof inputs. |
| `adl/tools/mock_ollama_v0_4.sh` | tool surface | internal helper | Mock helper used for focused testing or proof. |
| `adl/tools/owner_binary_resolution.sh` | tool surface | internal helper | Shell helper retained for bounded workflow support. |
| `adl/tools/polis_status_for_ssm.sh` | tool surface | internal helper | Shell helper retained for bounded workflow support. |
| `adl/tools/polis_status_for_ssm_qts.sh` | tool surface | internal helper | Shell helper retained for bounded workflow support. |
| `adl/tools/polis_status_for_ssm_windows.ps1` | tool surface | internal helper | Visible tool surface retained for bounded support work. |
| `adl/tools/preflight_review.sh` | tool surface | internal helper | Shell helper retained for bounded workflow support. |
| `adl/tools/rd01_repository_audit.py` | tool surface | internal helper | Python helper or generator used by a bounded tooling surface. |
| `adl/tools/real_chatgpt_gemini_claude_provider_adapter.py` | tool surface | internal helper | Python helper or generator used by a bounded tooling surface. |
| `adl/tools/record_podcast_browser_playback.mjs` | tool surface | internal helper | Visible tool surface retained for bounded support work. |
| `adl/tools/record_podcast_ios_safari_playback.sh` | tool surface | internal helper | Shell helper retained for bounded workflow support. |
| `adl/tools/record_podcast_native_playback.sh` | tool surface | internal helper | Shell helper retained for bounded workflow support. |
| `adl/tools/release` | tool surface | internal helper | Directory containing bounded helper assets, scripts, or proof inputs. |
| `adl/tools/release_ceremony.sh` | tool surface | internal helper | Shell helper retained for bounded workflow support. |
| `adl/tools/report_large_rust_modules.sh` | tool surface | internal helper | Shell helper retained for bounded workflow support. |
| `adl/tools/report_module_navigability.sh` | tool surface | internal helper | Shell helper retained for bounded workflow support. |
| `adl/tools/report_runtime_v3_loc.sh` | tool surface | internal helper | Shell helper retained for bounded workflow support. |
| `adl/tools/resolve_ci_backend.sh` | tool surface | internal helper | Shell helper retained for bounded workflow support. |
| `adl/tools/revision_validation_inventory.py` | tool surface | internal helper | Python helper or generator used by a bounded tooling surface. |
| `adl/tools/run_adl_v2_standalone.sh` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/run_authoritative_coverage_lane.sh` | tool surface | historical evidence | Historical Runtime source-tree route retired by #1223; invocation fails before execution. Original assertions are preserved in Git and retirement custody, not qualified as passing. |
| `adl/tools/run_aws_spot_builder_image_validation.sh` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/run_aws_spot_remote_validation_lane.sh` | tool surface | historical evidence | Historical Infrastructure source route retired by #1223; command refuses execution. Original assertions remain in Git and retirement custody; run the reviewed owner route. |
| `adl/tools/run_build_platform_benchmark.sh` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/run_cargo_validation.sh` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/run_ci_step_with_log.sh` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/run_issue268_remote_resident_qualification.sh` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/run_issue268_six_hour_spot_qualification.sh` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/run_issue509_gcp_drt_d_qualification.sh` | tool surface | historical evidence | Historical Infrastructure source route retired by #1223; command refuses execution. Original assertions remain in Git and retirement custody; run the reviewed owner route. |
| `adl/tools/run_issue607_warm_polis.sh` | tool surface | historical evidence | Historical Infrastructure source route retired by #1223; command refuses execution. Original assertions remain in Git and retirement custody; run the reviewed owner route. |
| `adl/tools/run_issue900_continuity_negatives.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/run_nessus_remote_validation.sh` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/run_pr_fast_test_lane.sh` | tool surface | historical evidence | Historical Runtime source-tree route retired by #1223; invocation fails before execution. Original assertions are preserved in Git and retirement custody, not qualified as passing. |
| `adl/tools/run_pvf_ci_release_policy_fixture.sh` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/run_pvf_validation_lane.sh` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/run_runtime_gateway_owner_contract.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/run_v0913_proof_validation_lane.sh` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/run_v0914_multi_agent_workcell_proof.sh` | tool surface | historical evidence | Historical model-backed v0.91.4 workcell rehearsal; retained manifest and review artifacts do not qualify a new execution, including its provider-active dry-run path. |
| `adl/tools/run_v0917_csm_api_gateway_bridge_live_proof.sh` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/run_validation_manager_nessus_lane.sh` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/rust_cache_env.sh` | tool surface | internal helper | Shell helper retained for bounded workflow support. |
| `adl/tools/rust_validation_warm_cache.sh` | tool surface | internal helper | Shell helper retained for bounded workflow support. |
| `adl/tools/select_validation_lanes.py` | tool surface | internal helper | Python helper or generator used by a bounded tooling surface. |
| `adl/tools/select_validation_lanes.sh` | tool surface | internal helper | Shell helper retained for bounded workflow support. |
| `adl/tools/setup_aws_spot_remote_validation_github_resources.sh` | tool surface | internal helper | Shell helper retained for bounded workflow support. |
| `adl/tools/suitability_specs` | tool surface | internal helper | Directory containing bounded helper assets, scripts, or proof inputs. |
| `adl/tools/test_archive_run_artifacts.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_arxiv_paper_writer_skill_contracts.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_batched_checks_no_codexpr_usage_banner.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_card_editor_repair_examples.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_card_prompt.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_card_reviewer_fixture.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_ccc_v0_instrumentation.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_check_issue_metadata_parity.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_check_milestone_closed_issue_sor_truth.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_check_no_tracked_adl_issue_record_residue.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_check_repo_quality_staleness.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_ci_path_policy.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_ci_runtime_contracts.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_cli_owner_command_guidance.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_cli_wrapper_migration_contract.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_closeout_completed_issue_wave.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_codefriend_fitness_ci_artifacts.py` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_completed_output_validation.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_csdlc_demo_proof_contract_packet.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_csdlc_prompt_editor.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_csdlc_v2_supply_chain.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_demo_operator_skill_contracts.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_demo_v0891_arxiv_manuscript_workflow.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_demo_v0902_arxiv_writer_field_test.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_demo_v090_architecture_document_generation.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_demo_v090_codebuddy_review_showcase.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_demo_v0912_quality_gate.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_demo_v0913_quality_gate.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_demo_v0914_codex_only_complete_issue_workcell.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_developer_throughput_fast_lane.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_documentation_specialist_skill_contracts.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_editor_action.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_ensure_csm_binary.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_ensure_final_merge_gate.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_evidence_bundle_packet.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_first_proof_demo_packet.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_first_proof_readiness_packet.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_five_minute_html_game_packet.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_five_minute_sprint_console_packet.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_gap_analysis_skill_contracts.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_generate_active_command_reference_scan.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_generate_tool_surface_registry.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_html_observatory.sh` | tool surface | historical evidence | Historical Runtime source-tree route retired by #1223; invocation fails before execution. Original assertions are preserved in Git and retirement custody, not qualified as passing. |
| `adl/tools/test_install_adl_operational_skills.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_install_issue268_runtime_volume.py` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_issue_folding_skill_contracts.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_issue_splitter_skill_contracts.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_issue_watcher_skill_contracts.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_merge_readiness_packet.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_milestone_dashboard.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_module_navigability_report.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_multi_agent_repo_review_skill_suite_contracts.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_multi_agent_repo_review_specialist_skill_contracts.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_multiagent_podcast_audio_policy.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_obsmem_handoff_packet.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_owner_binary_install.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_podcast_launch_packet.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_podcast_studio_v2_packet.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_portable_contract_normalizer_skill_contracts.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_product_report_writer_skill_contracts.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_prompt_spec_lint.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_prompt_template_structure_schemas.py` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_pvf_ci_release_policy.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_readiness_prep_metrics_non_terminal.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_records_hygiene_skill_contracts.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_refactoring_helper_skill_contracts.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_release_ceremony.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_release_evidence_skill_contracts.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_repo_code_review_skill_contracts.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_retained_diff_proof_contract.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_review_card_surface.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_review_comment_triage_skill_contracts.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_review_output_provenance.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_review_quality_evaluator_skill_contracts.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_review_subagent_auth_policy.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_revision_validation_inventory.py` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_run_authoritative_coverage_lane.sh` | tool surface | historical evidence | Historical Runtime source-tree route retired by #1223; invocation fails before execution. Original assertions are preserved in Git and retirement custody, not qualified as passing. |
| `adl/tools/test_run_aws_spot_builder_image_validation.sh` | tool surface | historical evidence | Historical Infrastructure source route retired by #1223; command refuses execution. Original assertions remain in Git and retirement custody; run the reviewed owner route. |
| `adl/tools/test_run_aws_spot_ci_profile.sh` | tool surface | historical evidence | Historical Infrastructure source route retired by #1223; command refuses execution. Original assertions remain in Git and retirement custody; run the reviewed owner route. |
| `adl/tools/test_run_cargo_validation.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_run_ci_step_with_log.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_run_issue268_remote_resident_qualification.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_run_nessus_remote_validation.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_run_pvf_validation_lane.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_run_v0913_proof_validation_lane.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_run_validation_manager_nessus_lane.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_rust_cache_env.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_rust_validation_warm_cache.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_software_development_polis_packet.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_structured_prompt_validation.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_test_generator_skill_contracts.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_transition_dag_packet.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_use_case_writer_skill_contracts.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_v0915_multi_agent_quality_comparison.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_v0915_openrouter_matrix.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_v0915_remote_gemma_watcher_probe.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_v0916_deepseek_suitability.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_v092_demo_proof_coverage.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_validate_v0922_runtime_qualification.py` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_validate_v092_runtime_native_receipts.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_validation_inventory.sh` | tool surface | historical evidence | Historical Runtime source-tree route retired by #1223; invocation fails before execution. Original assertions are preserved in Git and retirement custody, not qualified as passing. |
| `adl/tools/test_validation_manager.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_warm_rust_dependency_cache.py` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_workflow_guardrails.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_workspace_coverage_deadline.rb` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_worktree_doctor.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_worktree_prune.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_wp17a_demo_follow_ons.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/test_xcl_terraform_provider_major_bounds.sh` | tool surface | internal helper | Focused regression or contract test helper. |
| `adl/tools/tests` | tool surface | internal helper | Directory containing bounded helper assets, scripts, or proof inputs. |
| `adl/tools/update_latest_reports.sh` | tool surface | internal helper | Shell helper retained for bounded workflow support. |
| `adl/tools/update_reports_index.sh` | tool surface | internal helper | Shell helper retained for bounded workflow support. |
| `adl/tools/validate_architecture_docs.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_aws_runtime_platform_static.sh` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_ci_workflow_policy.rb` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_codebuddy_review_showcase_demo.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_csdlc_demo_proof_contract_packet.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_csdlc_v2_supply_chain.sh` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_csm_public_edge_live.sh` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_csm_public_edge_static.sh` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_evidence_bundle_packet.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_first_proof_demo_packet.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_first_proof_readiness_packet.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_five_minute_html_game_packet.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_five_minute_sprint_console_packet.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_merge_readiness_packet.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_obsmem_handoff_packet.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_planning_template.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_podcast_launch_packet.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_podcast_studio_v2_packet.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_polis_status_for_ssm_qts.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_polis_status_for_ssm_windows.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_pvf_manifest.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_skill_frontmatter.sh` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_software_development_polis_packet.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_transition_dag_packet.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_v0913_quality_gate_review_surfaces.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_v0915_multi_agent_quality_comparison.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_v0915_openrouter_matrix.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_v0915_remote_gemma_watcher_probe.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_v0916_agent_suitability_panel.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_v0916_deepseek_suitability.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_v0917_csm_4890_status.sh` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_v0917_csm_api_gateway_bridge_proof.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_v0917_csm_continuity_capsule_4910_status.sh` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_v0917_csm_governed_notice_4998_status.sh` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_v0917_csm_otlp_4904_status.sh` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_v0917_csm_service_4903_status.sh` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_v0917_html_observatory.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_v0917_no_sparrow_4909_status.sh` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_v0917_soak2_4682_status.sh` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_v0917_soak2_matrix.sh` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_v0922_runtime_qualification.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_v092_browser_trusted_observatory.mjs` | tool surface | historical evidence | Historical Runtime source-tree route retired by #1223; invocation fails before execution. Original assertions are preserved in Git and retirement custody, not qualified as passing. |
| `adl/tools/validate_v092_demo_proof_coverage.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_v092_durable_history_parent_integration.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_v092_html_observatory_roster.mjs` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_v092_runtime_guardian_lifecycle.sh` | tool surface | historical evidence | Historical Runtime source-tree route retired by #1223; invocation fails before execution. Original assertions are preserved in Git and retirement custody, not qualified as passing. |
| `adl/tools/validate_v092_runtime_native_receipts.rb` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_wp08_acip_sns_live_proof.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_wp08_aws_signal_integration_live_proof.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_wp08_cloudfront_control_proof.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_wp08_heartbeat_live_proof.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validate_wp08_polis_storage_live_proof.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/validation_inventory.py` | tool surface | internal helper | Python helper or generator used by a bounded tooling surface. |
| `adl/tools/validation_inventory.sh` | tool surface | internal helper | Shell helper retained for bounded workflow support. |
| `adl/tools/validation_manager.py` | tool surface | internal helper | Python helper or generator used by a bounded tooling surface. |
| `adl/tools/validation_manager.sh` | tool surface | internal helper | Shell helper retained for bounded workflow support. |
| `adl/tools/verify_ci_backend_route.py` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/verify_external_git_baseline.sh` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/verify_review_output_provenance.sh` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/verify_spot_advertised_ref.sh` | tool surface | internal helper | Bounded validation/generation helper used by primary workflow entrypoints. |
| `adl/tools/warm_rust_dependency_cache.py` | tool surface | internal helper | Python helper or generator used by a bounded tooling surface. |
| `adl/tools/workflow_guardrails.sh` | tool surface | internal helper | Shell helper retained for bounded workflow support. |
| `adl/tools/worktree_doctor.sh` | tool surface | internal helper | Shell helper retained for bounded workflow support. |
| `adl/tools/worktree_prune.sh` | tool surface | internal helper | Shell helper retained for bounded workflow support. |

## ADL Skills

Source skill directories only; installed skill admission is not inventoried here.

| Surface | Kind | Classification | Notes |
| --- | --- | --- | --- |
| `adl/tools/skills/pr-run` | skill | historical evidence | Partial historical ADL compatibility reference; use the selected native owner for current lifecycle commands. |

## Historical Evidence Artifacts

| Surface | Kind | Classification | Notes |
| --- | --- | --- | --- |
| `docs/milestones/v0.91.5/CLI_COMMAND_INVENTORY_3594.md` | historical evidence | historical evidence | Historical command-inventory evidence and prior ownership table. |
| `docs/milestones/v0.91.5/CLI_WRAPPER_MIGRATION_CONTRACT_3597.md` | historical evidence | historical evidence | Historical migration contract for wrapper/public workflow spine truth. |
| `docs/milestones/v0.91.5/CLI_SHIM_DEPRECATION_POLICY_3615.md` | historical evidence | historical evidence | Historical compatibility-sunset policy and active-bundle scan gate. |
| `docs/milestones/v0.91.5/review/tooling_adoption/CSDLC_SMALL_BINARIES_PROOF_3832.md` | historical evidence | historical evidence | Historical proof packet for the validator/editor small-binary slice. |

## Non-Claims

- This registry does not approve final deletion of retired wrappers.
- This registry does not prove active-reference absence for any shim.
- This registry does not replace the later simplification review and scan issues.
