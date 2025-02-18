//Just add some libuaries
use pnet::packet::icmp::{echo_request, IcmpTypes};
use pnet::packet::Packet;
use socket2::{Domain, Protocol, Socket, Type, SockAddr};
use std::net::{Ipv4Addr, SocketAddrV4, ToSocketAddrs};
use std::time::{Duration, Instant};
use std::thread::sleep;
use crossterm::{style::Color, ExecutableCommand};
use std::io::{stdout, Write};
use std::mem::MaybeUninit;
use std::env;
use rand::seq::SliceRandom;
use rand::thread_rng;

// Rainbow colors for the pining.
const RAINBOW_COLORS: [Color; 6] = [Color::Red, Color::Yellow, Color::Green, Color::Blue, Color::Magenta, Color::Cyan];

fn print_colored(text: &str, color: Color) {
    let mut stdout = stdout();
    stdout.execute(crossterm::style::SetForegroundColor(color)).unwrap();
    println!("{}", text);
    stdout.execute(crossterm::style::ResetColor).unwrap();
}

fn main() -> std::io::Result<()> {
    // Get domain from command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("💀 Useage: gfping <hostname>");
        return Ok(());
    }

    let hostname = &args[1];

    // Resolve hostname to IP
    let target = resolve_hostname(hostname).unwrap_or_else(|_| {
        eprintln!("💀 Failed to resolve hostname: {}", hostname);
        std::process::exit(1);
    });

    let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::ICMPV4))?;
    println!("😍 Welcome to GFPing:\n");
    println!("😊 We are now pinging: {} ({})\n", hostname, target);

    for i in 1..=10 {
        let mut buffer = [0u8; 64];
        let mut request = echo_request::MutableEchoRequestPacket::new(&mut buffer).unwrap();

        request.set_icmp_type(IcmpTypes::EchoRequest);
        request.set_identifier(1);
        request.set_sequence_number(i);
        let checksum = pnet::util::checksum(request.packet(), 1);
        request.set_checksum(checksum);

        let start = Instant::now();
        let sockaddr = SockAddr::from(SocketAddrV4::new(target, 0));
        socket.send_to(request.packet(), &sockaddr)?;

        let mut response_uninit: [MaybeUninit<u8>; 64] = unsafe { MaybeUninit::uninit().assume_init() };
        socket.set_read_timeout(Some(Duration::from_secs(2)))?;

        let color = RAINBOW_COLORS[i as usize % RAINBOW_COLORS.len()]; // Cycle colors

        match socket.recv_from(&mut response_uninit) {
            Ok((size, _)) => {
                let response: &[u8] = unsafe { std::mem::transmute(&response_uninit[..size]) };
                let rtt = start.elapsed();
                print_colored(&format!("{} Reply from {}: time={}ms", get_emoji(i), target, rtt.as_millis()), color);
            }
            Err(_) => {
                print_colored("💀 Request timed out", Color::DarkRed);
            }
        }

        sleep(Duration::from_secs(1));
    }

    Ok(())
}

// Resolve hostname to Ipv4Addr
fn resolve_hostname(hostname: &str) -> Result<Ipv4Addr, std::io::Error> {
    let addrs = format!("{}:0", hostname).to_socket_addrs()?;
    for addr in addrs {
        if let std::net::SocketAddr::V4(v4) = addr {
            return Ok(*v4.ip());
        }
    }
    Err(std::io::Error::new(std::io::ErrorKind::AddrNotAvailable, "No IPv4 address found"))
}

//  Cycle through fun emojis for more visual effect
fn get_emoji(seq: u16) -> &'static str {
    let emojis = ["💃","💄","💅","🌈", "🔥", "⚡", "💎", "✨", 
    "🎶", "🎈", "🌀", "🎯", "🚀","👨","👩",
    "🍀", "🌟", "💡", "🌊", "🌸", "🎩", "🕶️", "🎲", "💥", "🐉",
    "🎵", "🎮", "🎤", "🎷", "🥁", "🎻", "🧨", "🏆", "🎗️", "🏅",
    "🔮", "📀", "💾", "🔋", "🧲", "🛰️", "🌍", "🌕", "🌓", "🌑",
    "☀️", "🌤️", "🌩️", "⛈️", "🌪️", "🌫️", "🛸", "🚁", "🚂", "🚗",
    "🏎️", "🛶", "⛵", "🏄‍♂️", "🏊‍♀️", "🧗", "🚴‍♂️", "🕹️", "📡", "🔭",
    "🕰️", "🛠️", "⚙️", "🔑", "💣", "🛎️", "🗝️", "🚪", "🛏️", "🚦",
    "🎬", "🎭", "🎨", "🖼️", "📝", "📜", "📖", "📚", "🔖", "📌",
    "📍", "📎", "✂️", "🖍️", "🖊️", "🖋️", "🖌️", "📝", "🔍", "🔬",
    "📊", "📈", "📉", "💰", "💳", "🏦", "💸", "💎", "⚖️", "🏛️",
    "🛡️", "⚔️", "🪓", "🗡️", "🚧", "🏗️", "🏭", "🏰", "🕌", "🛕",
    "⛪", "🛤️", "🚏", "🗼", "🎡", "🎢", "🎠", "🏟️", "🎪", "🚜",
    "🛵", "🏍️", "🛻", "🚚", "🚛", "🚢", "🛳️", "🚤", "⛴️", "🛥️",
    "🛩️", "✈️", "🚀", "🛰️", "🚡", "🚠", "🚟", "🚲", "🦽", "🦼",
    "🏋️", "🤺", "🏌️", "🏄", "🏊", "🚣", "🧗", "🪂", "🏇", "🚵",
    "🚴", "🤹", "🎭", "🎨", "🎬", "🎤", "🎧", "🎼", "🎹", "🥁",
    "🎷", "🎺", "🎸", "🎻", "🎮", "🎰", "🎳", "🎯", "🎱", "🥊",
    "🥋", "🛷", "⛸️", "🥌", "🛹", "🪁", "🎽", "🎿", "🏂", "🏀",
    "🏈", "⚾", "🥎", "🏐", "🏉", "🎾", "🥏", "⛳", "🏹", "🎣",
    "🤿", "🥅", "🏒", "🏑", "🏏", "🪀", "🪃", "♟️", "🛶", "🏕️",
    "🏖️", "🏜️", "🏝️", "🏞️", "🌋", "🗻", "🏔️", "⛰️", "🛤️", "🛣️",
    "🏗️", "🏭", "🏰", "🏕️", "🕌", "🛕", "⛪", "🗼", "🏟️", "🎪",
    "🌇", "🌆", "🏙️", "🌃", "🌉", "🌁", "🛤️", "🚏", "🗿", "🛵",
    "🛻", "🚚", "🚛", "🚐", "🚑", "🚒", "🚓", "🚔", "🚖", "🚘",
    "🚡", "🚠", "🚟", "🚠", "🚟", "🚀", "🛸", "🛰️", "🛩️", "✈️",
    "🛳️", "🚤", "🛥️", "🚢", "🚂", "🚆", "🚊", "🚉", "🚇", "🚝",
    "🚈", "🚎", "🚍", "🚌", "🚘", "🚖", "🚔", "🚑", "🚒", "🚓",
    "🚛", "🚚", "🚜", "🏎️", "🚓", "🚕", "🚗", "🚙", "🚃", "🚞",
    "🛑", "🚧", "⚓", "⛴️", "🛳️", "🚤", "⛵", "🛶", "🛷", "🎿",
    "🏂", "⛷️", "🏋️", "🤺", "🏌️", "🏄", "🏊", "🚣", "🧗", "🪂",
    "🏇", "🚵", "🚴", "🤹", "🎭", "🎨", "🎬", "🎤", "🎧", "🎼",
    "🎹", "🥁", "🎷", "🎺", "🎸", "🎻", "🎮", "🎰", "🎳", "🎯",
    "🎱", "🥊", "🥋", "🛷", "⛸️", "🥌", "🛹", "🪁", "🎽", "🎿",
    "🏀", "🏈", "⚾", "🥎", "🏐", "🏉", "🎾", "🥏", "⛳", "🏹",
    "🎣", "🤿", "🥅", "🏒", "🏑", "🏏", "🪀", "🪃", "♟️", "🛶"];
    let mut rng = thread_rng();
    //emojis[(seq as usize) % emojis.len()
    *emojis.choose(&mut rng).unwrap_or(&"🌈") // Pick a random emoji]
}
