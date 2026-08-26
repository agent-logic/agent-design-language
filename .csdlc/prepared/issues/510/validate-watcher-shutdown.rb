#!/usr/bin/env ruby
# frozen_string_literal: true

Dir.chdir(File.expand_path("../../../../adl-runtime", __dir__))
cmd = ["cargo", "test", "--test", "config_reload", "watcher_shutdown_is_clean", "--", "--exact"]
abort("watcher-shutdown failed") unless system(*cmd)
