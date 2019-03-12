use std::time::Duration;

pub mod kraken;
pub mod smart_device;

pub const NZXT_PID: u16 = 0x1e71;

pub trait Device {
  fn print_info(&self) -> ();
}

pub struct UsbDevice<'a> {
  device: libusb::Device<'a>,
  handle: libusb::DeviceHandle<'a>,
  language: libusb::Language,
  timeout: Duration,
}

impl<'a> UsbDevice<'a> {
  pub fn all(context: &libusb::Context) -> Vec<UsbDevice> {
    let mut devices = vec![];
    for device in context.devices().unwrap().iter() {
      let device_desc = device.device_descriptor().unwrap();
      if device_desc.vendor_id() == NZXT_PID {
        let usb_device: Result<UsbDevice, &str> = device.try_into();
        match usb_device {
          Ok(dev) => devices.push(dev),
          Err(msg) => println!("Error: {}", msg),
        }
      }
    }
    devices
  }
}

impl<'a> Device for UsbDevice<'a> {
  fn print_info(&self) -> () {
    let device_desc = self.device.device_descriptor().unwrap();

    match device_desc.product_id() {
      kraken::X62::PRODUCT_ID => println!(
        "Bus {:03} Device {:03}: NZXT Kraken X62",
        self.device.bus_number(),
        self.device.address(),
      ),
      smart_device::PRODUCT_ID => println!(
        "Bus {:03} Device {:03}: NZXT Smart Device",
        self.device.bus_number(),
        self.device.address(),
      ),
      _ => println!(
        "Bus {:03} Device {:03}: Unknown NZXT Device: {:04x} (product: {})",
        self.device.bus_number(),
        self.device.address(),
        device_desc.product_id(),
        self
          .handle
          .read_product_string(self.language, &device_desc, self.timeout)
          .unwrap_or("unidentified".to_owned())
      ),
    }
  }
}

trait TryInto<T> {
  fn try_into(self: Self) -> Result<T, &'static str>;
}

impl<'a> TryInto<UsbDevice<'a>> for libusb::Device<'a> {
  fn try_into(self) -> Result<UsbDevice<'a>, &'static str> {
    let timeout = Duration::from_millis(200);
    match self.open() {
      Ok(handle) => match handle.read_languages(timeout) {
        Ok(l) => {
          if l.len() > 0 {
            Ok(UsbDevice {
              device: self,
              handle,
              language: l[0],
              timeout,
            })
          } else {
            Err("No language")
          }
        },
        Err(err) => Err(err.strerror()),
      },
      Err(err) => Err(err.strerror()),
    }
  }
}