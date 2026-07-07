use futures_util::StreamExt;
use tokio_tungstenite::{connect_async, tungstenite::Message};

pub async fn monitor(address: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Replace /config with /monitor in the address
    let monitor_addr = address.replace("/config", "/monitor");
    let (mut ws, _) = connect_async(&monitor_addr).await?;
    println!("Monitoring MIDI output (Ctrl+C to stop)...");

    while let Some(msg) = ws.next().await {
        match msg {
            Ok(Message::Binary(data)) => {
                parse_and_print(&data);
            }
            Ok(Message::Close(_)) => break,
            Err(e) => {
                eprintln!("Connection error: {}", e);
                break;
            }
            _ => {}
        }
    }
    Ok(())
}

fn parse_and_print(data: &[u8]) {
    let mut i = 0;
    while i < data.len() {
        let status = data[i];
        // SysEx: consume until 0xF7
        if status == 0xF0 {
            let end = data[i..]
                .iter()
                .position(|&b| b == 0xF7)
                .unwrap_or(data.len() - i);
            print_midi(&data[i..i + end + 1]);
            i += end + 1;
            continue;
        }
        // System real-time (1 byte)
        if status >= 0xF8 {
            print_midi(&data[i..i + 1]);
            i += 1;
            continue;
        }
        // Channel messages
        let len = match status & 0xF0 {
            0xC0 | 0xD0 => 2, // PC, Channel Pressure
            _ => 3,           // Note, CC, Bend, etc.
        };
        let end = (i + len).min(data.len());
        print_midi(&data[i..end]);
        i = end;
    }
}

fn print_midi(data: &[u8]) {
    if data.is_empty() {
        return;
    }
    let status = data[0];

    // SysEx
    if status == 0xF0 {
        println!("  SysEx     {} bytes", data.len());
        return;
    }

    // System real-time
    match status {
        0xF8 => {
            println!("  Clock");
            return;
        }
        0xFA => {
            println!("  Start");
            return;
        }
        0xFB => {
            println!("  Continue");
            return;
        }
        0xFC => {
            println!("  Stop");
            return;
        }
        0xFE => {
            return;
        } // Active Sensing — suppress
        0xFF => {
            println!("  Reset");
            return;
        }
        _ => {}
    }

    let channel = (status & 0x0F) + 1;
    let msg_type = status & 0xF0;

    match msg_type {
        0x90 if data.len() >= 3 => {
            if data[2] == 0 {
                println!("  Note Off ch={} note={} vel=0", channel, data[1]);
            } else {
                println!("  Note On  ch={} note={} vel={}", channel, data[1], data[2]);
            }
        }
        0x80 if data.len() >= 3 => {
            println!("  Note Off ch={} note={} vel={}", channel, data[1], data[2]);
        }
        0xB0 if data.len() >= 3 => {
            println!("  CC       ch={} cc={} val={}", channel, data[1], data[2]);
        }
        0xC0 if data.len() >= 2 => {
            println!("  PC       ch={} prog={}", channel, data[1]);
        }
        0xD0 if data.len() >= 2 => {
            println!("  ChPress  ch={} val={}", channel, data[1]);
        }
        0xE0 if data.len() >= 3 => {
            let bend = ((data[2] as u16) << 7) | data[1] as u16;
            println!("  Bend     ch={} val={}", channel, bend as i16 - 8192);
        }
        _ => {
            println!(
                "  Raw      {}",
                data.iter()
                    .map(|b| format!("{:02X}", b))
                    .collect::<Vec<_>>()
                    .join(" ")
            );
        }
    }
}
