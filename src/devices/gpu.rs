use ash::vk;
use std::ffi::CStr;

#[derive(Debug, Clone)]
pub struct GpuDevice {
    pub id: usize,
    pub name: String,
    pub vendor: u32,
    pub device: u32,
    pub device_type: vk::PhysicalDeviceType,
}

impl GpuDevice {
    pub fn scan() -> Result<Vec<GpuDevice>, Box<dyn std::error::Error>> {
        unsafe {
            let entry = ash::Entry::load()?;
            let app_info = vk::ApplicationInfo::default().api_version(vk::API_VERSION_1_3);
            let create_info = vk::InstanceCreateInfo::default().application_info(&app_info);
            let instance = entry.create_instance(&create_info, None)?;
            let result = (|| -> Result<Vec<GpuDevice>, vk::Result> {
                let physical_devices = instance.enumerate_physical_devices()?;

                Ok(physical_devices
                    .iter()
                    .enumerate()
                    .map(|(id, &device)| {
                        let properties = instance.get_physical_device_properties(device);
                        let name = CStr::from_ptr(properties.device_name.as_ptr())
                            .to_string_lossy()
                            .into_owned();

                        GpuDevice {
                            id,
                            name,
                            vendor: properties.vendor_id,
                            device: properties.device_id,
                            device_type: properties.device_type,
                        }
                    })
                    .collect())
            })();
            instance.destroy_instance(None);
            Ok(result?)
        }
    }
}
