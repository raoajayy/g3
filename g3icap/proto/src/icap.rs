//! ICAP Protocol Definitions

/// ICAP Methods
#[derive(Debug, Clone, PartialEq)]
pub enum IcapMethod {
    Options,
    Reqmod,
    Respmod,
}

impl std::fmt::Display for IcapMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IcapMethod::Options => write!(f, "OPTIONS"),
            IcapMethod::Reqmod => write!(f, "REQMOD"),
            IcapMethod::Respmod => write!(f, "RESPMOD"),
        }
    }
}

/// ICAP Version
#[derive(Debug, Clone, PartialEq)]
pub struct IcapVersion {
    pub major: u8,
    pub minor: u8,
}

impl Default for IcapVersion {
    fn default() -> Self {
        Self { major: 1, minor: 0 }
    }
}

impl std::fmt::Display for IcapVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ICAP/{}.{}", self.major, self.minor)
    }
}
