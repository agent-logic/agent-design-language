#!/usr/bin/env ruby
# frozen_string_literal: true

Dir.chdir(File.expand_path("../../../../adl-runtime", __dir__))
cmd = ["cargo", "test", "--test", "config_reload", "concurrent_readers_observe_complete_configurations", "--", "--exact"]
abort("concurrent-read failed") unless system(*cmd)
