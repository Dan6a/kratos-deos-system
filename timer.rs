// Kratos OS — HAL Timer
// BCM2835 System Timer на RPi Zero 2W
//
// System Timer: счётчик 64 бит, тикает на 1 МГц
// Базовый адрес: 0x3F003000

const TIMER_BASE: usize = 0x3F00_3000;

const TIMER_CS:  usize = 0x00;  // Control/Status
const TIMER_CLO: usize = 0x04;  // Counter Lower 32 bits
const TIMER_CHI: usize = 0x08;  // Counter Higher 32 bits

fn read(offset: usize) -> u32 {
    unsafe { ((TIMER_BASE + offset) as *const u32).read_volatile() }
}

/// Читаем текущее значение счётчика (64 бит, 1 МГц)
/// Переполняется через ~584542 лет
pub fn now_us() -> u64 {
    // Читаем CHI, CLO, CHI — защита от переполнения между чтениями
    let hi1 = read(TIMER_CHI) as u64;
    let lo  = read(TIMER_CLO) as u64;
    let hi2 = read(TIMER_CHI) as u64;

    if hi1 != hi2 {
        // Переполнение CLO произошло между чтениями — берём hi2
        (hi2 << 32) | lo
    } else {
        (hi1 << 32) | lo
    }
}

/// Задержка в микросекундах (активное ожидание)
/// Аксиома 3: минимум потерь — не используем прерывания для коротких задержек
pub fn delay_us(us: u64) {
    let end = now_us().wrapping_add(us);
    while now_us() < end {}
}

/// Задержка в миллисекундах
pub fn delay_ms(ms: u64) {
    delay_us(ms * 1000);
}
