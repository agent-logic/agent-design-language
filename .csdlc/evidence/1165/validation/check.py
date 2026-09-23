import importlib.util
import json
from pathlib import Path
from email.utils import parsedate_to_datetime
import xml.etree.ElementTree as ET
root = Path.cwd()
spec = importlib.util.spec_from_file_location("podcast_validator", root / "adl/tools/validate_podcast_launch_packet.py")
validator = importlib.util.module_from_spec(spec)
spec.loader.exec_module(validator)
validator.validate_feed(root / "demos/podcast")
package = root / "demos/podcast/episodes/001-meet-the-ai-coworkers"
validator.validate_storage_manifest(root / "demos/podcast", package, json.loads((package / "episode.json").read_text()))
channel = ET.parse(root / "demos/podcast/feed.xml").getroot().find("channel")
item = channel.find("item")
metadata = json.loads((root / "demos/podcast/episodes/001-meet-the-ai-coworkers/episode.json").read_text())
assert metadata["publication_status"] == "held_for_human_review"
assert item.find("pubDate") is None, "unreleased candidate must not claim a publication date"
assert item.findtext("guid") == metadata["guid"]
assert parsedate_to_datetime(channel.findtext("lastBuildDate")).tzinfo is not None
for disclosure in ("model-authored dialogue", "synthetic voices", "human editorial production"):
    assert disclosure in channel.findtext("description"), disclosure
print("Podcast feed metadata and existing enclosure contract verified")
