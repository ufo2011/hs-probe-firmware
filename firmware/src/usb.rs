use stm32ral::{
    otg_fs_global,
    otg_fs_device,
    otg_fs_pwrclk
};
use crate::app::Request;
use hs_probe_bsp::rcc::Clocks;
use hs_probe_bsp::otg_fs::{UsbBusType, UsbBus};
use usb_device::prelude::*;
use usb_device::bus::UsbBusAllocator;
use usbd_serial::SerialPort;


struct UninitializedUSB {
    global: otg_fs_global::Instance,
    device: otg_fs_device::Instance,
    pwrclk: otg_fs_pwrclk::Instance,
}

struct InitializedUSB {
    device: UsbDevice<'static, UsbBusType>,
    serial: SerialPort<'static, UsbBusType>,
}

enum State {
    Uninitialized(UninitializedUSB),
    Initialized(InitializedUSB),
    Initializing,
}

static mut EP_MEMORY: [u32; 1024] = [0; 1024];
static mut USB_BUS: Option<UsbBusAllocator<UsbBusType>> = None;

/// USB stack interface
pub struct USB {
    state: State,
}

impl USB {
    /// Create a new USB object from the peripheral instance
    pub fn new(
        global: otg_fs_global::Instance,
        device: otg_fs_device::Instance,
        pwrclk: otg_fs_pwrclk::Instance,
    ) -> Self {
        let usb = UninitializedUSB {
            global,
            device,
            pwrclk
        };
        USB {
            state: State::Uninitialized(usb)
        }
    }

    /// Initialise the USB peripheral ready to start processing packets
    pub fn setup(&mut self, clocks: &Clocks) {
        let state = core::mem::replace(&mut self.state, State::Initializing);
        if let State::Uninitialized(usb) = state {
            cortex_m::interrupt::free(|_| unsafe {
                let usb = hs_probe_bsp::otg_fs::USB {
                    usb_global: usb.global,
                    usb_device: usb.device,
                    usb_pwrclk: usb.pwrclk,
                    hclk: clocks.hclk()
                };

                let usb_bus = UsbBus::new(usb, &mut EP_MEMORY);
                USB_BUS = Some(usb_bus);
                let usb_bus = USB_BUS.as_ref().unwrap();

                let serial = SerialPort::new(&usb_bus);

                let device = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x1209, 0xFF50))
                    .manufacturer("AGG")
                    .product("FFP r1 with CMSIS-DAP Support")
                    .serial_number(stm32_device_signature::device_id_hex())
                    .device_class(0)
                    .build();

                let usb = InitializedUSB {
                    device,
                    serial,
                };
                self.state = State::Initialized(usb)
            });
        } else {
            panic!("Invalid state");
        }
    }

    /// Process a pending USB interrupt.
    ///
    /// Call this function when a USB interrupt occurs.
    ///
    /// Returns Some(Request) if a new request has been received
    /// from the host.
    ///
    /// This function will clear the interrupt bits of all interrupts
    /// it processes; if any are unprocessed the USB interrupt keeps
    /// triggering until all are processed.
    pub fn interrupt(&mut self) -> Option<Request> {
        todo!()
    }

    /// Transmit a DAP report back over the DAPv1 HID interface
    pub fn dap1_reply(&mut self, data: &[u8]) {
        todo!()
    }

    /// Transmit a DAP report back over the DAPv2 bulk interface
    pub fn dap2_reply(&mut self, data: &[u8]) {
        todo!()
    }

    /// Check if SWO endpoint is currently busy transmitting data
    pub fn dap2_swo_is_busy(&self) -> bool {
        todo!()
    }

    /// Transmit SWO streaming data back over the DAPv2 bulk interface
    pub fn dap2_stream_swo(&mut self, data: &[u8]) {
        todo!()
    }

    /// Indicate we can currently receive DAP requests
    pub fn dap_enable(&mut self) {
        todo!()
    }

    /// Indicate we cannot currently receive DAP requests
    pub fn dap_disable(&mut self) {
        todo!()
    }
}