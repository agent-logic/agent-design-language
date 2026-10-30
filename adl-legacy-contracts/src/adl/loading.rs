use crate::yaml as serde_yaml;
use anyhow::{anyhow, Context, Result};
use serde_yaml::{Mapping as YamlMapping, Value as YamlValue};
use std::fs;
use std::io::Read;
use std::path::{Component, Path, PathBuf};

// One budget per top-level load; repeated includes consume reads and bytes again.
const MAX_DOCUMENT_DEPTH: usize = 32;
const MAX_DOCUMENT_READS: usize = 256;
const MAX_DOCUMENT_BYTES: usize = 8 * 1024 * 1024;

struct LoadBudget {
    reads_left: usize,
    bytes_left: usize,
}

impl Default for LoadBudget {
    fn default() -> Self {
        Self {
            reads_left: MAX_DOCUMENT_READS,
            bytes_left: MAX_DOCUMENT_BYTES,
        }
    }
}

impl LoadBudget {
    fn read(&mut self, path: &Path) -> Result<String> {
        if self.reads_left == 0 {
            return Err(anyhow!("ADL include read-count limit exceeded"));
        }
        self.reads_left -= 1;
        // Read at most the remaining budget plus one sentinel byte. Metadata alone
        // cannot bound a file that grows between inspection and the actual read.
        let file = fs::File::open(path)
            .with_context(|| format!("read adl include file: {}", path.display()))?;
        let mut bytes = Vec::new();
        file.take(self.bytes_left as u64 + 1)
            .read_to_end(&mut bytes)
            .with_context(|| format!("read adl include file: {}", path.display()))?;
        if bytes.len() > self.bytes_left {
            return Err(anyhow!("ADL include aggregate-byte limit exceeded"));
        }
        self.bytes_left -= bytes.len();
        String::from_utf8(bytes)
            .with_context(|| format!("ADL include is not UTF-8: {}", path.display()))
    }
}

pub(super) fn load_yaml_with_includes(path: &Path, stack: &mut Vec<PathBuf>) -> Result<YamlValue> {
    let canonical = path
        .canonicalize()
        .with_context(|| format!("resolve include path: {}", path.display()))?;
    let root = canonical
        .parent()
        .context("initial ADL document has no parent")?;
    load_yaml_within_root(&canonical, root, stack, &mut LoadBudget::default())
}

fn load_yaml_within_root(
    path: &Path,
    root: &Path,
    stack: &mut Vec<PathBuf>,
    budget: &mut LoadBudget,
) -> Result<YamlValue> {
    if stack.len() >= MAX_DOCUMENT_DEPTH {
        return Err(anyhow!(
            "ADL include depth limit exceeded (maximum {MAX_DOCUMENT_DEPTH} documents)"
        ));
    }
    let canonical = path
        .canonicalize()
        .with_context(|| format!("resolve include path: {}", path.display()))?;
    if !canonical.starts_with(root) {
        return Err(anyhow!(
            "include resolves outside initial document directory: {}",
            path.display()
        ));
    }
    if let Some(idx) = stack.iter().position(|p| p == &canonical) {
        let mut cycle = stack[idx..]
            .iter()
            .map(|p| p.display().to_string())
            .collect::<Vec<_>>();
        cycle.push(canonical.display().to_string());
        return Err(anyhow!("include cycle detected: {}", cycle.join(" -> ")));
    }

    stack.push(canonical.clone());
    let result = (|| -> Result<YamlValue> {
        let text = budget.read(&canonical)?;
        let mut doc: YamlValue = serde_yaml::from_str(&text)
            .with_context(|| format!("parse adl yaml: {}", canonical.display()))?;
        let map = doc.as_mapping_mut().ok_or_else(|| {
            anyhow!(
                "top-level ADL document must be a mapping: {}",
                canonical.display()
            )
        })?;

        let include_key = YamlValue::String("include".to_string());
        let include_list = map.remove(&include_key);

        let mut merged = YamlMapping::new();

        if let Some(include_list) = include_list {
            let includes = include_list.as_sequence().ok_or_else(|| {
                anyhow!(
                    "include must be a YAML sequence of relative file paths: {}",
                    canonical.display()
                )
            })?;
            let base_dir = canonical.parent().unwrap_or(Path::new("."));
            for include_item in includes {
                let include_rel = include_item.as_str().ok_or_else(|| {
                    anyhow!("include entries must be strings in {}", canonical.display())
                })?;
                let include_path = Path::new(include_rel);
                if include_path.is_absolute()
                    || include_path
                        .components()
                        .any(|c| matches!(c, Component::ParentDir))
                {
                    return Err(anyhow!(
                        "include path must be relative and must not contain '..': '{}' in {}",
                        include_rel,
                        canonical.display()
                    ));
                }
                let include_file = base_dir.join(include_path);
                let include_doc = load_yaml_within_root(&include_file, root, stack, budget)?;
                let include_map = include_doc.as_mapping().ok_or_else(|| {
                    anyhow!(
                        "included ADL document must be a mapping: {}",
                        include_file.display()
                    )
                })?;
                merge_top_level_map(&mut merged, include_map, &include_file)?;
            }
        }

        merge_top_level_map(&mut merged, map, &canonical)?;
        Ok(YamlValue::Mapping(merged))
    })();
    let _ = stack.pop();
    result
}

fn merge_top_level_map(dst: &mut YamlMapping, src: &YamlMapping, src_path: &Path) -> Result<()> {
    for (key, value) in src {
        let Some(key_name) = key.as_str() else {
            return Err(anyhow!(
                "top-level ADL keys must be strings in {}",
                src_path.display()
            ));
        };

        match key_name {
            "providers" | "tools" | "agents" | "tasks" | "workflows" => {
                let src_map = value.as_mapping().ok_or_else(|| {
                    anyhow!(
                        "top-level '{}' must be a mapping in {}",
                        key_name,
                        src_path.display()
                    )
                })?;
                if !dst.contains_key(key) {
                    dst.insert(key.clone(), YamlValue::Mapping(YamlMapping::new()));
                }
                let dst_entry = dst.get_mut(key).expect("inserted mapping");
                let dst_map = dst_entry.as_mapping_mut().ok_or_else(|| {
                    anyhow!("top-level '{}' merge target is not a mapping", key_name)
                })?;
                for (item_key, item_val) in src_map {
                    if dst_map.contains_key(item_key) {
                        let item_id = item_key
                            .as_str()
                            .map(|s| s.to_string())
                            .unwrap_or_else(|| "<non-string>".to_string());
                        return Err(anyhow!(
                            "duplicate {} id '{}' while processing includes (source: {})",
                            key_name,
                            item_id,
                            src_path.display()
                        ));
                    }
                    dst_map.insert(item_key.clone(), item_val.clone());
                }
            }
            "patterns" => {
                let src_seq = value.as_sequence().ok_or_else(|| {
                    anyhow!(
                        "top-level '{}' must be a sequence in {}",
                        key_name,
                        src_path.display()
                    )
                })?;
                if !dst.contains_key(key) {
                    dst.insert(key.clone(), YamlValue::Sequence(Vec::new()));
                }
                let dst_entry = dst.get_mut(key).expect("inserted sequence");
                let dst_seq = dst_entry.as_sequence_mut().ok_or_else(|| {
                    anyhow!("top-level '{}' merge target is not a sequence", key_name)
                })?;
                dst_seq.extend(src_seq.iter().cloned());
            }
            _ => {
                if dst.contains_key(key) {
                    return Err(anyhow!(
                        "duplicate top-level key '{}' while processing includes (source: {})",
                        key_name,
                        src_path.display()
                    ));
                }
                dst.insert(key.clone(), value.clone());
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_yaml::Number as YamlNumber;

    // PVF deterministic unit regression: bounded temporary YAML fixtures, no providers.
    // Proof role: loader resource boundaries and semantic preservation; small local IO.
    // Release gate: public package validation and coverage (adl-legacy-contracts).
    #[test]
    fn include_depth_boundary_and_stack_cleanup() {
        let temp = tempfile::tempdir().unwrap();
        for n in 0..MAX_DOCUMENT_DEPTH {
            let text = if n + 1 == MAX_DOCUMENT_DEPTH {
                "{}".to_string()
            } else {
                format!("include: ['{}.yaml']", n + 1)
            };
            fs::write(temp.path().join(format!("{n}.yaml")), text).unwrap();
        }
        let root = temp.path().join("0.yaml");
        let mut stack = Vec::new();
        load_yaml_with_includes(&root, &mut stack).unwrap();
        assert!(stack.is_empty());
        fs::write(
            temp.path().join(format!("{}.yaml", MAX_DOCUMENT_DEPTH - 1)),
            format!("include: ['{}.yaml']", MAX_DOCUMENT_DEPTH),
        )
        .unwrap();
        // The rejected child need not exist: depth is refused before filesystem IO.
        let error = load_yaml_with_includes(&root, &mut stack).unwrap_err();
        assert!(error.to_string().contains("depth limit exceeded"));
        assert!(stack.is_empty());
    }

    #[test]
    fn repeated_includes_consume_global_read_budget() {
        let temp = tempfile::tempdir().unwrap();
        fs::write(temp.path().join("empty.yaml"), "{}").unwrap();
        let root = temp.path().join("root.yaml");
        for (includes, succeeds) in [(MAX_DOCUMENT_READS - 1, true), (MAX_DOCUMENT_READS, false)] {
            fs::write(
                &root,
                format!("include: [{}]", vec!["empty.yaml"; includes].join(",")),
            )
            .unwrap();
            let mut stack = Vec::new();
            let result = load_yaml_with_includes(&root, &mut stack);
            if succeeds {
                result.unwrap();
            } else {
                assert!(result
                    .unwrap_err()
                    .to_string()
                    .contains("read-count limit exceeded"));
            }
            assert!(stack.is_empty());
        }
    }

    #[test]
    fn aggregate_bytes_include_root_and_repeated_fragments() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path().join("root.yaml");
        let text = "include: [empty.yaml, empty.yaml]";
        fs::write(&root, text).unwrap();
        fs::write(temp.path().join("empty.yaml"), "{}").unwrap();
        let canonical = root.canonicalize().unwrap();
        let total = text.len() + 4;
        for (limit, succeeds) in [(total, true), (total - 1, false)] {
            let mut budget = LoadBudget {
                reads_left: 3,
                bytes_left: limit,
            };
            let mut stack = Vec::new();
            let result = load_yaml_within_root(
                &canonical,
                canonical.parent().unwrap(),
                &mut stack,
                &mut budget,
            );
            if succeeds {
                result.unwrap();
                assert_eq!(budget.bytes_left, 0);
                assert_eq!(budget.reads_left, 0);
            } else {
                assert!(result
                    .unwrap_err()
                    .to_string()
                    .contains("aggregate-byte limit exceeded"));
            }
            assert!(stack.is_empty());
        }
    }

    #[test]
    fn bounded_read_rejects_oversize_before_utf8_or_yaml_parsing() {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path().join("root.yaml");
        fs::write(&root, [0xff; 64]).unwrap();
        let mut budget = LoadBudget {
            reads_left: 1,
            bytes_left: 8,
        };
        assert!(budget
            .read(&root)
            .unwrap_err()
            .to_string()
            .contains("aggregate-byte limit exceeded"));
        let mut budget = LoadBudget::default();
        assert!(budget
            .read(&root)
            .unwrap_err()
            .to_string()
            .contains("not UTF-8"));
    }

    #[test]
    fn merge_top_level_map_rejects_non_string_keys_and_shape_mismatches() {
        let src_path = Path::new("fragment.yaml");

        let mut non_string_key = YamlMapping::new();
        non_string_key.insert(
            YamlValue::Number(YamlNumber::from(7)),
            YamlValue::String("bad".to_string()),
        );
        let err = merge_top_level_map(&mut YamlMapping::new(), &non_string_key, src_path)
            .expect_err("non-string keys should fail");
        assert!(err
            .to_string()
            .contains("top-level ADL keys must be strings"));

        let mut dst = YamlMapping::new();
        dst.insert(
            YamlValue::String("providers".to_string()),
            YamlValue::Sequence(Vec::new()),
        );
        let mut src = YamlMapping::new();
        src.insert(
            YamlValue::String("providers".to_string()),
            YamlValue::Mapping(YamlMapping::new()),
        );
        let err = merge_top_level_map(&mut dst, &src, src_path)
            .expect_err("mapping merge target mismatch should fail");
        assert!(err
            .to_string()
            .contains("top-level 'providers' merge target is not a mapping"));

        let mut dst = YamlMapping::new();
        dst.insert(
            YamlValue::String("patterns".to_string()),
            YamlValue::Mapping(YamlMapping::new()),
        );
        let mut src = YamlMapping::new();
        src.insert(
            YamlValue::String("patterns".to_string()),
            YamlValue::Sequence(Vec::new()),
        );
        let err = merge_top_level_map(&mut dst, &src, src_path)
            .expect_err("sequence merge target mismatch should fail");
        assert!(err
            .to_string()
            .contains("top-level 'patterns' merge target is not a sequence"));
    }

    #[test]
    fn merge_top_level_map_merges_component_maps_and_appends_patterns() {
        let src_path = Path::new("fragment.yaml");
        let mut dst = YamlMapping::new();

        let mut existing_providers = YamlMapping::new();
        existing_providers.insert(
            YamlValue::String("p1".to_string()),
            YamlValue::String("provider-one".to_string()),
        );
        dst.insert(
            YamlValue::String("providers".to_string()),
            YamlValue::Mapping(existing_providers),
        );
        dst.insert(
            YamlValue::String("patterns".to_string()),
            YamlValue::Sequence(vec![YamlValue::String("base".to_string())]),
        );

        let mut src = YamlMapping::new();
        let mut extra_providers = YamlMapping::new();
        extra_providers.insert(
            YamlValue::String("p2".to_string()),
            YamlValue::String("provider-two".to_string()),
        );
        src.insert(
            YamlValue::String("providers".to_string()),
            YamlValue::Mapping(extra_providers),
        );
        src.insert(
            YamlValue::String("patterns".to_string()),
            YamlValue::Sequence(vec![YamlValue::String("extra".to_string())]),
        );
        src.insert(
            YamlValue::String("version".to_string()),
            YamlValue::String("0.5".to_string()),
        );

        merge_top_level_map(&mut dst, &src, src_path).expect("merge should succeed");

        let providers = dst
            .get(YamlValue::String("providers".to_string()))
            .and_then(YamlValue::as_mapping)
            .expect("providers map");
        assert!(providers.contains_key(YamlValue::String("p1".to_string())));
        assert!(providers.contains_key(YamlValue::String("p2".to_string())));

        let patterns = dst
            .get(YamlValue::String("patterns".to_string()))
            .and_then(YamlValue::as_sequence)
            .expect("patterns sequence");
        assert_eq!(
            patterns,
            &vec![
                YamlValue::String("base".to_string()),
                YamlValue::String("extra".to_string())
            ]
        );

        assert_eq!(
            dst.get(YamlValue::String("version".to_string()))
                .and_then(YamlValue::as_str),
            Some("0.5")
        );
    }
}
