#!/usr/bin/env ruby
# frozen_string_literal: true

Dir.chdir(File.expand_path("../../../../adl-runtime", __dir__))
cmd = ["cargo", "test", "--test", "config_reload", "valid_reload_atomically_replaces_snapshot", "--", "--exact"]
abort("valid-reload failed") unless system(*cmd)
