use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct Config {
    #[serde(rename = "Audio", alias = "audio")]
    audio: Audio,
    #[serde(rename = "Debug", alias = "debug")]
    debug: Debug,
    #[serde(rename = "GPU", alias = "gpu")]
    gpu: Gpu,
    #[serde(rename = "General", alias = "general")]
    general: General,
    #[serde(rename = "Input", alias = "input")]
    input: Input,
    #[serde(rename = "Log", alias = "log")]
    log: Log,
    #[serde(rename = "Network", alias = "network")]
    network: Network,
    #[serde(rename = "Vulkan", alias = "vulkan")]
    vulkan: Vulkan,
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
struct Audio {
    audio_backend: u32,
    openal_hrtf: u32,
    openal_main_output_device: String,
    openal_mic_device: String,
    openal_output_mode: u32,
    openal_pad_spk_output_device: String,
    sdl_main_output_device: String,
    sdl_mic_device: String,
    sdl_pad_spk_output_device: String,
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
struct Debug {
    config_version: String,
    debug_dump: bool,
    shader_collect: bool,
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
struct Gpu {
    copy_gpu_buffers: bool,
    direct_memory_access_enabled: bool,
    dump_shaders: bool,
    fsr_enabled: bool,
    full_screen: bool,
    full_screen_mode: String,
    hdr_allowed: bool,
    inline_fetch_shader: bool,
    internal_screen_height: u32,
    internal_screen_width: u32,
    null_gpu: bool,
    patch_shaders: bool,
    present_mode: String,
    rcas_attenuation: u32,
    rcas_enabled: bool,
    readback_linear_images_enabled: bool,
    readbacks_mode: u32,
    userfaultfd: bool,
    vblank_frequency: u32,
    window_height: u32,
    window_width: u32,
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
struct InstallDir {
    enabled: bool,
    path: PathBuf,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
struct General {
    addon_install_dir: PathBuf,
    big_picture_folder_depth: u32,
    big_picture_scale: u32,
    connected_to_network: bool,
    console_language: u32,
    dev_kit_mode: bool,
    discord_rpc_enabled: bool,
    enable_upnp: bool,
    extra_dmem_in_mbytes: u32,
    extra_fmem_in_mbytes: u32,
    font_dir: String,
    home_dir: String,
    install_dirs: Vec<InstallDir>,
    neo_mode: bool,
    psn_signed_in: bool,
    redzone_patches: bool,
    shad_net_enabled: bool,
    shadnet_server: String,
    shadnet_webapi_server: String,
    show_fps_counter: bool,
    show_splash: bool,
    signaling_info: String,
    sys_modules_dir: String,
    trophy_notification_duration: f64,
    trophy_notification_side: String,
    trophy_popup_disabled: bool,
    volume_slider: u32,
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
struct Input {
    background_controller_input: bool,
    camera_id: i32,
    cursor_hide_timeout: u32,
    cursor_state: u32,
    default_controller_id: String,
    ime_accessibility_enabled: bool,
    ime_url_mail_short_panel: bool,
    is_circle_enter: bool,
    motion_controls_enabled: bool,
    special_pad_class: u32,
    usb_device_backend: u32,
    use_keyboard_as_keyboard: bool,
    use_mice_as_mice: bool,
    use_special_pad: bool,
    use_unified_input_config: bool,
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
struct Log {
    append: bool,
    enable: bool,
    filter: String,
    flush_level: String,
    max_skip_duration: u32,
    separate: bool,
    size_limit: u64,
    skip_duplicate: bool,
    sync: bool,
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
struct Network {
    connected_to_network: bool,
    disable_https: bool,
    enable_upnp: bool,
    p2p_port: u32,
    shad_net_enabled: bool,
    shadnet_server: String,
    shadnet_webapi_server: String,
    signaling_info: String,
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
struct Vulkan {
    gpu_id: i32,
    pipeline_cache_archived: bool,
    pipeline_cache_enabled: bool,
    renderdoc_enabled: bool,
    vkcrash_diagnostic_enabled: bool,
    vkguest_markers: bool,
    vkhost_markers: bool,
    vkvalidation_core_enabled: bool,
    vkvalidation_enabled: bool,
    vkvalidation_gpu_enabled: bool,
    vkvalidation_sync_enabled: bool,
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
