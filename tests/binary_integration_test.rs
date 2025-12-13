use std::process::Command;
use std::path::Path;

fn binary_name() -> String {
  if cfg!(windows) {
    "EnergyLogger4000-Reader.exe".into()
  } else {
    "EnergyLogger4000-Reader".into()
  }
}

#[test]
fn test_info_file() {
  let manifest_dir = env!("CARGO_MANIFEST_DIR");

  let binary_path = Path::new(manifest_dir)
    .join("target")
    .join("debug")
    .join(binary_name());

  let info_test_file = Path::new(env!("CARGO_MANIFEST_DIR"))
    .join("tests")
    .join("data")
    .join("B032361A.BIN");

  let output = Command::new(binary_path)
    .current_dir(manifest_dir)
    .arg("-f")
    .arg(info_test_file)
    .output()
    .expect("failed to run binary");

  assert!(output.status.success(), "Binary exited with {:?}\nstderr:\n{}", output.status.code(), String::from_utf8_lossy(&output.stderr));

  // Checkoutput info file
  let expected_output = "----- FINAL RESULT -----\n--- INFO ---\nUnit ID: 1\nTimestamp: 2025-12-11 17:34:00\nTarif 1: 0.35000000000000003\nTarif 2: 0.25\nTotal power comsumption: 0.045 kWh\nTotal recorded time 2269.98 h\nTotal on time 297.16 h\n- Day: 1\n-- Total kWh today min: 0.009 kWh\n-- Total recorded time today min: 7.4 h\n-- Total on time today min: 0.85 h\n- Day: 2\n-- Total kWh today min: 0.013 kWh\n-- Total recorded time today min: 2.15 h\n-- Total on time today min: 1.71 h\n- Day: 3\n-- Total kWh today min: 0.022 kWh\n-- Total recorded time today min: 6.45 h\n-- Total on time today min: 2.46 h\n- Day: 4\n-- Total kWh today min: 0 kWh\n-- Total recorded time today min: 0 h\n-- Total on time today min: 0 h\n- Day: 5\n-- Total kWh today min: 0 kWh\n-- Total recorded time today min: 0 h\n-- Total on time today min: 0 h\n- Day: 6\n-- Total kWh today min: 0 kWh\n-- Total recorded time today min: 0 h\n-- Total on time today min: 0 h\n- Day: 7\n-- Total kWh today min: 0 kWh\n-- Total recorded time today min: 0 h\n-- Total on time today min: 0 h\n- Day: 8\n-- Total kWh today min: 0 kWh\n-- Total recorded time today min: 0 h\n-- Total on time today min: 0 h\n- Day: 9\n-- Total kWh today min: 0 kWh\n-- Total recorded time today min: 0 h\n-- Total on time today min: 0 h\n- Day: 10\n-- Total kWh today min: 0 kWh\n-- Total recorded time today min: 0 h\n-- Total on time today min: 0 h\n";
  let actual_output = String::from_utf8_lossy(&output.stdout);
  assert_eq!(actual_output, expected_output, "Unexpected output:\n{}", actual_output);
}
