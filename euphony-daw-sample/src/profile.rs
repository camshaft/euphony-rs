use core::fmt;
use euphony_daw_identity::{Digest, Encoder};
use std::collections::BTreeMap;

/// The precision a processor computes in internally.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Precision {
    Binary32,
    Binary64,
}

/// How a processor rounds an inexact result.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Rounding {
    /// To nearest, ties to even.
    NearestEven,
    /// Toward zero.
    TowardZero,
}

/// Whether a processor fuses multiplies and adds into one rounding.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FusedOperations {
    Separate,
    Fused,
}

/// How a processor vectorizes its arithmetic.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Simd {
    /// One lane at a time.
    Scalar,
    /// Vectors of the given number of lanes, reduced in lane order.
    Lanes(u16),
}

/// What a processor does with subnormal values.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Subnormals {
    Preserve,
    FlushToZero,
}

/// The numerical rules a processor follows. Its identity is part of every
/// cache identity the processor's results have.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct NumericalProfile {
    pub internal_precision: Precision,
    pub rounding: Rounding,
    pub fused_operations: FusedOperations,
    pub simd: Simd,
    pub subnormals: Subnormals,
}

impl NumericalProfile {
    /// The version of the profile's canonical fields.
    pub const VERSION: u32 = 1;
    const DOMAIN: &'static str = "euphony numerical profile v1";

    pub fn canonical_bytes(&self) -> Vec<u8> {
        let precision = match self.internal_precision {
            Precision::Binary32 => 0,
            Precision::Binary64 => 1,
        };
        let rounding = match self.rounding {
            Rounding::NearestEven => 0,
            Rounding::TowardZero => 1,
        };
        let fused = match self.fused_operations {
            FusedOperations::Separate => 0,
            FusedOperations::Fused => 1,
        };
        let (simd, lanes) = match self.simd {
            Simd::Scalar => (0, 0),
            Simd::Lanes(lanes) => (1, lanes),
        };
        let subnormals = match self.subnormals {
            Subnormals::Preserve => 0,
            Subnormals::FlushToZero => 1,
        };
        let mut encoder = Encoder::new();
        encoder
            .u32(Self::VERSION)
            .u8(precision)
            .u8(rounding)
            .u8(fused)
            .u8(simd)
            .u16(lanes)
            .u8(subnormals);
        encoder.finish()
    }

    /// The profile's identity: a domain-separated digest of its versioned
    /// canonical fields.
    pub fn identity(&self) -> ProfileId {
        ProfileId(Digest::of(Self::DOMAIN, &self.canonical_bytes()))
    }
}

/// The identity of a numerical profile.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ProfileId(pub Digest);

/// A processor's definition: its implementation, its parameter and schema
/// versions and the numerical profile it follows.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ProcessorDefinition {
    pub name: String,
    /// The digest of the processor's implementation.
    pub implementation: Digest,
    pub parameter_version: u32,
    pub schema_version: u32,
    pub profile: NumericalProfile,
}

impl ProcessorDefinition {
    const DOMAIN: &'static str = "euphony processor v1";

    /// The processor's identity, which equal numerical settings don't make
    /// equal for different implementations.
    pub fn identity(&self) -> ProcessorId {
        let mut encoder = Encoder::new();
        encoder
            .str(&self.name)
            .digest(&self.implementation)
            .u32(self.parameter_version)
            .u32(self.schema_version)
            .digest(&self.profile.identity().0);
        ProcessorId(encoder.digest_in(Self::DOMAIN))
    }
}

/// The identity of a processor and its numerical profile together.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ProcessorId(pub Digest);

/// A processor submitted for registration with the identities its submitter
/// computed.
#[derive(Clone, Debug)]
pub struct Registration {
    pub definition: ProcessorDefinition,
    pub profile: ProfileId,
    pub processor: ProcessorId,
}

impl Registration {
    /// A registration whose identities are derived from `definition`.
    pub fn of(definition: ProcessorDefinition) -> Self {
        Self {
            profile: definition.profile.identity(),
            processor: definition.identity(),
            definition,
        }
    }
}

/// Why a registration was refused.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RegistrationError {
    /// The claimed profile identity isn't the one its fields derive.
    ProfileMismatch {
        claimed: ProfileId,
        derived: ProfileId,
    },
    /// The claimed processor identity isn't the one its definition derives.
    ProcessorMismatch {
        claimed: ProcessorId,
        derived: ProcessorId,
    },
}

impl fmt::Display for RegistrationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ProfileMismatch { claimed, derived } => {
                write!(f, "profile {} derives as {}", claimed.0, derived.0)
            }
            Self::ProcessorMismatch { claimed, derived } => {
                write!(f, "processor {} derives as {}", claimed.0, derived.0)
            }
        }
    }
}

impl std::error::Error for RegistrationError {}

/// The processors a host runs, by identity.
#[derive(Clone, Debug, Default)]
pub struct Registry {
    processors: BTreeMap<ProcessorId, ProcessorDefinition>,
}

impl Registry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a processor after deriving both of its identities again, and
    /// refuses a registration whose claimed identities differ from them.
    pub fn register(
        &mut self,
        registration: Registration,
    ) -> Result<ProcessorId, RegistrationError> {
        let derived = registration.definition.profile.identity();
        if derived != registration.profile {
            return Err(RegistrationError::ProfileMismatch {
                claimed: registration.profile,
                derived,
            });
        }
        let derived = registration.definition.identity();
        if derived != registration.processor {
            return Err(RegistrationError::ProcessorMismatch {
                claimed: registration.processor,
                derived,
            });
        }
        self.processors.insert(derived, registration.definition);
        Ok(derived)
    }

    pub fn get(&self, id: &ProcessorId) -> Option<&ProcessorDefinition> {
        self.processors.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn profile() -> NumericalProfile {
        NumericalProfile {
            internal_precision: Precision::Binary32,
            rounding: Rounding::NearestEven,
            fused_operations: FusedOperations::Separate,
            simd: Simd::Scalar,
            subnormals: Subnormals::Preserve,
        }
    }

    fn definition() -> ProcessorDefinition {
        ProcessorDefinition {
            name: "gain".to_string(),
            implementation: Digest::of("euphony test implementation", b"gain v1"),
            parameter_version: 1,
            schema_version: 1,
            profile: profile(),
        }
    }

    #[test]
    fn each_profile_field_changes_the_identity() {
        let base = profile();
        let variants = [
            NumericalProfile {
                internal_precision: Precision::Binary64,
                ..base
            },
            NumericalProfile {
                rounding: Rounding::TowardZero,
                ..base
            },
            NumericalProfile {
                fused_operations: FusedOperations::Fused,
                ..base
            },
            NumericalProfile {
                simd: Simd::Lanes(8),
                ..base
            },
            NumericalProfile {
                subnormals: Subnormals::FlushToZero,
                ..base
            },
        ];
        let mut seen = vec![base.identity()];
        for variant in variants {
            let id = variant.identity();
            assert!(!seen.contains(&id), "{variant:?} reuses an identity");
            seen.push(id);

            let changed = ProcessorDefinition {
                profile: variant,
                ..definition()
            };
            assert_ne!(changed.identity(), definition().identity());
        }
        let lanes = NumericalProfile {
            simd: Simd::Lanes(4),
            ..base
        };
        assert_ne!(lanes.identity(), NumericalProfile { simd: Simd::Lanes(8), ..base }.identity());
    }

    #[test]
    fn the_implementation_changes_the_processor_identity() {
        let changed = ProcessorDefinition {
            implementation: Digest::of("euphony test implementation", b"gain v2"),
            ..definition()
        };
        assert_eq!(changed.profile.identity(), definition().profile.identity());
        assert_ne!(changed.identity(), definition().identity());

        let versions = [
            ProcessorDefinition {
                parameter_version: 2,
                ..definition()
            },
            ProcessorDefinition {
                schema_version: 2,
                ..definition()
            },
        ];
        for changed in versions {
            assert_ne!(changed.identity(), definition().identity());
        }
    }

    #[test]
    fn registration_refuses_mismatched_identities() {
        let mut registry = Registry::new();

        let mut forged = Registration::of(definition());
        forged.definition.profile.rounding = Rounding::TowardZero;
        assert!(matches!(
            registry.register(forged),
            Err(RegistrationError::ProfileMismatch { .. })
        ));

        let mut forged = Registration::of(definition());
        forged.definition.implementation = Digest::of("euphony test implementation", b"other");
        assert!(matches!(
            registry.register(forged),
            Err(RegistrationError::ProcessorMismatch { .. })
        ));

        let id = registry.register(Registration::of(definition())).unwrap();
        assert_eq!(id, definition().identity());
        assert_eq!(registry.get(&id), Some(&definition()));
    }
}
