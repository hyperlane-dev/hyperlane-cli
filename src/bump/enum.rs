/// Types of version bumps
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum BumpVersionType {
    /// Bump patch version (0.1.2 -> 0.1.3)
    Patch,
    /// Bump minor version (0.1.2 -> 0.2.0)
    Minor,
    /// Bump major version (0.1.2 -> 1.0.0)
    Major,
    /// Remove pre-release identifier to make it a release version
    Release,
    /// Add or bump alpha pre-release version (0.1.2 -> 0.1.2-alpha, 0.1.2-alpha -> 0.1.2-alpha.1)
    Alpha,
    /// Add or bump beta pre-release version (0.1.2 -> 0.1.2-beta, 0.1.2-alpha.2 -> 0.1.2-beta.1)
    Beta,
    /// Add or bump rc pre-release version (0.1.2 -> 0.1.2-rc, 0.1.2-beta.1 -> 0.1.2-rc.1)
    Rc,
}
