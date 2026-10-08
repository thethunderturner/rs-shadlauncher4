use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
    audio: Audio,
    debug: Debug,
    gpu: Gpu,
    general: General,
    input: Input,
    log: Log,
    network: Network,
    vulkan: Vulkan,
}
#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
struct Debug {
    config_version: String,
    debug_dump: bool,
    shader_collect: bool,
}

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
struct InstallDir {
    enabled: bool,
    path: String,
}

#[derive(Debug, Deserialize)]
struct General {
    addon_install_dir: String,
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

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
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
