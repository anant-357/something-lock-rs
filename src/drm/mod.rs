pub use drm::control::Device as ControlDevice;
pub use drm::Device;
use std::{fs::File, os::fd::AsFd};

pub struct Card(File);

impl AsFd for Card {
    fn as_fd(&self) -> std::os::unix::prelude::BorrowedFd<'_> {
        self.0.as_fd()
    }
}

impl Device for Card {}

impl ControlDevice for Card {}

impl Card {
    pub fn open(path: &str) -> Self {
        let mut options = std::fs::OpenOptions::new();
        options.read(true);
        options.write(true);
        Card(options.open(path).unwrap())
    }

    pub fn open_global() -> Self {
        Self::open("/dev/dri/card1")
    }
}

pub mod capabilities {
    use drm::ClientCapability as CC;
    pub const CLIENT_CAP_ENUMS: &[CC] = &[CC::Stereo3D, CC::UniversalPlanes, CC::Atomic];

    use drm::DriverCapability as DC;
    pub const DRIVER_CAP_ENUMS: &[DC] = &[
        DC::DumbBuffer,
        DC::VBlankHighCRTC,
        DC::DumbPreferredDepth,
        DC::DumbPreferShadow,
        DC::Prime,
        DC::MonotonicTimestamp,
        DC::ASyncPageFlip,
        DC::CursorWidth,
        DC::CursorHeight,
        DC::AddFB2Modifiers,
        DC::PageFlipTarget,
        DC::CRTCInVBlankEvent,
        DC::SyncObj,
        DC::TimelineSyncObj,
    ];
}
