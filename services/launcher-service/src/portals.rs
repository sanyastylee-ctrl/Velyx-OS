#[derive(Clone, Debug)]
pub struct FilePortalContract {
    pub portal_id: String,
    pub requested_mode: String,
}

#[derive(Clone, Debug)]
pub struct DevicePortalContract {
    pub portal_id: String,
    pub device_class: String,
}

pub fn future_file_portal_contract() -> FilePortalContract {
    FilePortalContract {
        portal_id: "future.file.portal".to_string(),
        requested_mode: "not_implemented".to_string(),
    }
}

pub fn future_device_portal_contract() -> DevicePortalContract {
    DevicePortalContract {
        portal_id: "future.device.portal".to_string(),
        device_class: "not_implemented".to_string(),
    }
}
