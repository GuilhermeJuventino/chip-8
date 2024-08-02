use byteorder::{BigEndian, ByteOrder, ReadBytesExt};

// Variaveis constantes do Chip-8
pub const MEMORY_SIZE: usize = 4095;
const ROM_OFFSET: u16 = 0x200;
const STACK_SIZE: usize = 16;
pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;
const NUM_REGS: usize = 16;

const FONTSET_SIZE: usize = 80;

const FONTSET: [u8; FONTSET_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

// Estrutura do emulador de Chip-8
pub struct Emu {
    // Especificações do Chip-8
    pub p_counter: u16,                           // program counter
    s_pointer: u16,                               // stack pointer
    ram: [u8; MEMORY_SIZE],                       // memória do Chip-8
    screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT], // Tela do Chip-8
    opcode: u16,
    stack: [u16; STACK_SIZE],
    v_reg: [u8; NUM_REGS], // Registros do Chip-8
    i_reg: u16,            // Registro de indice do Chip-8
    d_timer: u8,           // Delay Timer
    s_timer: u8,           // Sound Timer (emite um som quando o timer chega a zero)
}

impl Emu {
    // Inicializando o emulador
    pub fn new() -> Self {
        let mut new_emu = Self {
            p_counter: ROM_OFFSET,
            s_pointer: 0,
            ram: [0; MEMORY_SIZE],
            screen: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            opcode: 0,
            stack: [0; STACK_SIZE],
            v_reg: [0; NUM_REGS],
            i_reg: 0,
            d_timer: 0,
            s_timer: 0,
        };

        new_emu.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);

        new_emu
    }

    // Carregando rom
    pub fn load(&mut self, data: &[u8]) {
        let start = ROM_OFFSET as usize;
        let end = ROM_OFFSET as usize + data.len();
        self.ram[start..end].copy_from_slice(data);
    }

    // Adicionando valores ao stack
    fn push(&mut self, val: u16) {
        self.stack[self.s_pointer as usize] = val;
        self.s_pointer += 1;
    }

    // Removendo valores do stack
    fn pop(&mut self) -> u16 {
        self.s_pointer -= 1;
        self.stack[self.s_pointer as usize]
    }

    // Resetando o emulador
    fn reset(&mut self) {
        self.p_counter = ROM_OFFSET;
        self.s_pointer = 0;
        self.ram = [0; MEMORY_SIZE];
        self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
        self.opcode = 0;
        self.stack = [0; STACK_SIZE];
        self.v_reg = [0; NUM_REGS];
        self.i_reg = 0;
        self.d_timer = 0;
        self.s_timer = 0;
        self.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);
    }

    pub fn tick(&mut self) {
        // Carregar e decifrar opcodes
        let op = self.fetch();

        // Executar instruções
        self.execute(op);
    }

    // Carregando opcodes
    fn fetch(&mut self) -> u16 {
        // Opcodes são guardados na rom como valores 8 bits (ou 1 byte)
        // O código abaixo irá ler os opcodes da rom na posição atual do program counter,
        // combinar os 2 opcodes na posição atual em formato Big Endian,
        // incrementar o program counter em 2 (por que nos lemos duas sequências de 8 bits,
        // portanto devemos incrementar de 2 em 2)
        // e por fim retornar o opcode como um unico valor em 16 bits.

        let data = [
            self.ram[self.p_counter as usize],
            self.ram[self.p_counter as usize + 1],
        ];

        let mut encoded_op = &data[..];
        let op = encoded_op.read_u16::<BigEndian>().unwrap();
        
        /*
        if op != 0 {
            println!("{:#06x}", op);
        }
        */

        //self.p_counter += 2;
        op
    }

    fn tick_timers(&mut self) {
        if self.d_timer > 0 {
            self.d_timer -= 1;
        }

        if self.s_timer > 0 {
            if self.s_timer == 1 {
                // BEEP
            }

            self.s_timer -= 1;
        }
    }

    fn execute(&mut self, op: u16) {
        let digit1 = (op & 0xF000) >> 12;
        let digit2 = (op & 0x0F00) >> 8;
        let digit3 = (op & 0x00F0) >> 4;
        let digit4 = op & 0x000F;

        match (digit1, digit2, digit3, digit4) {
            (0, 0, 0, 0,) => return,

            (0, 0, 0xE, 0) => {
                println!("Executando 00E0: clear_screen()");
                self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
                self.p_counter += 2;
            }

            (0, 0, 0xE, 0xE) => {
                println!("Executando 00E0: return()");
                self.pop();
            }

            (1, _, _, _) => {
                println!("Executando 1NNN: jump()");
                let nnn = op & 0xFFF;
                self.p_counter = nnn;
            }

            (2, _, _, _) => {
                println!("Executando 2NNN: call_subroutine()"); 
                let nnn = op & 0xFFF;
                self.push(nnn);
            }

            (6, _, _, _) => {
                println!("Executando 6XNN: set_vx_to_nn()");
                let nn = (op & 0xFF) as u8;
                let vx = digit2 as usize;
                self.v_reg[vx] = nn;
                self.p_counter += 2;
            }

            (7, _, _, _) => {
                println!("Executando 7XNN: add_nn_to_vx()");
                let nn = (op & 0xFF) as u8;
                let vx = digit2 as usize;
                self.v_reg[vx] += nn;
                self.p_counter += 2;
            }

            (0xA, _, _, _) => {
                print!("Executando ANNN: set_i_to_nnn()");
                let nnn = op & 0xFFF;
                self.i_reg = nnn;
                self.p_counter += 2;
            }

            (0xD, _, _, _) => {
                println!("Executando DXYN: draw_sprite()");
                self.p_counter += 2;
            }

            (_, _, _, _) => {
                //self.p_counter += 2;
                unimplemented!("Opcode: {:#06x} não implementado....", op);
            }
        }
    }
}
