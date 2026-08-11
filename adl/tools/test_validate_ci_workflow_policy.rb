#!/usr/bin/env ruby
# frozen_string_literal: true

require "minitest/autorun"
require_relative "validate_ci_workflow_policy"

class ValidateCiWorkflowPolicyTest < Minitest::Test
  def valid_ci
    <<~YAML
      name: ci
      "on":
        pull_request:
        workflow_dispatch:
      concurrency:
        group: ${{ github.repository }}-${{ github.workflow }}-${{ github.event.pull_request.head.repo.full_name }}-${{ github.event.pull_request.head.ref }}
        cancel-in-progress: true
      jobs:
        adl_path_policy:
          runs-on: ubuntu-latest
        adl_ci:
          needs: adl_path_policy
          runs-on: ${{ needs.adl_path_policy.outputs.required_runner }}
        adl_coverage:
          runs-on: ubuntu-latest
    YAML
  end

  def sources(ci: valid_ci, extra: {})
    { ".github/workflows/ci.yaml" => ci }.merge(extra)
  end

  def errors_for(**values)
    CiWorkflowPolicy.validate_sources(sources(**values)).first
  end

  def test_valid_minimal_required_pr_surface_passes
    assert_empty(errors_for)
  end

  def test_optional_job_is_rejected_even_when_job_if_is_false
    fixture = valid_ci.sub(
      "  adl_coverage:\n",
      "  adl-slow-proof:\n    if: false\n    runs-on: ubuntu-latest\n  adl_coverage:\n"
    )
    assert(errors_for(ci: fixture).any? { |error| error.include?("optional jobs must not materialize") })
  end

  def test_two_shard_heavy_matrix_is_rejected
    fixture = valid_ci.sub(
      "    runs-on: ${{ needs.adl_path_policy.outputs.required_runner }}\n",
      "    runs-on: ${{ needs.adl_path_policy.outputs.required_runner }}\n    strategy:\n      matrix:\n        shard: [1, 2]\n"
    )
    assert(errors_for(ci: fixture).any? { |error| error.include?("matrix expansion count must be exactly one") })
  end

  def test_quoted_pull_request_key_in_standalone_workflow_is_rejected
    standalone = <<~YAML
      name: escaped
      "on":
        "pull_request":
        workflow_dispatch:
      jobs:
        proof:
          runs-on: ubuntu-latest
    YAML
    errors = errors_for(extra: { ".github/workflows/escaped.yml" => standalone })
    assert(errors.any? { |error| error.include?("automatic event pull_request is not allowed") })
  end
end
