use std::time::Duration;

pub mod kraken;
pub mod smart_device;
use hidapi::{HidApi, HidDevice, HidResult};

pub const NZXT_PID: u16 = 0x1e71;

pub struct DeviceManager {
  hid: HidApi,
}

impl DeviceManager {
  pub fn new() -> Result<Self, String> {
    match HidApi::new() {
      // for device in api.devices() {
      //     println!("{:#?}", device);
      // }
      Ok(hid) => Ok(DeviceManager { hid }),
      Err(err) => Err(err.to_string()),
    }
    // match libusb::Context::new() {
    //   Ok(context) => Ok(DeviceManager { context }),
    //   Err(err) => Err(err.strerror()),
    // }
  }

  pub fn all(&self) -> Vec<Result<UsbDevice, String>> {
    let mut devices = vec![];
    for device in self.hid.devices().iter() {
      if device.vendor_id == NZXT_PID {
        let usb_device: Result<UsbDevice, String> = device.open_device(&self.hid).try_into();

        match usb_device {
          Ok(dev) => devices.push(Ok(dev)),
          Err(msg) => devices.push(Err(format!("Couldn't open device..."))),
        }
      }
    }
    // for device in self.context.devices().unwrap().iter() {
    //   let device_desc = device.device_descriptor().unwrap();
    //   if device_desc.vendor_id() == NZXT_PID {
    //     let bus = device.bus_number();
    //     let addr = device.address();
    //     let usb_device: Result<UsbDevice, &str> = device.try_into();
    //     match usb_device {
    //       Ok(dev) => devices.push(Ok(dev)),
    //       Err(msg) => devices.push(Err(format!("Couldn't open device at {:03}:{:03}: {}", bus, addr, msg))),
    //     }
    //   }
    // }
    devices
  }
}

pub trait Device {
  fn print_info(&self) -> ();
  fn device_id(&self) -> u16;
}

pub struct UsbDevice {
  // device: libusb::Device,
  // handle: libusb::DeviceHandle,
  // language: libusb::Language,
  // timeout: Duration,
  device: HidDevice,
}

impl Device for UsbDevice {
  fn print_info(&self) -> () {
    // let device_desc = self.device.device_descriptor().unwrap();

    // match device_desc.product_id() {
    //   kraken::X62::PRODUCT_ID => println!(
    //     "Bus {:03} Device {:03}: NZXT Kraken X62 [s/n: {}]",
    //     self.device.bus_number(),
    //     self.device.address(),
    //     self
    //       .handle
    //       .read_serial_number_string(self.language, &device_desc, self.timeout)
    //       .unwrap_or("unknown".to_owned()),
    //   ),
    //   smart_device::PRODUCT_ID => println!(
    //     "Bus {:03} Device {:03}: NZXT Smart Device [s/n: {}]",
    //     self.device.bus_number(),
    //     self.device.address(),
    //     self
    //       .handle
    //       .read_serial_number_string(self.language, &device_desc, self.timeout)
    //       .unwrap_or("unknown".to_owned()),
    //   ),
    //   _ => println!(
    //     "Bus {:03} Device {:03}: Unknown NZXT Device: {:04x} (product: {})",
    //     self.device.bus_number(),
    //     self.device.address(),
    //     device_desc.product_id(),
    //     self
    //       .handle
    //       .read_product_string(self.language, &device_desc, self.timeout)
    //       .unwrap_or("unidentified".to_owned())
    //   ),
    // }
  }

  fn device_id(&self) -> u16 {
    return 1;
    // self.device.device_descriptor().unwrap().product_id()
  }
}

trait TryInto<T> {
  fn try_into(self: Self) -> Result<T, String>;
}

impl TryInto<UsbDevice> for HidResult<HidDevice> {
  fn try_into(self) -> Result<UsbDevice, String> {
    match self {
      Ok(device) => Ok(UsbDevice { device }),
      Err(err) => Err(err.to_string()),
    }
  }
}

// impl TryInto<UsbDevice> for libusb::Device<'a> {
//   fn try_into(self) -> Result<UsbDevice, &'static str> {
//     let timeout = Duration::from_millis(200);
//     match self.open() {
//       Ok(handle) => match handle.read_languages(timeout) {
//         Ok(l) => {
//           if l.len() > 0 {
//             Ok(UsbDevice {
//               device: self,
//               handle,
//               language: l[0],
//               timeout,
//             })
//           } else {
//             Err("No language")
//           }
//         },
//         Err(err) => Err(err.strerror()),
//       },
//       Err(err) => Err(err.strerror()),
//     }
//   }
// }
