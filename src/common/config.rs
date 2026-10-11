use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::{fs, io};

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Config {
    #[serde(rename = "Audio", alias = "audio")]
    pub audio: Audio,
    #[serde(rename = "Frontend", alias = "frontend")]
    pub frontend: Frontend,
    #[serde(rename = "Debug", alias = "debug")]
    pub debug: Debug,
    #[serde(rename = "GPU", alias = "gpu")]
    pub gpu: Gpu,
    #[serde(rename = "General", alias = "general")]
    pub general: General,
    #[serde(rename = "Input", alias = "input")]
    pub input: Input,
    #[serde(rename = "Log", alias = "log")]
    pub log: Log,
    #[serde(rename = "Network", alias = "network")]
    pub network: Network,
    #[serde(rename = "Vulkan", alias = "vulkan")]
    pub vulkan: Vulkan,
}

// QT saves these in a .ini file. I can try saving it in the config.json, not sure if this will cause issues down the road
#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct Frontend {
    pub show_background_image: bool,
    pub background_image_opacity: u32,
    pub play_background_music: bool,
    pub background_music_volume: u32,
    pub show_compatibility: bool,
    pub check_compatibility_at_startup: bool,
    pub check_for_updates: bool,
    pub show_change_log: bool,
    pub home_tab: String,
}

impl Default for Frontend {
    fn default() -> Self {
        Self {
            show_background_image: false,
            background_image_opacity: 100,
            play_background_music: false,
            background_music_volume: 100,
            show_compatibility: false,
            check_compatibility_at_startup: false,
            check_for_updates: false,
            show_change_log: false,
            home_tab: "General".to_owned(),
        }
    }
}

impl Config {
    pub fn game_paths() -> io::Result<Vec<PathBuf>> {
        let path = super::user_directory()?.join("config.json");
        let config: Self = serde_json::from_slice(&fs::read(path)?).map_err(io::Error::other)?;
        Ok(config.enabled_game_paths())
    }

    fn enabled_game_paths(self) -> Vec<PathBuf> {
        self.general
            .install_dirs
            .into_iter()
            .filter(|dir| dir.enabled && !dir.path.as_os_str().is_empty())
            .map(|dir| dir.path)
            .collect()
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct Audio {
    pub audio_backend: u32,
    pub openal_hrtf: u32,
    pub openal_main_output_device: String,
    pub openal_mic_device: String,
    pub openal_output_mode: u32,
    pub openal_pad_spk_output_device: String,
    pub sdl_main_output_device: String,
    pub sdl_mic_device: String,
    pub sdl_pad_spk_output_device: String,
}

impl Default for Audio {
    fn default() -> Self {
        Self {
            audio_backend: 0,
            openal_hrtf: 0,
            openal_main_output_device: String::from("Default Device"),
            openal_mic_device: String::from("Default Device"),
            openal_output_mode: 0,
            openal_pad_spk_output_device: String::from("Default Device"),
            sdl_main_output_device: String::from("Default Device"),
            sdl_mic_device: String::from("Default Device"),
            sdl_pad_spk_output_device: String::from("Default Device"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct Debug {
    pub config_version: String,
    pub debug_dump: bool,
    pub shader_collect: bool,
}

impl Default for Debug {
    fn default() -> Self {
        Self {
            config_version: String::new(),
            debug_dump: false,
            shader_collect: false,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct Gpu {
    pub copy_gpu_buffers: bool,
    pub direct_memory_access_enabled: bool,
    pub dump_shaders: bool,
    pub fsr_enabled: bool,
    pub full_screen: bool,
    pub full_screen_mode: String,
    pub hdr_allowed: bool,
    pub inline_fetch_shader: bool,
    pub internal_screen_height: u32,
    pub internal_screen_width: u32,
    pub null_gpu: bool,
    pub patch_shaders: bool,
    pub present_mode: String,
    pub rcas_attenuation: u32,
    pub rcas_enabled: bool,
    pub readback_linear_images_enabled: bool,
    pub readbacks_mode: u32,
    pub userfaultfd: bool,
    pub vblank_frequency: u32,
    pub window_height: u32,
    pub window_width: u32,
}

impl Default for Gpu {
    fn default() -> Self {
        Self {
            copy_gpu_buffers: false,
            direct_memory_access_enabled: false,
            dump_shaders: false,
            fsr_enabled: false,
            full_screen: false,
            full_screen_mode: String::from("Windowed"),
            hdr_allowed: false,
            inline_fetch_shader: false,
            internal_screen_height: 720,
            internal_screen_width: 1280,
            null_gpu: false,
            patch_shaders: false,
            present_mode: String::from("Mailbox"),
            rcas_attenuation: 250,
            rcas_enabled: true,
            readback_linear_images_enabled: false,
            readbacks_mode: 0,
            userfaultfd: false,
            vblank_frequency: 60,
            window_height: 720,
            window_width: 1280,
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct InstallDir {
    pub enabled: bool,
    pub path: PathBuf,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct General {
    pub addon_install_dir: PathBuf,
    pub big_picture_folder_depth: u32,
    pub big_picture_scale: u32,
    pub connected_to_network: bool,
    pub console_language: u32,
    pub dev_kit_mode: bool,
    pub discord_rpc_enabled: bool,
    pub enable_upnp: bool,
    pub extra_dmem_in_mbytes: u32,
    pub extra_fmem_in_mbytes: u32,
    pub font_dir: String,
    pub home_dir: String,
    pub install_dirs: Vec<InstallDir>,
    pub neo_mode: bool,
    pub psn_signed_in: bool,
    pub redzone_patches: bool,
    pub shad_net_enabled: bool,
    pub shadnet_server: String,
    pub shadnet_webapi_server: String,
    pub show_fps_counter: bool,
    pub show_splash: bool,
    pub signaling_info: String,
    pub sys_modules_dir: String,
    pub trophy_notification_duration: f64,
    pub trophy_notification_side: String,
    pub trophy_popup_disabled: bool,
    pub volume_slider: u32,
}

impl Default for General {
    fn default() -> Self {
        Self {
            addon_install_dir: PathBuf::new(),
            big_picture_folder_depth: 2,
            big_picture_scale: 1000,
            connected_to_network: false,
            console_language: 1,
            dev_kit_mode: false,
            discord_rpc_enabled: false,
            enable_upnp: true,
            extra_dmem_in_mbytes: 0,
            extra_fmem_in_mbytes: 0,
            font_dir: String::new(),
            home_dir: String::new(),
            install_dirs: Vec::new(),
            neo_mode: false,
            psn_signed_in: false,
            redzone_patches: false,
            shad_net_enabled: false,
            shadnet_server: String::from("srv.shadps4.net:31313"),
            shadnet_webapi_server: String::from("http://srv.shadps4.net:31315"),
            show_fps_counter: false,
            show_splash: false,
            signaling_info: String::new(),
            sys_modules_dir: String::new(),
            trophy_notification_duration: 6.0,
            trophy_notification_side: String::from("right"),
            trophy_popup_disabled: false,
            volume_slider: 100,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct Input {
    pub background_controller_input: bool,
    pub camera_id: i32,
    pub cursor_hide_timeout: u32,
    pub cursor_state: u32,
    pub default_controller_id: String,
    pub ime_accessibility_enabled: bool,
    pub ime_url_mail_short_panel: bool,
    pub is_circle_enter: bool,
    pub motion_controls_enabled: bool,
    pub special_pad_class: u32,
    pub usb_device_backend: u32,
    pub use_keyboard_as_keyboard: bool,
    pub use_mice_as_mice: bool,
    pub use_special_pad: bool,
    pub use_unified_input_config: bool,
}

impl Default for Input {
    fn default() -> Self {
        Self {
            background_controller_input: false,
            camera_id: -1,
            cursor_hide_timeout: 5,
            cursor_state: 1,
            default_controller_id: String::new(),
            ime_accessibility_enabled: false,
            ime_url_mail_short_panel: false,
            is_circle_enter: false,
            motion_controls_enabled: true,
            special_pad_class: 1,
            usb_device_backend: 0,
            use_keyboard_as_keyboard: false,
            use_mice_as_mice: false,
            use_special_pad: false,
            use_unified_input_config: true,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct Log {
    pub append: bool,
    pub enable: bool,
    pub filter: String,
    pub flush_level: String,
    pub max_skip_duration: u32,
    pub separate: bool,
    pub size_limit: u64,
    pub skip_duplicate: bool,
    pub sync: bool,
}

impl Default for Log {
    fn default() -> Self {
        Self {
            append: false,
            enable: true,
            filter: String::new(),
            flush_level: String::new(),
            max_skip_duration: 5000,
            separate: true,
            size_limit: 104857600,
            skip_duplicate: true,
            sync: true,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct Network {
    pub connected_to_network: bool,
    pub disable_https: bool,
    pub enable_upnp: bool,
    pub p2p_port: u32,
    pub shad_net_enabled: bool,
    pub shadnet_server: String,
    pub shadnet_webapi_server: String,
    pub signaling_info: String,
}

impl Default for Network {
    fn default() -> Self {
        Self {
            connected_to_network: false,
            disable_https: false,
            enable_upnp: true,
            p2p_port: 3658,
            shad_net_enabled: false,
            shadnet_server: String::from("srv.shadps4.net:31313"),
            shadnet_webapi_server: String::from("http://srv.shadps4.net:31315"),
            signaling_info: String::new(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct Vulkan {
    pub gpu_id: i32,
    pub pipeline_cache_archived: bool,
    pub pipeline_cache_enabled: bool,
    pub renderdoc_enabled: bool,
    pub vkcrash_diagnostic_enabled: bool,
    pub vkguest_markers: bool,
    pub vkhost_markers: bool,
    pub vkvalidation_core_enabled: bool,
    pub vkvalidation_enabled: bool,
    pub vkvalidation_gpu_enabled: bool,
    pub vkvalidation_sync_enabled: bool,
}

impl Default for Vulkan {
    fn default() -> Self {
        Self {
            gpu_id: -1,
            pipeline_cache_archived: false,
            pipeline_cache_enabled: false,
            renderdoc_enabled: false,
            vkcrash_diagnostic_enabled: false,
            vkguest_markers: false,
            vkhost_markers: false,
            vkvalidation_core_enabled: true,
            vkvalidation_enabled: false,
            vkvalidation_gpu_enabled: false,
            vkvalidation_sync_enabled: false,
        }
    }
}
