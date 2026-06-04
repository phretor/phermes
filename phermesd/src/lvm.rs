//! LVM-thin operations: pure argv builders, an `lvs --reportformat json` parser,
//! and (in a later task) the `LvmOps` seam.

use serde::Deserialize;

/// One logical volume as reported by `lvs`.
#[derive(Debug, Clone, PartialEq)]
pub struct Lv {
    pub lv_name: String,
    pub tags: Vec<String>,
    pub pool_lv: String,
    pub origin: String,
    pub data_percent: Option<f64>,
}

#[must_use]
pub fn create_thin_argv(vg: &str, pool: &str, name: &str, size_gb: u32) -> Vec<String> {
    vec![
        "lvcreate".into(),
        "--thin".into(),
        "--virtualsize".into(),
        format!("{size_gb}G"),
        format!("{vg}/{pool}"),
        "-n".into(),
        name.into(),
    ]
}

#[must_use]
pub fn addtag_argv(device: &str, tag: &str) -> Vec<String> {
    vec!["lvchange".into(), "--addtag".into(), tag.into(), device.into()]
}

#[must_use]
pub fn snapshot_argv(vg: &str, origin: &str, snap_name: &str) -> Vec<String> {
    vec![
        "lvcreate".into(),
        "--snapshot".into(),
        "--name".into(),
        snap_name.into(),
        format!("{vg}/{origin}"),
    ]
}

#[must_use]
pub fn merge_argv(vg: &str, snap_name: &str) -> Vec<String> {
    vec!["lvconvert".into(), "--merge".into(), format!("{vg}/{snap_name}")]
}

#[must_use]
pub fn remove_argv(device: &str) -> Vec<String> {
    vec!["lvremove".into(), "-y".into(), device.into()]
}

#[must_use]
pub fn lvs_json_argv(vg: &str) -> Vec<String> {
    vec![
        "lvs".into(),
        "--reportformat".into(),
        "json".into(),
        "-o".into(),
        "lv_name,lv_tags,pool_lv,origin,data_percent".into(),
        vg.into(),
    ]
}

#[derive(Deserialize)]
struct LvsReport {
    report: Vec<LvsGroup>,
}

#[derive(Deserialize)]
struct LvsGroup {
    lv: Vec<LvsRow>,
}

#[derive(Deserialize)]
struct LvsRow {
    lv_name: String,
    lv_tags: String,
    pool_lv: String,
    origin: String,
    data_percent: String,
}

/// Parse `lvs --reportformat json` output into [`Lv`] rows.
///
/// # Errors
///
/// Returns `serde_json::Error` if the JSON is malformed or does not match the
/// expected `{"report":[{"lv":[...]}]}` shape.
pub fn parse_lvs(json: &str) -> Result<Vec<Lv>, serde_json::Error> {
    let parsed: LvsReport = serde_json::from_str(json)?;
    let mut out = Vec::new();
    for group in parsed.report {
        for row in group.lv {
            let tags = row
                .lv_tags
                .split(',')
                .filter(|t| !t.is_empty())
                .map(str::to_string)
                .collect();
            let data_percent = row.data_percent.parse::<f64>().ok();
            out.push(Lv {
                lv_name: row.lv_name,
                tags,
                pool_lv: row.pool_lv,
                origin: row.origin,
                data_percent,
            });
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_thin_argv_is_correct() {
        let a = create_thin_argv("pve", "data", "vm-102-disk-0", 40);
        assert_eq!(
            a,
            vec!["lvcreate", "--thin", "--virtualsize", "40G", "pve/data", "-n", "vm-102-disk-0"]
        );
    }

    #[test]
    fn addtag_argv_targets_the_device_path() {
        assert_eq!(
            addtag_argv("/dev/pve/vm-102-disk-0", "@phermesd"),
            vec!["lvchange", "--addtag", "@phermesd", "/dev/pve/vm-102-disk-0"]
        );
    }

    #[test]
    fn snapshot_argv_is_a_thin_snapshot() {
        let a = snapshot_argv("pve", "vm-102-disk-0", "vm-102-disk-0-snap-auto-20260603T141500Z");
        assert_eq!(
            a,
            vec![
                "lvcreate",
                "--snapshot",
                "--name",
                "vm-102-disk-0-snap-auto-20260603T141500Z",
                "pve/vm-102-disk-0"
            ]
        );
    }

    #[test]
    fn merge_and_remove_argv() {
        assert_eq!(
            merge_argv("pve", "vm-102-disk-0-snap-manual-x"),
            vec!["lvconvert", "--merge", "pve/vm-102-disk-0-snap-manual-x"]
        );
        assert_eq!(
            remove_argv("/dev/pve/vm-102-disk-0"),
            vec!["lvremove", "-y", "/dev/pve/vm-102-disk-0"]
        );
    }

    #[test]
    fn lvs_json_argv_requests_machine_readable_report() {
        let a = lvs_json_argv("pve");
        assert!(a.contains(&"--reportformat".to_string()));
        assert!(a.contains(&"json".to_string()));
        assert!(a.iter().any(|s| s.contains("lv_name")));
        assert_eq!(a.last().unwrap(), "pve");
    }

    #[test]
    fn parse_lvs_extracts_volumes_tags_and_pool_percent() {
        let json = r#"{"report":[{"lv":[
          {"lv_name":"data","lv_tags":"","pool_lv":"","origin":"","data_percent":"42.50"},
          {"lv_name":"vm-102-disk-0","lv_tags":"@phermesd","pool_lv":"data","origin":"","data_percent":""},
          {"lv_name":"vm-102-disk-0-snap-auto-20260603T141500Z","lv_tags":"@phermesd-snap","pool_lv":"data","origin":"vm-102-disk-0","data_percent":""}
        ]}]}"#;
        let lvs = parse_lvs(json).unwrap();
        assert_eq!(lvs.len(), 3);
        let pool = lvs.iter().find(|l| l.lv_name == "data").unwrap();
        assert_eq!(pool.data_percent, Some(42.5));
        let snap = lvs.iter().find(|l| l.origin == "vm-102-disk-0").unwrap();
        assert!(snap.tags.iter().any(|t| t == "@phermesd-snap"));
    }
}
