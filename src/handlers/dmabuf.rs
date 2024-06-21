use smithay_client_toolkit::dmabuf::DmabufHandler;

use crate::app_data::AppData;

impl DmabufHandler for AppData {
    fn dmabuf_state(&mut self) -> &mut smithay_client_toolkit::dmabuf::DmabufState {
        &mut self.dmabuf_state
    }
    fn dmabuf_feedback(
        &mut self,
        _conn: &smithay_client_toolkit::reexports::client::Connection,
        _qh: &smithay_client_toolkit::reexports::client::QueueHandle<Self>,
        _proxy: &smithay_client_toolkit::reexports::protocols::wp::linux_dmabuf::zv1::client::zwp_linux_dmabuf_feedback_v1::ZwpLinuxDmabufFeedbackV1,
        feedback: smithay_client_toolkit::dmabuf::DmabufFeedback,
    ) {
        tracing::trace!("dmabuf_feedback");
        self.feedback = Some(feedback);
    }

    fn created(
        &mut self,
        _conn: &smithay_client_toolkit::reexports::client::Connection,
        _qh: &smithay_client_toolkit::reexports::client::QueueHandle<Self>,
        _params: &smithay_client_toolkit::reexports::protocols::wp::linux_dmabuf::zv1::client::zwp_linux_buffer_params_v1::ZwpLinuxBufferParamsV1,
        _buffer: smithay_client_toolkit::reexports::client::protocol::wl_buffer::WlBuffer,
    ) {
        tracing::trace!("dmabuf_created");
    }

    fn failed(
        &mut self,
        _conn: &smithay_client_toolkit::reexports::client::Connection,
        _qh: &smithay_client_toolkit::reexports::client::QueueHandle<Self>,
        params: &smithay_client_toolkit::reexports::protocols::wp::linux_dmabuf::zv1::client::zwp_linux_buffer_params_v1::ZwpLinuxBufferParamsV1,
    ) {
        tracing::trace!("dmabuf_failed: {:#?}", params);
    }

    fn released(
        &mut self,
        _conn: &smithay_client_toolkit::reexports::client::Connection,
        _qh: &smithay_client_toolkit::reexports::client::QueueHandle<Self>,
        _buffer: &smithay_client_toolkit::reexports::client::protocol::wl_buffer::WlBuffer,
    ) {
    }
}

smithay_client_toolkit::delegate_dmabuf!(AppData);
