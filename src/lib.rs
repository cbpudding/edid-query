use core::slice;
use edid_rs::EDID;
use raw_window_handle::RawDisplayHandle;
use std::{ffi::CString, io::Cursor};

#[cfg(unix)]
use x11::{
    xlib::{self, Display},
    xrandr,
};

/// Queries the given monitor's EDID, if available.
pub fn query_edid(handle: RawDisplayHandle) -> Vec<EDID> {
    match handle {
        #[cfg(unix)]
        RawDisplayHandle::Xlib(raw) => {
            let mut edids = Vec::new();
            let display = raw.display as *mut Display;
            let screen = unsafe { xlib::XScreenOfDisplay(display, raw.screen) };
            let resources = unsafe { xrandr::XRRGetScreenResources(display, (*screen).root) };
            let output_ids =
                unsafe { slice::from_raw_parts((*resources).outputs, (*resources).noutput as _) };
            let edid_atom_name = CString::new(xrandr::RR_PROPERTY_RANDR_EDID).unwrap();
            let edid_atom =
                unsafe { xlib::XInternAtom(display, edid_atom_name.as_ptr() as _, true as _) };
            for output_id in output_ids {
                let mut actual_type = 0u64;
                let mut actual_format = 0i32;
                let mut nitems = 0u64;
                let mut bytes_after = 0u64;
                let mut edid = Vec::with_capacity(128);
                unsafe {
                    xrandr::XRRGetOutputProperty(
                        display,
                        *output_id,
                        edid_atom,
                        0,
                        128,
                        false as _,
                        false as _,
                        19,
                        &mut actual_type,
                        &mut actual_format,
                        &mut nitems,
                        &mut bytes_after,
                        &mut edid.as_mut_ptr(),
                    )
                };
                edids.push(edid_rs::parse(&mut Cursor::new(edid)).unwrap());
            }
            edids
        }
        _ => vec![],
    }
}
