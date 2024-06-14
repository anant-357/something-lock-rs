use smithay_client_toolkit::shm::ShmHandler;

use crate::app_data::AppData;

impl ShmHandler for AppData {
    fn shm_state(&mut self) -> &mut smithay_client_toolkit::shm::Shm {
        &mut self.shm
    }
}

smithay_client_toolkit::delegate_shm!(AppData);
