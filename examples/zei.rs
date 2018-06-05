extern crate rumble;
extern crate winrt;

use rumble::api::Central;
use rumble::api::UUID;
use std::thread;
use std::time::Duration;
use rumble::winrtble::utils::to_addr;

use rumble::api::{Peripheral as APIPeripheral};
use rumble::winrtble::peripheral::Peripheral;
use rumble::winrtble::adapter::Adapter;
use rumble::winrtble::manager::Manager;

use winrt::RuntimeContext;

fn main() {
    let manager = Manager::new();

    let adapter = manager.adapters().unwrap();

    adapter.start_scan().unwrap();
    thread::sleep(Duration::from_secs(5));
    adapter.stop_scan().unwrap();

    for i in adapter.peripherals() {
        println!("{}", i);
    }

    /*{
        let zei = Peripheral::new(to_addr(252566450624623));

        zei.on_notification(Box::new(|n| {
            println!("called {:?}", n);
        }));


        let connected = zei.connect();
        println!("! {:?}", connected);
        thread::sleep(Duration::from_secs(1));
        let chara = zei.discover_characteristics().unwrap();
        println!("3 discovered {:?}", chara.len());

        let mut ve = [0x3Cu8, 0x40, 0x5D, 0xA5, 0x89, 0x8C, 0x75, 0x81, 0x11, 0xE6, 0xC8, 0x47, 0xC7, 0xE7, 0x00, 0x12];
        ve.reverse();
        let pos_char_uuid = UUID::B128(ve);

        // 0xC7E70012, Data2: 0xC847, Data3: 0x11E6, Data4: [0x81, 0x75, 0x8C, 0x89, 0xA5, 0x5D, 0x40, 0x3C
        for c in chara {
            if c.uuid == pos_char_uuid {
                println!("++ 3 found uuiid");
                let r = zei.read_by_type_async(&c, pos_char_uuid, Some(Box::new(|res|  {
                    println!("result {:?}", res);
                })));
                println!("++ 3 found end");
                /*if let Ok(r) = r {
                    let st : String = r.iter().map(|c| *c as char).collect();
                    println!("3 {:?} {:?} ", r, st);
                }
                let status = zei.subscribe(&c);
                println!("3  subscribe {:?}", status);*/
            }
        }
        thread::sleep(Duration::from_secs(35));
        let disconnected = zei.disconnect();
        println!("- {:?}", disconnected);

        // let zei = watcher.peripheral(to_addr(252566450624623)).unwrap();
        /*let connected = zei.connect();
        println!("{:?}", connected);
        let disconnected = zei.disconnect();
        println!("{:?}", disconnected);*/
        /*println!("{:?}", zei.is_connected());
        let chara = zei.discover_characteristics().unwrap();

        let mut ve = [0x3Cu8, 0x40, 0x5D, 0xA5, 0x89, 0x8C, 0x75, 0x81, 0x11, 0xE6, 0xC8, 0x47, 0xC7, 0xE7, 0x00, 0x12];
        ve.reverse();
        let pos_char_uuid = UUID::B128(ve);

        // 0xC7E70012, Data2: 0xC847, Data3: 0x11E6, Data4: [0x81, 0x75, 0x8C, 0x89, 0xA5, 0x5D, 0x40, 0x3C
        for c in chara {
            if c.get_uuid() == pos_char_uuid {
                let r = zei.read_by_type(&c, pos_char_uuid);
                if let Ok(r) = r {
                    let st : String = r.iter().map(|c| *c as char).collect();
                    println!("{:?} {:?} ", r, st);
                }
                zei.subscribe(&c);
            }
        }
        thread::sleep(Duration::from_secs(25));
        zei.disconnect().unwrap();
        println!("{:?}", zei.is_connected());*/
    }*/
    
    
    /*for i in watcher.peripherals() {
        println!("{:?}", i);
    }*/

    /* let address = 252566450624623;

    let device = BluetoothLEDevice::from_bluetooth_address_async(address).unwrap().blocking_get().unwrap().unwrap();
    let d = Device::new(device);

    
    let ret = d.is_connected();
    println!("is_connected: {:?}", ret);

    /* if !d.is_paired() {
        let ret = d.pair();
        println!("pair: {:?}", ret);
    } */

    let orientation_service = winrt::Guid{ Data1: 0xC7E70010, Data2: 0xC847, Data3: 0x11E6, Data4: [0x81, 0x75, 0x8C, 0x89, 0xA5, 0x5D, 0x40, 0x3C]};
    let position_characteristic = winrt::Guid{ Data1: 0xC7E70012, Data2: 0xC847, Data3: 0x11E6, Data4: [0x81, 0x75, 0x8C, 0x89, 0xA5, 0x5D, 0x40, 0x3C]};
    let services = d.get_services().unwrap();
    for service in services {
        if service.uuid == orientation_service {
            let characteristics = service.get_characteristics();
            for characteristic in characteristics {
                if characteristic.uuid == position_characteristic {
                    let res = characteristic.read_value();
                    println!("read: {:?}", res);
                    break;
                }
            }
            break;
        }
    }
    let ret = d.is_connected();
    println!("is_connected: {:?}", ret); */

    // let ret = d.un_pair();
    // println!("unpair: {:?}", ret);

    /* let ad = BluetoothLEAdvertisementFilter::new();
    let watcher = BluetoothLEAdvertisementWatcher::create(&ad).unwrap();
    watcher.set_scanning_mode(BluetoothLEScanningMode::Active).unwrap();
    let handler = TypedEventHandler::new(move |_sender, args: *mut BluetoothLEAdvertisementReceivedEventArgs| {
        let aa = unsafe { (&*args) };
        if let Some(adv) = aa.get_advertisement().unwrap() {
            let local_name = adv.get_local_name().unwrap();
            let a = aa.get_bluetooth_address().unwrap();
            if local_name == HString::new("Timeular ZEI") {
                println!("Found {} {}", local_name, a );
            }
            println!("{}", local_name);
        }
        Ok(())
    });
    watcher.add_received(&handler).unwrap();
    watcher.start().unwrap();

    println!("{}", "Hit enter to stop watching.");
    let mut s=String::new();
    stdin().read_line(&mut s).unwrap();
    
    println!("Stopping Watcher");
    watcher.stop().unwrap(); */
}
