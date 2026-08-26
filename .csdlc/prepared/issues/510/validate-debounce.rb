#!/usr/bin/env ruby
# frozen_string_literal: true

Dir.chdir(File.expand_path("../../../../adl-runtime", __dir__))
cmd = ["cargo", "test", "--test", "config_reload", "file_events_are_debounced", "--", "--exact"]
abort("debounce failed") unless system(*cmd)
