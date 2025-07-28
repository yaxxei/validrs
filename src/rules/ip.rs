use std::{borrow::Cow, net::IpAddr, str::FromStr as _};

use crate::error::{Error, Result};

pub enum IpVersions {
    V4,
    V6,
}

// TODO: validate loopback and multicast
pub trait ValidateIp {
    fn validate_ip(&self, version: Option<IpVersions>, msg: Option<String>) -> Result<()> {
        let err = msg.map(Error::Custom).unwrap_or(Error::Ip);
        let Some(ip_str) = self.ip_str() else {
            return Err(err);
        };

        match IpAddr::from_str(ip_str) {
            Ok(ip) => match version {
                Some(IpVersions::V4) if !ip.is_ipv4() => Err(err),
                Some(IpVersions::V6) if !ip.is_ipv6() => Err(err),
                _ => Ok(()),
            },
            Err(_) => Err(err),
        }
    }

    fn ip_str(&self) -> Option<&str>;
}

impl ValidateIp for String {
    fn ip_str(&self) -> Option<&str> {
        Some(self.as_str())
    }
}

impl ValidateIp for str {
    fn ip_str(&self) -> Option<&str> {
        Some(self)
    }
}

impl ValidateIp for &str {
    fn ip_str(&self) -> Option<&str> {
        Some(self)
    }
}

impl ValidateIp for Cow<'_, str> {
    fn ip_str(&self) -> Option<&str> {
        Some(self.as_ref())
    }
}

impl<T: ValidateIp> ValidateIp for Option<T> {
    fn ip_str(&self) -> Option<&str> {
        let Some(s) = self else {
            return None;
        };
        T::ip_str(s)
    }
}

#[cfg(test)]
mod tests {
    use super::ValidateIp;

    #[test]
    fn test_validate_ip() {
        let tests = vec![
            "1.1.1.1",
            "255.0.0.0",
            "0.0.0.0",
            "2a02::223:6cff:fe8a:2e8a",
            "::ffff:254.42.16.14",
        ];

        for input in tests {
            println!("{input}");
            assert!(input.validate_ip(None, None).is_ok());
        }
    }
}
