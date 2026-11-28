//! Which stock AWS Lambda base image runs which runtime identifier.

use winterbaume_lambda::StubReason;

/// Where a runtime's base image is published, and how to start its runtime.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeImage {
    /// Repository in `public.ecr.aws`, e.g. `lambda/nodejs`.
    pub repository: String,
    /// e.g. `22`, `3.13`, `al2023`.
    pub tag: String,
    /// The bootstrap programs to try, in order.
    pub bootstrap: Vec<&'static str>,
}

const MANAGED_BOOTSTRAP: &[&str] = &["/var/runtime/bootstrap"];
/// An OS-only runtime runs the `bootstrap` the function ( or a layer )
/// brings.
const CUSTOM_BOOTSTRAP: &[&str] = &["/var/task/bootstrap", "/opt/bootstrap"];

/// The base image for `runtime`.
///
/// Every runtime family that AWS publishes base images for is mapped by
/// its version, so that a new runtime version works as soon as its image
/// exists; a version without an image surfaces as `image-unavailable` when
/// the pull fails.  Runtimes on Amazon Linux 1, whose images AWS no longer
/// publishes, are `deprecated-runtime`.
pub fn image_for(runtime: &str) -> Result<RuntimeImage, StubReason> {
    let managed = |repository: &str, tag: &str| {
        Ok(RuntimeImage {
            repository: format!("lambda/{repository}"),
            tag: tag.to_string(),
            bootstrap: MANAGED_BOOTSTRAP.to_vec(),
        })
    };
    // Versions become image tags and cache path components: digits and
    // single dots only, starting and ending with a digit.
    let version_ok = |v: &str| {
        v.starts_with(|c: char| c.is_ascii_digit())
            && v.ends_with(|c: char| c.is_ascii_digit())
            && !v.contains("..")
            && v.chars().all(|c| c.is_ascii_digit() || c == '.')
    };

    if let Some(major) = runtime
        .strip_prefix("nodejs")
        .and_then(|r| r.strip_suffix(".x"))
    {
        if !version_ok(major) {
            return Err(StubReason::UnsupportedRuntime);
        }
        // Node.js 16 and older ran on Amazon Linux 2 images that are no
        // longer published for both architectures; 18 and later have images.
        return match major.parse::<u32>() {
            Ok(n) if n >= 18 => managed("nodejs", major),
            Ok(_) => Err(StubReason::DeprecatedRuntime),
            Err(_) => Err(StubReason::UnsupportedRuntime),
        };
    }
    if let Some(version) = runtime.strip_prefix("python") {
        if !version_ok(version) {
            return Err(StubReason::UnsupportedRuntime);
        }
        if matches!(version, "2.7" | "3.6" | "3.7") {
            return Err(StubReason::DeprecatedRuntime);
        }
        return managed("python", version);
    }
    if runtime == "java8.al2" {
        return managed("java", "8.al2");
    }
    if runtime == "java8" {
        return Err(StubReason::DeprecatedRuntime);
    }
    if let Some(version) = runtime.strip_prefix("java") {
        return if version_ok(version) {
            managed("java", version)
        } else {
            Err(StubReason::UnsupportedRuntime)
        };
    }
    if let Some(version) = runtime.strip_prefix("ruby") {
        if !version_ok(version) {
            return Err(StubReason::UnsupportedRuntime);
        }
        if version.starts_with('2') {
            return Err(StubReason::DeprecatedRuntime);
        }
        return managed("ruby", version);
    }
    if runtime.starts_with("dotnetcore") {
        return Err(StubReason::DeprecatedRuntime);
    }
    if let Some(version) = runtime.strip_prefix("dotnet") {
        return if version_ok(version) {
            managed("dotnet", version)
        } else {
            Err(StubReason::UnsupportedRuntime)
        };
    }
    match runtime {
        "provided.al2" | "provided.al2023" => Ok(RuntimeImage {
            repository: "lambda/provided".to_string(),
            tag: runtime.trim_start_matches("provided.").to_string(),
            bootstrap: CUSTOM_BOOTSTRAP.to_vec(),
        }),
        "provided" | "go1.x" => Err(StubReason::DeprecatedRuntime),
        _ => Err(StubReason::UnsupportedRuntime),
    }
}

/// The name of a runtime's directory in an image override directory:
/// the runtime identifier itself, e.g. `nodejs22.x`.
pub fn image_dir_name(runtime: &str) -> &str {
    runtime
}

#[cfg(test)]
mod tests {
    use super::*;

    fn image(runtime: &str) -> (String, String, Vec<&'static str>) {
        let image = image_for(runtime).unwrap();
        (image.repository, image.tag, image.bootstrap)
    }

    #[test]
    fn managed_runtimes_map_to_their_base_images() {
        assert_eq!(
            image("nodejs22.x"),
            (
                "lambda/nodejs".into(),
                "22".into(),
                MANAGED_BOOTSTRAP.to_vec()
            )
        );
        assert_eq!(image("nodejs24.x").1, "24");
        assert_eq!(
            image("python3.13"),
            (
                "lambda/python".into(),
                "3.13".into(),
                MANAGED_BOOTSTRAP.to_vec()
            )
        );
        assert_eq!(image("java21").1, "21");
        assert_eq!(image("java8.al2").1, "8.al2");
        assert_eq!(image("ruby3.4").0, "lambda/ruby");
        assert_eq!(image("dotnet8").0, "lambda/dotnet");
    }

    #[test]
    fn os_only_runtimes_run_the_function_bootstrap() {
        assert_eq!(
            image("provided.al2023"),
            (
                "lambda/provided".into(),
                "al2023".into(),
                CUSTOM_BOOTSTRAP.to_vec()
            )
        );
        assert_eq!(image("provided.al2").1, "al2");
    }

    #[test]
    fn retired_and_unknown_runtimes_are_refused() {
        for runtime in [
            "go1.x",
            "provided",
            "nodejs16.x",
            "python2.7",
            "java8",
            "dotnetcore3.1",
            "ruby2.7",
        ] {
            assert_eq!(
                image_for(runtime),
                Err(StubReason::DeprecatedRuntime),
                "{runtime}"
            );
        }
        for runtime in [
            "",
            "cobol85",
            "nodejs.x",
            "pythonx",
            "java/../x",
            "python..",
            "java.1",
            "nodejs..x",
        ] {
            assert_eq!(
                image_for(runtime),
                Err(StubReason::UnsupportedRuntime),
                "{runtime}"
            );
        }
    }
}
