//! Manage Hyper-V virtual switches: create, modify, delete.
//!
//! Run with: cargo run --example manage_switch -- <command> [options]
//!
//! Commands:
//!   list                    List all virtual switches
//!   list-adapters           List physical network adapters
//!   create <name> <type>    Create a switch (type: private, internal, external)
//!   delete <name>           Delete a virtual switch
//!   info <name>             Show detailed switch information
//!
//! Examples:
//!   cargo run --example manage_switch -- list
//!   cargo run --example manage_switch -- list-adapters
//!   cargo run --example manage_switch -- create "MyPrivate" private
//!   cargo run --example manage_switch -- create "MyInternal" internal
//!   cargo run --example manage_switch -- create "MyExternal" external --adapter "Ethernet"
//!   cargo run --example manage_switch -- info "Default Switch"
//!   cargo run --example manage_switch -- delete "MyPrivate"
//!
//! Requires: Administrator privileges, Hyper-V enabled

use std::env;
use windows_hyperv::{HyperV, Result, VirtualSwitchSettings};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return Ok(());
    }

    let command = &args[1];

    match command.as_str() {
        "list" => list_switches()?,
        "list-adapters" => list_physical_adapters()?,
        "create" => {
            if args.len() < 4 {
                println!("Usage: manage_switch create <name> <type> [--adapter <adapter_name>]");
                println!("Types: private, internal, external");
                return Ok(());
            }
            let name = &args[2];
            let switch_type = &args[3];
            let adapter = find_arg_value(&args, "--adapter");
            create_switch(name, switch_type, adapter.as_deref())?;
        }
        "delete" => {
            if args.len() < 3 {
                println!("Usage: manage_switch delete <name>");
                return Ok(());
            }
            delete_switch(&args[2])?;
        }
        "info" => {
            if args.len() < 3 {
                println!("Usage: manage_switch info <name>");
                return Ok(());
            }
            show_switch_info(&args[2])?;
        }
        _ => {
            println!("Unknown command: {}", command);
            print_usage();
        }
    }

    Ok(())
}

fn print_usage() {
    println!("Hyper-V Virtual Switch Management");
    println!();
    println!("Usage: manage_switch <command> [options]");
    println!();
    println!("Commands:");
    println!("  list                    List all virtual switches");
    println!("  list-adapters           List physical network adapters");
    println!("  create <name> <type>    Create a switch (type: private, internal, external)");
    println!("  delete <name>           Delete a virtual switch");
    println!("  info <name>             Show detailed switch information");
    println!();
    println!("Examples:");
    println!("  manage_switch list");
    println!("  manage_switch create \"MySwitch\" private");
    println!("  manage_switch create \"ExtSwitch\" external --adapter \"Ethernet\"");
}

fn find_arg_value(args: &[String], flag: &str) -> Option<String> {
    for i in 0..args.len() - 1 {
        if args[i] == flag {
            return Some(args[i + 1].clone());
        }
    }
    None
}

fn list_switches() -> Result<()> {
    println!("Connecting to Hyper-V...\n");
    let hyperv = HyperV::connect()?;

    let switches = hyperv.list_switches()?;

    if switches.is_empty() {
        println!("No virtual switches found.");
        return Ok(());
    }

    println!(
        "{:<30} {:<10} {:<10} {:<10} {:<40}",
        "NAME", "TYPE", "MGMT OS", "IOV", "ID"
    );
    println!("{}", "-".repeat(100));

    for switch in switches {
        println!(
            "{:<30} {:<10} {:<10} {:<10} {:<40}",
            truncate(&switch.name, 28),
            format!("{}", switch.switch_type()),
            if switch.allows_management_os() {
                "Yes"
            } else {
                "No"
            },
            if switch.iov_mode().is_enabled() {
                "Enabled"
            } else {
                "Disabled"
            },
            switch.id(),
        );
    }

    Ok(())
}

fn list_physical_adapters() -> Result<()> {
    println!("Connecting to Hyper-V...\n");
    let hyperv = HyperV::connect()?;

    let adapters = hyperv.list_physical_adapters()?;

    if adapters.is_empty() {
        println!("No physical network adapters found.");
        return Ok(());
    }

    println!(
        "{:<30} {:<15} {:<20} {:<15}",
        "NAME", "STATUS", "MAC ADDRESS", "SPEED"
    );
    println!("{}", "-".repeat(80));

    for adapter in adapters {
        let speed = if adapter.speed_mbps() >= 1000 {
            format!("{:.1} Gbps", adapter.speed_gbps())
        } else {
            format!("{} Mbps", adapter.speed_mbps())
        };

        println!(
            "{:<30} {:<15} {:<20} {:<15}",
            truncate(&adapter.name, 28),
            format!("{}", adapter.connection_status()),
            adapter.mac_address().unwrap_or("-"),
            speed,
        );
    }

    println!();
    println!("Use the adapter NAME with --adapter when creating an external switch.");

    Ok(())
}

fn create_switch(name: &str, switch_type: &str, adapter_name: Option<&str>) -> Result<()> {
    println!("Connecting to Hyper-V...");
    let hyperv = HyperV::connect()?;

    let settings = match switch_type.to_lowercase().as_str() {
        "private" => {
            println!("Creating private switch '{}'...", name);
            VirtualSwitchSettings::builder()
                .name(name)
                .private()
                .notes("Created by manage_switch example")
                .build()?
        }
        "internal" => {
            println!("Creating internal switch '{}'...", name);
            VirtualSwitchSettings::builder()
                .name(name)
                .internal()
                .notes("Created by manage_switch example")
                .build()?
        }
        "external" => {
            let adapter_name = adapter_name.ok_or_else(|| windows_hyperv::Error::Validation {
                field: "adapter",
                message: "External switch requires --adapter <name>".to_string(),
            })?;

            println!(
                "Creating external switch '{}' with adapter '{}'...",
                name, adapter_name
            );

            // Find the adapter by name
            let adapters = hyperv.list_physical_adapters()?;
            let adapter = adapters
                .iter()
                .find(|a| a.name.contains(adapter_name) || a.device_id == adapter_name)
                .ok_or_else(|| windows_hyperv::Error::OperationFailed {
                    operation: "FindAdapter",
                    return_value: 0,
                    message: format!("Physical adapter '{}' not found", adapter_name),
                })?;

            VirtualSwitchSettings::builder()
                .name(name)
                .external(&adapter.device_id)
                .allow_management_os(true)
                .notes("Created by manage_switch example")
                .build()?
        }
        _ => {
            println!("Unknown switch type: {}", switch_type);
            println!("Valid types: private, internal, external");
            return Ok(());
        }
    };

    let switch = hyperv.create_switch(&settings)?;

    println!();
    println!("Switch created successfully!");
    println!("  Name: {}", switch.name());
    println!("  Type: {}", switch.switch_type());
    println!("  ID:   {}", switch.id());

    Ok(())
}

fn delete_switch(name: &str) -> Result<()> {
    println!("Connecting to Hyper-V...");
    let hyperv = HyperV::connect()?;

    println!("Finding switch '{}'...", name);
    let switch = hyperv.get_switch(name)?;

    println!("Deleting switch...");
    hyperv.delete_switch(&switch)?;

    println!("Switch '{}' deleted successfully!", name);
    Ok(())
}

fn show_switch_info(name: &str) -> Result<()> {
    println!("Connecting to Hyper-V...\n");
    let hyperv = HyperV::connect()?;

    let switch = hyperv.get_switch(name)?;

    println!("Virtual Switch Information");
    println!("{}", "=".repeat(50));
    println!("Name:                  {}", switch.name());
    println!("ID:                    {}", switch.id());
    println!("Type:                  {} ({})", switch.switch_type(), switch.switch_type().description());
    println!("Management OS Access:  {}", if switch.allows_management_os() { "Yes" } else { "No" });
    println!("IOV Mode:              {:?}", switch.iov_mode());
    println!("Bandwidth Mode:        {:?}", switch.bandwidth_reservation_mode());

    if switch.default_flow_min_bandwidth_weight > 0 {
        println!(
            "Default Bandwidth Weight: {}",
            switch.default_flow_min_bandwidth_weight
        );
    }
    if switch.default_flow_min_bandwidth_absolute > 0 {
        println!(
            "Default Bandwidth (Mbps): {}",
            switch.default_flow_min_bandwidth_absolute / 1_000_000
        );
    }

    if let Some(notes) = switch.notes() {
        println!("Notes:                 {}", notes);
    }

    Ok(())
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len - 3])
    }
}
