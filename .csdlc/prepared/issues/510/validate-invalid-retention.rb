#!/usr/bin/env ruby
# frozen_string_literal: true

Dir.chdir(File.expand_path("../../../../adl-runtime", __dir__))
cmd = ["cargo", "test", "--test", "config_reload", "invalid_update_retains_last_known_good", "--", "--exact"]
abort("invalid-retention failed") unless system(*cmd)
