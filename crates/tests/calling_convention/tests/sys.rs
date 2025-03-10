use windows_sys::{Win32::Foundation::*, Win32::Networking::Ldap::*, Win32::System::SystemInformation::*};

#[test]
fn calling_convention() {
    unsafe {
        // This function requires cdecl on x86.
        assert_eq!(ERROR_BUSY, LdapMapErrorToWin32(LDAP_BUSY));

        // This function requires stdcall on x86.
        GetTickCount();
    }
}

#[test]
#[cfg(windows_raw_dylib)]
fn raw_dylib() {}
