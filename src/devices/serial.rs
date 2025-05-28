// src/devices/cupana_tcp_serial.rs

use std::collections::VecDeque;
use std::io::{ErrorKind, Read, Write};
use std::net::{TcpListener, TcpStream, Shutdown}; // Adicionado para TCP
use std::sync::mpsc::{Receiver, Sender, TryRecvError, channel}; // Para comunicação thread -> dispositivo
use std::thread; // Para a thread do listener TCP

use crate::error::MemoryError;
use super::Device; //

// Constantes de registradores e bits (iguais ao cupana_serial.rs)
const DATA_REGISTER_OFFSET: u16 = 0x00;
const STATUS_REGISTER_OFFSET: u16 = 0x01;
const CONTROL_REGISTER_OFFSET: u16 = 0x02;
const TCP_SERIAL_DEVICE_SIZE: u16 = 3;

const RX_READY_BIT: u8 = 0b0000_0001;
const TX_READY_BIT: u8 = 0b0000_0010;
const CLIENT_CONNECTED_BIT: u8 = 0b0000_0100; // Novo: Bit para indicar se um cliente está conectado

const RX_INTERRUPT_ENABLE_BIT: u8 = 0b0000_0001;
const TX_INTERRUPT_ENABLE_BIT: u8 = 0b0000_0010; // Não usado ativamente para interrupção TX
const CONNECT_INTERRUPT_ENABLE_BIT: u8 = 0b0000_0100; // Novo: Habilitar interrupção na conexão/desconexão


// Mensagens que a thread TCP pode enviar para o dispositivo principal
#[derive(Debug)]
enum TcpThreadMessage {
    ClientConnected(TcpStream),
    ClientDisconnected,
    DataReceived(Vec<u8>),
    Error(String),
}

#[derive(Debug)]
pub struct CupanaTcpSerial {
    base_address: u16,
    input_buffer: VecDeque<u8>, // Buffer para dados lidos do TcpStream

    rx_interrupt_enabled: bool,
    // tx_interrupt_enabled: bool, //  TX interrupt não focado nesta versão
    connect_interrupt_enabled: bool,

    // Estado da conexão TCP
    client_stream: Option<TcpStream>,
    is_client_connected: bool, // Para o bit de status

    // Canal para receber mensagens da thread do listener TCP
    message_receiver: Receiver<TcpThreadMessage>,
    // Opcional: handle da thread para join no Drop
    _listener_thread_handle: Option<thread::JoinHandle<()>>,
}

impl CupanaTcpSerial {
    pub fn new(base_address: u16, listen_addr: &str) -> Self {
        let (tx, rx): (Sender<TcpThreadMessage>, Receiver<TcpThreadMessage>) = channel();
        let listener_tx = tx.clone();
        let owned_listen_addr = listen_addr.to_string();

        println!("[TcpSerial] Iniciando servidor TCP em {}", owned_listen_addr);

        let handle = thread::spawn(move || {
            match TcpListener::bind(owned_listen_addr) {
                Ok(listener) => {
                    println!("[TcpSerialThread] Escutando por conexões...");
                    // Aceita apenas uma conexão por vez para simular uma porta serial simples
                    for stream_result in listener.incoming() {
                        match stream_result {
                            Ok(mut stream) => {
                                println!("[TcpSerialThread] Cliente conectado: {}", stream.peer_addr().unwrap());
                                if let Err(e) = stream.set_nonblocking(true) {
                                     let _ = listener_tx.send(TcpThreadMessage::Error(format!("Falha ao definir stream como não bloqueante: {}", e)));
                                     continue; // Tenta aceitar o próximo
                                }
                                 let _ = stream.write(b"Bem-vindo a Cupana VM via TCP Serial!\r\n");


                                let stream_tx = listener_tx.clone();
                                let peer_addr = stream.peer_addr().map_or_else(|_| "desconhecido".to_string(), |a| a.to_string());
                                
                                // Envia a stream para o dispositivo principal
                                if listener_tx.send(TcpThreadMessage::ClientConnected(stream.try_clone().expect("Falha ao clonar stream"))).is_err() {
                                    // Dispositivo principal provavelmente foi dropado
                                    break;
                                }

                                let mut buffer = [0; 1024];
                                loop {
                                    match stream.read(&mut buffer) {
                                        Ok(0) => { // Conexão fechada pelo cliente
                                            println!("[TcpSerialThread] Cliente {} desconectado (read 0 bytes).", peer_addr);
                                            let _ = stream_tx.send(TcpThreadMessage::ClientDisconnected);
                                            break; // Sai do loop de leitura, espera por nova conexão
                                        }
                                        Ok(n) => { // Dados recebidos
                                            let data = buffer[..n].to_vec();
                                            if stream_tx.send(TcpThreadMessage::DataReceived(data)).is_err() {
                                                break; // Dispositivo principal foi dropado
                                            }
                                        }
                                        Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                                            // Nada para ler, apenas continue verificando (ou durma um pouco)
                                            // Para evitar busy-looping na thread, pode-se adicionar um pequeno sleep
                                            // ou usar um sistema de polling mais avançado (como mio/tokio, mas isso adiciona complexidade)
                                            thread::sleep(std::time::Duration::from_millis(10));
                                            continue;
                                        }
                                        Err(e) => { // Erro de leitura
                                            println!("[TcpSerialThread] Erro ao ler do cliente {}: {}", peer_addr, e);
                                            let _ = stream_tx.send(TcpThreadMessage::ClientDisconnected);
                                            break; 
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                let _ = listener_tx.send(TcpThreadMessage::Error(format!("Erro ao aceitar conexão: {}", e)));
                            }
                        }
                         // Após um cliente desconectar, estamos prontos para um novo
                        println!("[TcpSerialThread] Esperando por nova conexão...");
                    }
                }
                Err(e) => {
                    let _ = listener_tx.send(TcpThreadMessage::Error(format!("Falha ao iniciar TcpListener: {}", e)));
                }
            }
        });

        Self {
            base_address,
            input_buffer: VecDeque::new(),
            rx_interrupt_enabled: false,
            connect_interrupt_enabled: false,
            client_stream: None,
            is_client_connected: false,
            message_receiver: rx,
            _listener_thread_handle: Some(handle),
        }
    }

    fn get_status_register_value(&self) -> u8 {
        let mut status: u8 = 0;
        if !self.input_buffer.is_empty() {
            status |= RX_READY_BIT;
        }
        if self.client_stream.is_some() { // TX_READY se estivermos conectados
            status |= TX_READY_BIT;
            status |= CLIENT_CONNECTED_BIT;
        }
        status
    }

    fn get_control_register_value(&self) -> u8 {
        let mut control: u8 = 0;
        if self.rx_interrupt_enabled {
            control |= RX_INTERRUPT_ENABLE_BIT;
        }
        if self.connect_interrupt_enabled {
            control |= CONNECT_INTERRUPT_ENABLE_BIT;
        }
        control
    }

    // Processa mensagens da thread TCP
    fn process_tcp_messages(&mut self) -> bool {
        let mut client_status_changed = false;
        loop {
            match self.message_receiver.try_recv() {
                Ok(TcpThreadMessage::ClientConnected(stream)) => {
                    println!("[TcpSerial] Mensagem: Cliente Conectado.");
                    self.client_stream = Some(stream);
                    self.is_client_connected = true;
                    client_status_changed = true;
                }
                Ok(TcpThreadMessage::ClientDisconnected) => {
                    println!("[TcpSerial] Mensagem: Cliente Desconectado.");
                    if let Some(stream) = self.client_stream.as_mut() {
                        let _ = stream.shutdown(Shutdown::Both);
                    }
                    self.client_stream = None;
                    self.is_client_connected = false;
                    client_status_changed = true;
                }
                Ok(TcpThreadMessage::DataReceived(data)) => {
                    // println!("[TcpSerial] Mensagem: Dados recebidos ({} bytes)", data.len());
                    for byte in data {
                        if self.input_buffer.len() < 256 { // Limite do buffer
                            self.input_buffer.push_back(byte);
                        } else {
                            eprintln!("[TcpSerial] Buffer de entrada cheio, byte descartado.");
                            break;
                        }
                    }
                }
                Ok(TcpThreadMessage::Error(e)) => {
                    eprintln!("[TcpSerial] Mensagem de Erro da Thread TCP: {}", e);
                }
                Err(TryRecvError::Empty) => {
                    break; // Sem mais mensagens no momento
                }
                Err(TryRecvError::Disconnected) => {
                    // A thread do remetente terminou, isso não deveria acontecer a menos que a VM esteja desligando
                    eprintln!("[TcpSerial] Canal de mensagens da thread TCP desconectado!");
                    break;
                }
            }
        }
        client_status_changed
    }
}

impl Device for CupanaTcpSerial {
    fn aabb(&self) -> (u16, u16) {
        (self.base_address, self.base_address + TCP_SERIAL_DEVICE_SIZE - 1)
    }

    fn read_u8(&mut self, addr_offset: u16) -> Result<u8, MemoryError> {
        let _ = self.process_tcp_messages(); // Sempre processa mensagens antes de agir

        match addr_offset {
            DATA_REGISTER_OFFSET => {
                if let Some(val) = self.input_buffer.pop_front() {
                    Ok(val)
                } else {
                    Ok(0) // Buffer de entrada vazio
                }
            }
            STATUS_REGISTER_OFFSET => Ok(self.get_status_register_value()),
            CONTROL_REGISTER_OFFSET => Ok(self.get_control_register_value()),
            _ => Err(MemoryError::InvalidRamAddress(self.base_address + addr_offset)),
        }
    }

    fn write_u8(&mut self, addr_offset: u16, val: u8) -> Result<(), MemoryError> {
        let _ = self.process_tcp_messages(); 

        match addr_offset {
            DATA_REGISTER_OFFSET => {
                if let Some(stream) = self.client_stream.as_mut() {
                    match stream.write(&[val]) {
                        Ok(0) => { eprintln!("[TcpSerial] Escrita de 0 bytes no stream TCP."); }
                        Ok(_) => { /* Sucesso */ }
                        Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                            eprintln!("[TcpSerial] Escrita no stream TCP bloquearia. Byte perdido.");
                            // Idealmente, a CPU verificaria TX_READY ou haveria um buffer de saída.
                        }
                        Err(e) => {
                            eprintln!("[TcpSerial] Erro ao escrever no stream TCP: {}. Desconectando.", e);
                            self.is_client_connected = false;
                            let _ = stream.shutdown(Shutdown::Both);
                            self.client_stream = None;
                        }
                    }
                } else {
                    // println!("[TcpSerial] Aviso: Tentativa de escrita sem cliente conectado.");
                }
                Ok(())
            }
            CONTROL_REGISTER_OFFSET => {
                self.rx_interrupt_enabled = (val & RX_INTERRUPT_ENABLE_BIT) != 0;
                self.connect_interrupt_enabled = (val & CONNECT_INTERRUPT_ENABLE_BIT) != 0;
                Ok(())
            }
            STATUS_REGISTER_OFFSET => {
                Err(MemoryError::WriteNotPermitted(self.base_address + addr_offset))
            }
            _ => Err(MemoryError::InvalidRamAddress(self.base_address + addr_offset)),
        }
    }

    fn check_interrupt(&mut self) -> bool {
        let client_status_changed = self.process_tcp_messages();
        let mut interrupt_pending = false;

        if self.rx_interrupt_enabled && !self.input_buffer.is_empty() {
            interrupt_pending = true;
        }
        if self.connect_interrupt_enabled && client_status_changed {
            // Gera interrupção se o status da conexão mudou e a interrupção de conexão está habilitada
            interrupt_pending = true;
        }
        interrupt_pending
    }
}

// Opcional: Implementar Drop para limpar a thread se necessário, embora
// o handle da thread sendo dropado quando CupanaTcpSerial é dropado
// deve fazer a thread terminar se ela verificar o canal de envio.
impl Drop for CupanaTcpSerial {
    fn drop(&mut self) {
        println!("[TcpSerial] Dropando CupanaTcpSerial. A thread do listener deve terminar.");
        // A thread deve sair naturalmente quando o Sender do canal principal é dropado.
        // Se você quiser ter certeza, você pode precisar de um mecanismo de sinalização para a thread.
    }
}