use windows_core::PCWSTR;

use crate::bindings::{
    AdjustTokenPrivileges, CloseHandle, ERROR_NOT_ALL_ASSIGNED, GetCurrentProcess, GetLastError, HANDLE,
    LUID_AND_ATTRIBUTES, LookupPrivilegeValueW, OpenProcessToken, SE_PRIVILEGE_ENABLED, TOKEN_ADJUST_PRIVILEGES,
    TOKEN_PRIVILEGES, TOKEN_QUERY,
};
use crate::error::{Error, Result};

/// Turns on a privilege the process token already holds; fails when it
/// holds none of this name. `AdjustTokenPrivileges` tells that only through
/// the last error, while returning success.
pub(crate) fn enable(name: &str) -> Result<()> {
    let mut token = HANDLE::default();
    let access = (TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY) as u32;
    if !unsafe { OpenProcessToken(GetCurrentProcess(), access, &mut token) }.as_bool() {
        return Err(Error::new("OpenProcessToken", unsafe { GetLastError() }));
    }
    let adjusted = adjust(token, name);
    let _ = unsafe { CloseHandle(token) };
    adjusted
}

fn adjust(token: HANDLE, name: &str) -> Result<()> {
    let name: Vec<u16> = name.encode_utf16().chain(Some(0)).collect();
    let mut luid = Default::default();
    if !unsafe { LookupPrivilegeValueW(PCWSTR::null(), PCWSTR(name.as_ptr()), &mut luid) }.as_bool() {
        return Err(Error::new("LookupPrivilegeValueW", unsafe { GetLastError() }));
    }
    let privileges = TOKEN_PRIVILEGES {
        PrivilegeCount: 1,
        Privileges: [LUID_AND_ATTRIBUTES {
            Luid: luid,
            Attributes: SE_PRIVILEGE_ENABLED as u32,
        }],
    };
    if !unsafe { AdjustTokenPrivileges(token, false, Some(&privileges), 0, None, None) }.as_bool() {
        return Err(Error::new("AdjustTokenPrivileges", unsafe { GetLastError() }));
    }
    let last = unsafe { GetLastError() };
    if last == ERROR_NOT_ALL_ASSIGNED as u32 {
        return Err(Error::new("AdjustTokenPrivileges", last));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const ERROR_NOT_ALL_ASSIGNED: u32 = 1300;
    const ERROR_NO_SUCH_PRIVILEGE: u32 = 1313;

    #[test]
    fn a_privilege_the_token_does_not_hold_is_refused() {
        let refused = enable("SeTcbPrivilege");
        assert_eq!(refused.map_err(|error| error.code), Err(ERROR_NOT_ALL_ASSIGNED));
    }

    #[test]
    fn a_privilege_of_no_such_name_is_refused() {
        let refused = enable("SeNoSuchPrivilege");
        assert_eq!(refused.map_err(|error| error.code), Err(ERROR_NO_SUCH_PRIVILEGE));
    }
}
