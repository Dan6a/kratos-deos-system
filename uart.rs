// Kratos OS — HAL UART
// PL011 UART на RPi Zero 2W
//
// RPi Zero 2W (BCM2710):
//   Базовый адрес периферии: 0x3F000000
//   PL011 UART: 0x3F201000
//
// Аксиома 1: весь вывод через UART — прозрачность

// Базовый адрес PL011 UART для RPi Zero 2W
const UART_BASE: usize = 0x3F20_1000;

// Регистры PL011 (смещения от UART_BASE)
const UART_DR:   usize = 0x00;  // Data Register
const UART_FR:   usize = 0x18;  // Flag Register
const UART_IBRD: usize = 0x24;  // Integer Baud Rate Divisor
const UART_FBRD: usize = 0x28;  // Fractional Baud Rate Divisor
const UART_LCR:  usize = 0x2C;  // Line Control Register
const UART_CR:   usize = 0x30;  // Control Register
const UART_ICR:  usize = 0x44;  // Interrupt Clear Register

// Флаги FR
const FR_TXFF: u32 = 1 << 5;   // TX FIFO full
const FR_RXFE: u32 = 1 << 4;   // RX FIFO empty
const FR_BUSY: u32 = 1 << 3;   // UART busy

fn reg(offset: usize) -> *mut u32 {
    (UART_BASE + offset) as *mut u32
}

fn read(offset: usize) -> u32 {
    unsafe { reg(offset).read_volatile() }
}

fn write(offset: usize, val: u32) {
    unsafe { reg(offset).write_volatile(val) }
}

/// Инициализация UART: 115200 baud, 8N1
/// Вызывается один раз при старте
pub fn init() {
    // Отключаем UART
    write(UART_CR, 0);

    // Ждём пока UART не занят
    while read(UART_FR) & FR_BUSY != 0 {}

    // Очищаем прерывания
    write(UART_ICR, 0x7FF);

    // Baud rate = 115200
    // Формула: Divisor = UARTCLK / (16 * BaudRate)
    // UARTCLK = 3 МГц на RPi Zero 2W
    // Divisor = 3_000_000 / (16 * 115200) = 1.627
    // IBRD = 1, FBRD = round(0.627 * 64) = 40
    write(UART_IBRD, 1);
    write(UART_FBRD, 40);

    // 8 бит, без чётности, 1 стоп-бит, FIFO включено
    write(UART_LCR, 0b0111_0000); // WLEN=11 (8bit), FEN=1

    // Включаем UART: TX + RX
    write(UART_CR, 0x301); // UARTEN | TXE | RXE
}

/// Отправить один байт (блокирующий)
pub fn putc(c: u8) {
    // Ждём пока TX FIFO не освободится
    while read(UART_FR) & FR_TXFF != 0 {}
    write(UART_DR, c as u32);
}

/// Принять один байт (неблокирующий)
pub fn getc() -> Option<u8> {
    if read(UART_FR) & FR_RXFE != 0 {
        None
    } else {
        Some((read(UART_DR) & 0xFF) as u8)
    }
}

/// Отправить строку
pub fn puts(s: &str) {
    for b in s.bytes() {
        if b == b'\n' {
            putc(b'\r'); // CRLF для терминала
        }
        putc(b);
    }
}

/// Отправить число в hex (для отладки)
pub fn put_hex(val: u64) {
    puts("0x");
    let mut started = false;
    for i in (0..16).rev() {
        let nibble = ((val >> (i * 4)) & 0xF) as u8;
        if nibble != 0 || started || i == 0 {
            started = true;
            let c = if nibble < 10 { b'0' + nibble } else { b'a' + nibble - 10 };
            putc(c);
        }
    }
}

/// Отправить число в десятичной (для отладки)
pub fn put_dec(mut val: u64) {
    if val == 0 {
        putc(b'0');
        return;
    }
    let mut buf = [0u8; 20];
    let mut len = 0;
    while val > 0 {
        buf[len] = b'0' + (val % 10) as u8;
        val /= 10;
        len += 1;
    }
    for i in (0..len).rev() {
        putc(buf[i]);
    }
}
