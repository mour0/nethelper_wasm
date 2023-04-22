
use netcalc::utils::{ipv4::{cidr_to_sm, get_network, get_class, sm_to_wm, possible_host, vlsm}, ipv6::{get_network6, cidr_to_sm6, sm_to_wm6, possible_host6}};
use wasm_bindgen::prelude::*;
use js_sys::*;
use num_format::{Locale, ToFormattedString};


// Exposed functions to JS
#[wasm_bindgen]
pub fn exp_network(ip:&str,cidr: u8) -> Result<Array,JsValue>
{
    match get_network(ip, cidr)
    {
        Ok(net) => {
            let arr = Array::new_with_length(net.len() as u32);

            let class = get_class(net[0][0]).unwrap(); // Can unwrap, checks in 'get_network'

            arr.set(0, JsValue::from_str(format!("{}.{}.{}.{} (Class {})\n{:08b}.{:08b}.{:08b}.{:08b}" , net[0][0], net[0][1], net[0][2], net[0][3], class,
                                                                                                        net[0][0], net[0][1], net[0][2], net[0][3]).as_str()));
            for i in 1..net.len()
            {
                arr.set(i as u32, JsValue::from_str(format!("{}.{}.{}.{}\n{:08b}.{:08b}.{:08b}.{:08b}" , net[i][0], net[i][1], net[i][2], net[i][3], 
                                                                                                        net[i][0], net[i][1], net[i][2], net[i][3]).as_str()));
            }
            Ok(arr)
        },
        Err(e) => Err(JsValue::from_str(&e.to_string())),
    }
}

#[wasm_bindgen]
pub fn exp_hosts(cidr: u8) -> Result<Array,JsValue>
{
    match cidr_to_sm(cidr)
    {
        Ok(temp) => {
            let arr = Array::new_with_length(4);
            arr.set(0,JsValue::from_str(format!("{}.{}.{}.{}\n{:08b}.{:08b}.{:08b}.{:08b}",temp[0],temp[1],temp[2],temp[3],
                                                                                                        temp[0],temp[1],temp[2],temp[3]).as_str()));
            let wm = sm_to_wm(temp);
            arr.set(1, JsValue::from_str(format!("{}.{}.{}.{}\n{:08b}.{:08b}.{:08b}.{:08b}",wm[0],wm[1],wm[2],wm[3],
                                                                                                        wm[0],wm[1],wm[2],wm[3]).as_str()));
            arr.set(2,JsValue::from_str(format!("/{}",cidr).as_str()));
            arr.set(3,JsValue::from(possible_host(cidr).to_formatted_string(&Locale::en)));
            Ok(arr) 
        },
        Err(e) => Err(JsValue::from_str(&e.to_string())),
    }
}

#[wasm_bindgen]
pub fn exp_vlsm(ip:&str,cidr: u8,n_host:&str) -> Result<Array,JsValue>
{
    let n_host_vec:Vec<u32> = n_host.split(",").map(|x| match x.parse::<u32>() {
        Ok(n) => n,
        Err(_) => 0,
    }).filter(|w| *w != 0).collect(); 

    match vlsm(ip,cidr,n_host_vec)
    {
        Ok(temp) => {
            let arr = Array::new_with_length(temp.len() as u32);
            for i in 0..temp.len()
            {
                arr.set(i as u32,JsValue::from_str(format!("Network: {}\nBroadcast: {}\nHost Range: {} - {}\nRequired Hosts: {}",temp[i][0],temp[i][1],temp[i][2],temp[i][3],temp[i][4]).as_str()));
            }
            Ok(arr) 
        },
        Err(e) => Err(JsValue::from_str(&e.to_string())),
    }

}

#[wasm_bindgen]
pub fn exp_network6(ip:&str,cidr: u8) -> Result<Array,JsValue>
{
    match get_network6(ip, cidr)
    {
        Ok(net) => {
            let arr = Array::new_with_length(net.len() as u32);
            for i in 0..net.len()
            {
                arr.set(i as u32, JsValue::from_str(format!("{:04x}:{:04x}:{:04x}:{:04x}:{:04x}:{:04x}:{:04x}:{:04x}", net[i][0], net[i][1], net[i][2], net[i][3], net[i][4], net[i][5], net[i][6], net[i][7]).as_str()));
            }
            Ok(arr)
        },
        Err(e) => Err(JsValue::from_str(&e.to_string())),
    }
}

#[wasm_bindgen]
pub fn exp_hosts6(cidr: u8) -> Result<Array,JsValue>
{
    match cidr_to_sm6(cidr)
    {
        Ok(temp) => {
            let arr = Array::new_with_length(4);
            arr.set(0,JsValue::from_str(format!("{:04x}:{:04x}:{:04x}:{:04x}:{:04x}:{:04x}:{:04x}:{:04x}",temp[0],temp[1],temp[2],temp[3],temp[4],temp[5],temp[6],temp[7]).as_str()));
            let wm = sm_to_wm6(temp);
            arr.set(1, JsValue::from_str(format!("{:04x}:{:04x}:{:04x}:{:04x}:{:04x}:{:04x}:{:04x}:{:04x}",wm[0],wm[1],wm[2],wm[3],wm[4],wm[5],wm[6],wm[7]).as_str()));
            arr.set(2,JsValue::from_str(format!("/{}",cidr).as_str()));
            //let mut n_host_str = format!("{}",possible_host6(cidr));
            let mut n_host_str = possible_host6(cidr).to_formatted_string(&Locale::en);
            
            // Temp before using bigint
            if cidr == 0 
            {
                n_host_str.pop();
                n_host_str.push('6');
            }
            arr.set(3,JsValue::from_str(n_host_str.as_str()));
            Ok(arr)
        },
        Err(e) => Err(JsValue::from_str(&e.to_string())),
    }
}

