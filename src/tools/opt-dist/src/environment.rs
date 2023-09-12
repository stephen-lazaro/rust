use camino::Utf8PathBuf;
use derive_builder::Builder;

#[derive(Builder)]
pub struct Environment {
    host_triple: String,
    python_binary: String,
    /// The rustc checkout, where the compiler source is located.
    checkout_dir: Utf8PathBuf,
    /// The main directory where the build occurs.
    build_dir: Utf8PathBuf,
    /// Directory where the optimization artifacts (PGO/BOLT profiles, etc.)
    /// will be stored.
    artifact_dir: Utf8PathBuf,
    /// Path to the host LLVM used to compile LLVM in `src/llvm-project`.
    host_llvm_dir: Utf8PathBuf,
    /// List of test paths that should be skipped when testing the optimized artifacts.
    skipped_tests: Vec<&'static str>,
    use_bolt: bool,
    shared_llvm: bool,
}

impl Environment {
    pub fn host_triple(&self) -> &str {
        &self.host_triple
    }

    pub fn python_binary(&self) -> &str {
        &self.python_binary
    }

    pub fn checkout_path(&self) -> Utf8PathBuf {
        self.checkout_dir.clone()
    }

    pub fn build_root(&self) -> Utf8PathBuf {
        self.build_dir.clone()
    }

    pub fn build_artifacts(&self) -> Utf8PathBuf {
        self.build_root().join("build").join(&self.host_triple)
    }

    pub fn artifact_dir(&self) -> Utf8PathBuf {
        self.artifact_dir.clone()
    }

    pub fn cargo_stage_0(&self) -> Utf8PathBuf {
        self.build_artifacts()
            .join("stage0")
            .join("bin")
            .join(format!("cargo{}", executable_extension()))
    }

    pub fn rustc_stage_0(&self) -> Utf8PathBuf {
        self.build_artifacts()
            .join("stage0")
            .join("bin")
            .join(format!("rustc{}", executable_extension()))
    }

    pub fn rustc_stage_2(&self) -> Utf8PathBuf {
        self.build_artifacts()
            .join("stage2")
            .join("bin")
            .join(format!("rustc{}", executable_extension()))
    }

    /// Path to the built rustc-perf benchmark suite.
    pub fn rustc_perf_dir(&self) -> Utf8PathBuf {
        self.artifact_dir.join("rustc-perf")
    }

    pub fn host_llvm_dir(&self) -> Utf8PathBuf {
        self.host_llvm_dir.clone()
    }

    pub fn use_bolt(&self) -> bool {
        self.use_bolt
    }

    pub fn supports_shared_llvm(&self) -> bool {
        self.shared_llvm
    }

    pub fn skipped_tests(&self) -> &[&'static str] {
        &self.skipped_tests
    }
}

/// What is the extension of binary executables on this platform?
#[cfg(target_family = "unix")]
pub fn executable_extension() -> &'static str {
    ""
}

#[cfg(target_family = "windows")]
pub fn executable_extension() -> &'static str {
    ".exe"
}
