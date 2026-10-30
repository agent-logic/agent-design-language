#!/usr/bin/env python3
import importlib.util, json, pathlib, tempfile, unittest
SCRIPT = pathlib.Path(__file__).with_name("issue414_s3_linux_bootstrap.py")
SPEC = importlib.util.spec_from_file_location("issue414_bootstrap", SCRIPT)
MODULE = importlib.util.module_from_spec(SPEC); assert SPEC.loader; SPEC.loader.exec_module(MODULE)
class BootstrapManifestTest(unittest.TestCase):
    def manifest(self):
        commit="a"*40; prefix=f"shepherd/issue-414/{commit}/installer"; artifacts=[{"kind":"ollama_runtime","source_key":MODULE.RUNTIME_KEY,"bundle_key":f"{prefix}/artifacts/runtime/ollama.tar.zst","relative_path":"runtime/ollama.tar.zst","sha256":"1"*64}]
        for index,model in enumerate(MODULE.MODELS,2):
            digest=str(index)*64; relative=f"models/{model.replace(':','-')}.tar.zst"; artifacts.append({"kind":"ollama_model_store","model":model,"source_digest":digest,"source_key":f"shepherd/{model.replace(':','-')}/ollama-0.31.1/{digest}/model-store/store.tar.zst","bundle_key":f"{prefix}/artifacts/{relative}","relative_path":relative,"sha256":str(index+3)*64})
        return {"schema":"adl.issue414.linux_x86_bootstrap.v1","bucket":MODULE.BUCKET,"region":MODULE.REGION,"reviewed_git_sha":commit,"immutable_installer_prefix":prefix,"continuity_binary_sha256":"b"*64,"runner_sha256":"c"*64,"platform":{"os":"linux","arch":"x86_64"},"ollama_version":MODULE.OLLAMA_VERSION,"artifacts":artifacts,"continuity_authority":"none_bootstrap_cache_only"}
    def validate(self,data):
        with tempfile.TemporaryDirectory() as directory:
            path=pathlib.Path(directory)/"manifest.json"; path.write_text(json.dumps(data)); return MODULE.load_and_validate(path)
    def test_exact_linux_matrix_passes(self): self.assertEqual(len(self.validate(self.manifest())["artifacts"]),4)
    def test_mac_artifact_fails_closed(self):
        data=self.manifest(); data["artifacts"][1]["source_key"]+="/metal"
        with self.assertRaises(ValueError): self.validate(data)
    def test_s3_cannot_be_continuity_authority(self):
        data=self.manifest(); data["continuity_authority"]="restore"
        with self.assertRaises(ValueError): self.validate(data)
    def test_manifest_must_match_executing_reviewed_sha(self):
        data=self.manifest()
        with tempfile.TemporaryDirectory() as directory:
            path=pathlib.Path(directory)/"manifest.json"; path.write_text(json.dumps(data))
            with self.assertRaises(ValueError): MODULE.load_and_validate(path, "d"*40)
    def test_runner_and_binary_provenance_are_required(self):
        data=self.manifest(); del data["runner_sha256"]
        with self.assertRaises(ValueError): self.validate(data)
if __name__ == "__main__": unittest.main()
