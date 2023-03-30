use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use esp_idf_hal::task::thread::ThreadSpawnConfiguration;
use esp_idf_svc::systime::EspSystemTime;
use esp_idf_svc::log::EspLogger;
use log::info;
fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    esp_idf_svc::log::EspLogger::initialize_default();
    const LOGGER: EspLogger = EspLogger;
    LOGGER.set_target_level("*", log::LevelFilter::Trace);
    LOGGER.set_target_level("rust-logging", log::LevelFilter::Trace);

    ThreadSpawnConfiguration {
        name: Some("mid-lvl-thread".as_bytes()),
        stack_size: 10000,
        priority: 15,
        ..Default::default()
    };

    let _thread_1 = std::thread::Builder::new()
    .spawn(move || {
        loop {
            let time = EspSystemTime;
            let mut now = time.now();
            info!("Current time: {:?}", now);
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    })
    .unwrap();

    _thread_1.join().unwrap();
    std::thread::sleep(std::time::Duration::from_millis(10000));
    loop {
        std::thread::sleep(std::time::Duration::from_millis(1000));
        info!("MAIN THREAD waker");
    }
}
