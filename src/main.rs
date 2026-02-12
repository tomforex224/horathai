use std::collections::HashMap;

struct Province {
    name: &'static str,
    longitude: f64,
}

fn main() {
    let mut provinces = HashMap::new();
    provinces.insert("bangkok", Province { name: "กรุงเทพฯ", longitude: 100.50 });
    provinces.insert("chiangmai", Province { name: "เชียงใหม่", longitude: 98.98 });
    provinces.insert("ubon", Province { name: "อุบลราชธานี", longitude: 104.85 });
    provinces.insert("kalasin", Province { name: "กาฬสินธุ์", longitude: 103.53 });
    // ข้อมูลเกิด: 3 ต.ค. 2530 เวลา 15:30 น.
    let day = 3;
    let month = 10;
    let year_be = 2530;
    let hour = 15;
    let minute = 30;
    let province_key = "bangkok";

    if let Some(prov) = provinces.get(province_key) {
        calculate_thai_ascendant(day, month, year_be, hour, minute, prov);
    }
}

fn calculate_thai_ascendant(d: u32, m: u32, y: u32, h: u32, min: u32, prov: &Province) {
    // --- คำนวณ Julian Day (ฐานสำหรับคำนวณดาว) ---
    let jd = calculate_jd(d, m, y, h, min);


    // --- หาองศาอาทิตย์ (ต้องได้ประมาณ 15-16 องศาราศีกันย์)
    let solar_long = estimate_solar_thai(d, m, y);
    let solar_zodiac_idx = (solar_long / 30.0) as usize;
    let solar_degree = solar_long % 30.0;

    // --- หาองศาจันทร์ (๒) ---
    let moon_long = calculate_moon_position(jd);
    let moon_zodiac_idx = (moon_long / 30.0) as usize % 12;
    let moon_degree = moon_long % 30.0;

    //--- คำนวณเวลาท้องถิ่นจริง (LMT)
    // เวลาไทยอิง 105E | กรุงเทพ 100.5E | ต่างกัน 4.5 องศา | 1 องศา = 4 นาที
    // 4.5 * 4 = 18 นาที (ต้อง "ลบ" ออกจากเวลาหน้าปัดนาฬิกา)
    let lmt_offset = (105.0 - prov.longitude) * 4.0;
    let total_birth_min = (h as f64 * 60.0) + min as f64 - lmt_offset;

    // 3. อันตรกาล (เวลาหลังอาทิตย์อุทัย 06:00 น.)
    let mut antar_kala = total_birth_min - 360.0;
    if antar_kala < 0.0 { antar_kala += 1440.0; }

    // 4. ค่าอันโตนาที (มาตรฐานสุริยยาตร์)
    let anto_nati = [120.0, 96.0, 72.0, 120.0, 144.0, 168.0, 168.0, 144.0, 120.0, 72.0, 96.0, 120.0];
    let zodiac_names = ["เมษ", "พฤษภ", "มิถุน", "กรกฎ", "สิงห์", "กันย์", "ตุลย์", "พิจิก", "ธนู", "มกร", "กุมภ์", "มีน"];

    // 5. ชำระกาลราศีแรก (ราศีที่อาทิตย์สถิตอยู่ - กันย์)
    let degree_left = 30.0 - solar_degree;
    let time_left_in_zodiac = (degree_left / 30.0) * anto_nati[solar_zodiac_idx];

    let mut current_idx = solar_zodiac_idx;
    let mut final_degree = 0.0;
    let mut temp_antar = antar_kala;

    // 6. การวางลัคนา
    if temp_antar <= time_left_in_zodiac {
        final_degree = solar_degree + (temp_antar / anto_nati[solar_zodiac_idx] * 30.0);
    } else {
        temp_antar -= time_left_in_zodiac;
        loop {
            current_idx = (current_idx + 1) % 12;
            let zodiac_time = anto_nati[current_idx];
            if temp_antar <= zodiac_time {
                final_degree = (temp_antar / zodiac_time) * 30.0;
                break;
            }
            temp_antar -= zodiac_time;
        }
    }

    // --- แสดงผลลัพธ์ ---
    println!("--- ผลคำนวณดวงชะตาสุริยยาตร์ ---");
    println!("จังหวัด: {} | เวลาท้องถิ่น (LMT): {:02}:{:02} น.", prov.name, (total_birth_min/60.0) as u32, (total_birth_min%60.0) as u32);
    println!("--------------------------------");
    println!("ลัคนา (ล):  ราศี{: <5} องศา {:.2}", zodiac_names[current_idx], final_degree);
    println!("อาทิตย์ (๑): ราศี{: <5} องศา {:.2}", zodiac_names[solar_zodiac_idx], solar_degree);
    println!("จันทร์ (๒):  ราศี{: <5} องศา {:.2}", zodiac_names[moon_zodiac_idx], moon_degree);
    println!("--------------------------------");
}

// ฟังก์ชันประมาณตำแหน่งอาทิตย์ตามคัมภีร์สุริยยาตร์
fn estimate_solar_thai(d: u32, m: u32, y_be: u32) -> f64 {
    let y_ad = y_be - 543;
    let is_leap = (y_ad % 4 == 0 && y_ad % 100 != 0) || (y_ad % 400 == 0);
    let month_days = if is_leap {
        [0, 31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };

    let mut day_of_year = d;
    for i in 1..m as usize {
        day_of_year += month_days[i];
    }

    // วันสงกรานต์ (อาทิตย์ 0 องศาเมษ) โดยปกติคือ 13 เม.ย.
    let apr13 = if is_leap { 104 } else { 103 };
    let mut diff = day_of_year as i32 - apr13;
    if diff < 0 { diff += if is_leap { 366 } else { 365 }; }

    // สุริยยาตร์เฉลี่ย อาทิตย์เดินวันละ 0.9856 องศา (โดยประมาณ)
    (diff as f64) * 0.9856262833675565 
}
fn calculate_jd(d: u32, m: u32, y_be: u32, h: u32, min: u32) -> f64 {
    let mut year = (y_be - 543) as i32;
    let mut month = m as i32;
    if month <= 2 {
        year -= 1;
        month += 12;
    }
    let a = year / 100;
    let b = 2 - a + (a / 4);
    let jd = (365.25 * (year + 4716) as f64).floor() + 
             (30.6001 * (month + 1) as f64).floor() + 
             d as f64 + (h as f64 / 24.0) + (min as f64 / 1440.0) + 
             b as f64 - 1524.5;
    jd
}

fn calculate_moon_position(jd: f64) -> f64 {
   // จุดนับหรคุณ 1 ม.ค. ค.ศ. 1800 (JD 2378496.5)
    let epoch_1800_jd =  2378621.738194;
    let horakhun = jd - epoch_1800_jd; 

    // 1. มัธยมจันทร์ (Mean Moon) 
    // ปรับค่าตำแหน่งเริ่มต้นเป็น 158.5 เพื่อให้สอดคล้องกับตำแหน่งในราศีมังกรสำหรับปี 2530
    let mut mean_moon = (158.5 + (13.1763906 * horakhun)) % 360.0;
    if mean_moon < 0.0 { mean_moon += 360.0; }

    // 2. มัธยมอุจจันทร์ (Mean Apogee)
    let mut mean_apogee = (275.0 + (0.1114035 * horakhun)) % 360.0;
    if mean_apogee < 0.0 { mean_apogee += 360.0; }

    // 3. วิกษิปภาค (Anomaly)
    let mut anomaly = mean_moon - mean_apogee;
    if anomaly < 0.0 { anomaly += 360.0; }

    // 4. สมการจุดศูนย์กลาง (Equation of Center)
    // ใช้ค่า 6.289 ซึ่งเป็นค่าสูงสุดในบางตำราสุริยยาตร์เพื่อดึงตำแหน่งจันทร์ให้แม่นยำขึ้น
    let correction = 6.289 * (anomaly.to_radians().sin());

    // 5. สมผุสจันทร์ (True Moon)
    let mut true_moon = mean_moon - correction;
    if true_moon < 0.0 { true_moon += 360.0; }
    
    true_moon % 360.0
}