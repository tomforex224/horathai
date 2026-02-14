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
    let hour = 16;
    let minute = 0;
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

    // -- หาตำแหน่งดาวอังคาร (๓)
    let mars_long = calculate_mars_position(jd);
    let mars_zodiac_idx = (mars_long / 30.0) as usize % 12;
    let mars_degree = mars_long % 30.0;

    // -- หาตำแหน่งดาวพุธ (๔) - ต้องส่งค่า solar_long เข้าไปด้วย
    let mercury_long = calculate_mercury_position(jd, solar_long);
    let mercury_zodiac_idx = (mercury_long / 30.0) as usize % 12;
    let mercury_degree = mercury_long % 30.0;

    // ดาวพฤหัสบดี (๕)
    let jupiter_long = calculate_jupiter_position(jd);
    // ดาวศุกร์ (๖) - ใช้ solar_long เป็นฐานเหมือนดาวพุธ
    let venus_long = calculate_venus_position(jd, solar_long);
    // ดาวเสาร์ (๗)
    let saturn_long = calculate_saturn_position(jd);
    // ราหู (๘)
    let rahu_long = calculate_rahu_position(jd);
    // เกตุไทย (๙)
    let ketu_long = calculate_ketu_thai_position(jd);

    let uranus_long = calculate_uranus_position(jd);

    // วนลูปแสดงผล (ตัวอย่าง)
    let planets = [
        ("พฤหัสบดี (๕)", jupiter_long),
        ("ศุกร์ (๖)", venus_long),
        ("เสาร์ (๗)", saturn_long),
        ("ราหู (๘)", rahu_long),
        ("เกตุ (๙)", ketu_long),
        ("มฤตยู (๐)", uranus_long),
    ];

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
    println!("อังคาร (๓): ราศี{: <5} องศา {:.2}", zodiac_names[mars_zodiac_idx], mars_degree);
    println!("พุธ (๔):    ราศี{: <5} องศา {:.2}", zodiac_names[mercury_zodiac_idx], mercury_degree);
    for (name, pos) in planets {
        let z_idx = (pos / 30.0) as usize % 12;
        let deg = pos % 30.0;
        println!("{: <12}: ราศี{: <5} องศา {:.2}", name, zodiac_names[z_idx], deg);
    }
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
    let epoch_1800_jd =  2378600.45833;
    let horakhun = jd - epoch_1800_jd; 

    // 1. มัธยมจันทร์ (Mean Moon) 
    // ปรับค่าตำแหน่งเริ่มต้นเป็น 158.5 เพื่อให้สอดคล้องกับตำแหน่งในราศีมังกรสำหรับปี 2530
    let mut mean_moon = (238.15 + (13.1763906 * horakhun)) % 360.0;
    if mean_moon < 0.0 { mean_moon += 360.0; }

    // 2. มัธยมอุจจันทร์ (Mean Apogee)
    let mut mean_apogee = (172.42 + (0.1114035 * horakhun)) % 360.0;
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

// ฟังก์ชันคำนวณดาวอังคาร (๓)
fn calculate_mars_position(jd: f64) -> f64 {
    let epoch_1800_jd = 2378598.458333;
    let horakhun = (jd - epoch_1800_jd)/ 36525.0;
    // 1. คำนวณวงโคจรโลก (Earth/Sun elements)
    let e_l = (100.46435 + 35999.37249 * horakhun) % 360.0; // Mean Longitude
    let e_g = (102.93768 + 0.32327 * horakhun) % 360.0;    // Perihelion
    let e_m = (e_l - e_g).to_radians();             // Mean Anomaly
    let e_v = e_l + 1.9148 * e_m.sin() + 0.0200 * (2.0 * e_m).sin(); // True Longitude
    let e_r = 1.00014 * (1.0 - 0.01671 * e_m.cos()); // Distance from Sun (AU)

    // 2. คำนวณวงโคจรดาวอังคาร (Mars elements)
    let m_l = (317.18 + 19140.30268 * horakhun) % 360.0; // Mean Longitude
    let m_g = (335.15 + 0.44301 * horakhun) % 360.0;     // Perihelion
    let m_m = (m_l - m_g).to_radians();              // Mean Anomaly
    
    // Equation of Center สำหรับอังคาร (แม่นยำขึ้น)
    let m_v = m_l + 10.6912 * m_m.sin() + 0.6228 * (2.0 * m_m).sin(); 
    let m_v_rad = m_v.to_radians();
    
    // ระยะห่างจากดวงอาทิตย์ (Mars Distance - r)
    // สูตร: r = a(1 - e^2) / (1 + e cos(v)) โดยประมาณ
    let m_r = 1.52368 * (1.0 - 0.0934 * m_m.cos());

    // 3. แปลงจาก Heliocentric (ดวงอาทิตย์เป็นศูนย์กลาง) เป็น Geocentric (โลกเป็นศูนย์กลาง)
    // ใช้กฎของ Sine/Cosine ในการหาตำแหน่งที่มองจากโลก
    let e_v_rad = e_v.to_radians();
    
    let x = m_r * m_v_rad.cos() - e_r * e_v_rad.cos();
    let y = m_r * m_v_rad.sin() - e_r * e_v_rad.sin();

    let mut geocentric_long = y.atan2(x).to_degrees();

    // ปรับค่าให้อยู่ในช่วง 0-360
    if geocentric_long < 0.0 {
        geocentric_long += 360.0;
    }
    
    geocentric_long
}

// ฟังก์ชันคำนวณดาวพุธ (๔) - ปรับปรุงแบบละเอียด
// อ้างอิง Epoch: 13 เมษายน 1800 เวลา 6:00 น. (GMT+7) = JD 2378493.458333 (UTC 12 เม.ย. 1800 23:00)
fn calculate_mercury_position(jd: f64, solar_long: f64) -> f64 {
    // Epoch อ้างอิง: 13 เมษายน 1800 เวลา 06:00 GMT+7
    // = 12 เมษายน 1800 เวลา 23:00 UTC
    // JD = 2378493.458333
    let epoch_1800_jd = 2378493.458333;
    let t = (jd - epoch_1800_jd) / 36525.0; // จำนวนศตวรรษจูเลียนหลัง epoch

    // ===== 1. คำนวณตำแหน่งโลก (Earth/Sun) - ละเอียดขึ้น =====
    // Mean Longitude ของโลก (องศา)
    let mut e_l = 100.46435 + 35999.37249 * t + 0.0003032 * t * t;
    e_l = e_l % 360.0;
    if e_l < 0.0 { e_l += 360.0; }
    
    // Perihelion ของโลก
    let mut e_g = 102.93768 + 0.32327 * t + 0.00015 * t * t;
    e_g = e_g % 360.0;
    if e_g < 0.0 { e_g += 360.0; }
    
    // Mean Anomaly ของโลก
    let e_m = (e_l - e_g).to_radians();
    
    // Equation of Center สำหรับโลก (แม่นยำขึ้น - พจน์ 5 ระดับ)
    let e_eq_center = 1.914600 * e_m.sin() 
                    + 0.019993 * (2.0 * e_m).sin()
                    + 0.000290 * (3.0 * e_m).sin()
                    + 0.000020 * (4.0 * e_m).sin()
                    + 0.000002 * (5.0 * e_m).sin();
    
    // True Longitude ของโลก
    let e_v = e_l + e_eq_center;
    let e_v_rad = e_v.to_radians();
    
    // ระยะห่างโลก-ดวงอาทิตย์ (AU) - สูตรแม่นยำ
    let e_ecc = 0.016708617 - 0.000042037 * t - 0.0000001236 * t * t; // eccentricity ของโลก
    let e_r = 1.000001018 * (1.0 - e_ecc * e_ecc) / (1.0 + e_ecc * e_m.cos());

    // ===== 2. คำนวณวงโคจรดาวพุธ (Heliocentric) - แม่นยำสูง =====
    
    // Mean Longitude ของดาวพุธ ณ Epoch (องศา)
    // ค่าเริ่มต้น ณ 13 เม.ย. 1800: ประมาณ 315.5 องศา
    let mut m_l = 315.5484 + 149472.6746 * t + 0.00030397 * t * t;
    m_l = m_l % 360.0;
    if m_l < 0.0 { m_l += 360.0; }
    
    // Perihelion ของดาวพุธ
    let mut m_perihelion = 77.4561 + 0.15856 * t + 0.00002953 * t * t;
    m_perihelion = m_perihelion % 360.0;
    if m_perihelion < 0.0 { m_perihelion += 360.0; }
    
    // Ascending Node ของดาวพุธ
    let mut m_node = 48.3309 - 0.12534 * t + 0.00008863 * t * t;
    m_node = m_node % 360.0;
    if m_node < 0.0 { m_node += 360.0; }
    
    // Inclination (ความเอียงของวงโคจร)
    let m_inc = 7.00487 - 0.00178 * t; // องศา
    
    // Mean Anomaly ของดาวพุธ
    let m_m = (m_l - m_perihelion).to_radians();
    
    // Eccentricity ของดาวพุธ (แปรตามเวลา)
    let m_ecc = 0.20563069 + 0.000020406 * t - 0.0000000284 * t * t;
    
    // Equation of Center สำหรับดาวพุธ (พจน์ 8 ระดับ - เนื่องจากวงรีมาก)
    let m_eq_center = 23.4406 * m_m.sin()
                    + 2.9818 * (2.0 * m_m).sin()
                    + 0.5255 * (3.0 * m_m).sin()
                    + 0.1058 * (4.0 * m_m).sin()
                    + 0.0241 * (5.0 * m_m).sin()
                    + 0.0055 * (6.0 * m_m).sin()
                    + 0.0013 * (7.0 * m_m).sin()
                    + 0.0003 * (8.0 * m_m).sin();
    
    // True Anomaly
    let m_v_true = m_l + m_eq_center;
    
    // Argument of Latitude (u = v + ω - Ω)
    let m_arg_lat = (m_v_true - m_node).to_radians();
    
    // ระยะห่างดาวพุธ-ดวงอาทิตย์ (AU)
    let m_a = 0.387098; // Semi-major axis
    let m_r = m_a * (1.0 - m_ecc * m_ecc) / (1.0 + m_ecc * m_m.cos());
    
    // ===== 3. คำนวณตำแหน่ง 3 มิติและแปลงเป็นระนาบอุปราศี =====
    
    // ตำแหน่งในระนาบวงโคจร (orbital plane)
    let m_inc_rad = m_inc.to_radians();
    let m_node_rad = m_node.to_radians();
    
    // แปลงเป็นพิกัด heliocentric ecliptic
    let x_orb = m_r * m_arg_lat.cos();
    let y_orb = m_r * m_arg_lat.sin();
    
    // แปลงจากระนาบวงโคจรไปสู่ระนาบอุปราศี (ecliptic)
    let m_x_helio = (x_orb * m_node_rad.cos() - y_orb * m_inc_rad.cos() * m_node_rad.sin());
    let m_y_helio = (x_orb * m_node_rad.sin() + y_orb * m_inc_rad.cos() * m_node_rad.cos());
    let m_z_helio = (y_orb * m_inc_rad.sin());
    
    // ===== 4. แปลงเป็น Geocentric (มองจากโลก) =====
    
    // ตำแหน่งโลกในระบบ heliocentric
    let e_x = e_r * e_v_rad.cos();
    let e_y = e_r * e_v_rad.sin();
    let e_z = 0.0; // โลกอยู่ในระนาบอุปราศี
    
    // ตำแหน่งดาวพุธที่มองจากโลก
    let geo_x = m_x_helio - e_x;
    let geo_y = m_y_helio - e_y;
    let geo_z = m_z_helio - e_z;
    
    // Geocentric longitude (ลองจิจูดท้องฟ้า)
    let mut geocentric_long = geo_y.atan2(geo_x).to_degrees();
    
    // ===== 5. การแก้ไขเพิ่มเติม (Perturbations) =====
    // พจน์แก้ไขจากดาวเคราะห์ดวงอื่น (โดยเฉพาะดาวศุกร์และดาวพฤหัสบดี)
    
    // Mean longitude ของดาวศุกร์
    let venus_mean_long = (181.9798 + 58517.8156 * t) % 360.0;
    
    // Mean longitude ของดาวพฤหัสบดี  
    let jupiter_mean_long = (34.3515 + 3034.9056 * t) % 360.0;
    
    // Perturbation terms (องศา)
    let pert_venus = 0.00204 * (5.0 * venus_mean_long.to_radians() - 2.0 * m_l.to_radians() + 0.21328).sin();
    let pert_jupiter = 0.00103 * (2.0 * jupiter_mean_long.to_radians() - 5.0 * m_l.to_radians() - 3.07577).sin();
    let pert_earth = 0.00091 * (2.0 * e_l.to_radians() - 2.0 * m_l.to_radians() - 0.05149).sin();
    
    geocentric_long += pert_venus + pert_jupiter + pert_earth;
    
    // ===== 6. Nutation (การส่ายของแกนโลก) - แก้ไขเล็กน้อย =====
    let omega = (125.04 - 1934.136 * t).to_radians(); // Longitude ของ ascending node ของดวงจันทร์
    let nutation_long = -0.00569 - 0.00479 * omega.sin(); // องศา
    
    geocentric_long += nutation_long;
    
    // ===== 7. ปรับค่าให้อยู่ในช่วง 0-360 องศา =====
    geocentric_long = geocentric_long % 360.0;
    if geocentric_long < 0.0 { 
        geocentric_long += 360.0; 
    }
    
    geocentric_long
}

// ดาวพฤหัสบดี (๕): เดินประมาณ 1 ราศีต่อปี (ความเร็ว 0.083 องศา/วัน)
// ฟังก์ชันคำนวณดาวพฤหัสบดี (๕) - ปรับปรุงแบบละเอียด
// อ้างอิง Epoch: 13 เมษายน 1800 เวลา 6:00 น. (GMT+7) = JD 2378493.458333
fn calculate_jupiter_position(jd: f64) -> f64 {
    // Epoch อ้างอิง: 13 เมษายน 1800 เวลา 06:00 GMT+7
    let epoch_1800_jd = 2378493.458333;
    let t = (jd - epoch_1800_jd) / 36525.0; // จำนวนศตวรรษจูเลียนหลัง epoch

    // ===== 1. คำนวณตำแหน่งโลก (Earth/Sun) =====
    // Mean Longitude ของโลก (องศา)
    let mut e_l = 100.46435 + 35999.37249 * t + 0.0003032 * t * t;
    e_l = e_l % 360.0;
    if e_l < 0.0 { e_l += 360.0; }
    
    // Perihelion ของโลก
    let mut e_g = 102.93768 + 0.32327 * t + 0.00015 * t * t;
    e_g = e_g % 360.0;
    if e_g < 0.0 { e_g += 360.0; }
    
    // Mean Anomaly ของโลก
    let e_m = (e_l - e_g).to_radians();
    
    // Equation of Center สำหรับโลก
    let e_eq_center = 1.914600 * e_m.sin() 
                    + 0.019993 * (2.0 * e_m).sin()
                    + 0.000290 * (3.0 * e_m).sin();
    
    // True Longitude ของโลก
    let e_v = e_l + e_eq_center;
    let e_v_rad = e_v.to_radians();
    
    // ระยะห่างโลก-ดวงอาทิตย์ (AU)
    let e_ecc = 0.016708617 - 0.000042037 * t;
    let e_r = 1.000001018 * (1.0 - e_ecc * e_ecc) / (1.0 + e_ecc * e_m.cos());

    // ===== 2. คำนวณวงโคจรดาวพฤหัสบดี (Heliocentric) =====
    
    // Mean Longitude ของดาวพฤหัสบดี ณ Epoch (องศา)
    // ค่าเริ่มต้น ณ 13 เม.ย. 1800: ประมาณ 34.3 องศา
    let mut j_l = 65.32108 + 3034.90567 * t + 0.00022374 * t * t;
    j_l = j_l % 360.0;
    if j_l < 0.0 { j_l += 360.0; }
    
    // Perihelion ของดาวพฤหัสบดี
    let mut j_perihelion = 14.75385 + 0.21252 * t + 0.00031097 * t * t;
    j_perihelion = j_perihelion % 360.0;
    if j_perihelion < 0.0 { j_perihelion += 360.0; }
    
    // Ascending Node ของดาวพฤหัสบดี
    let mut j_node = 100.55615 - 0.05237 * t - 0.00021819 * t * t;
    j_node = j_node % 360.0;
    if j_node < 0.0 { j_node += 360.0; }
    
    // Inclination (ความเอียงของวงโคจร)
    let j_inc = 1.30530 - 0.00155 * t; // องศา (เอียงน้อยมาก)
    
    // Mean Anomaly ของดาวพฤหัสบดี
    let j_m = (j_l - j_perihelion).to_radians();
    
    // Eccentricity ของดาวพฤหัสบดี (แปรตามเวลา)
    let j_ecc = 0.04839266 - 0.000013528 * t - 0.0000000864 * t * t;
    
    // Equation of Center สำหรับดาวพฤหัสบดี (4 พจน์ - เพราะ eccentricity น้อย)
    let j_eq_center = 5.55549 * j_m.sin()
                    + 0.16763 * (2.0 * j_m).sin()
                    + 0.00526 * (3.0 * j_m).sin()
                    + 0.00188 * (4.0 * j_m).sin();
    
    // True Anomaly
    let j_v_true = j_l + j_eq_center;
    
    // Argument of Latitude (u = v + ω - Ω)
    let j_arg_lat = (j_v_true - j_node).to_radians();
    
    // ระยะห่างดาวพฤหัสบดี-ดวงอาทิตย์ (AU)
    let j_a = 5.202561; // Semi-major axis
    let j_r = j_a * (1.0 - j_ecc * j_ecc) / (1.0 + j_ecc * j_m.cos());
    
    // ===== 3. คำนวณตำแหน่ง 3 มิติและแปลงเป็นระนาบอุปราศี =====
    
    let j_inc_rad = j_inc.to_radians();
    let j_node_rad = j_node.to_radians();
    
    // แปลงเป็นพิกัด heliocentric ecliptic
    let x_orb = j_r * j_arg_lat.cos();
    let y_orb = j_r * j_arg_lat.sin();
    
    // แปลงจากระนาบวงโคจรไปสู่ระนาบอุปราศี (ecliptic)
    let j_x_helio = (x_orb * j_node_rad.cos() - y_orb * j_inc_rad.cos() * j_node_rad.sin());
    let j_y_helio = (x_orb * j_node_rad.sin() + y_orb * j_inc_rad.cos() * j_node_rad.cos());
    let j_z_helio = (y_orb * j_inc_rad.sin());
    
    // ===== 4. แปลงเป็น Geocentric (มองจากโลก) =====
    
    // ตำแหน่งโลกในระบบ heliocentric
    let e_x = e_r * e_v_rad.cos();
    let e_y = e_r * e_v_rad.sin();
    let e_z = 0.0; // โลกอยู่ในระนาบอุปราศี
    
    // ตำแหน่งดาวพฤหัสบดีที่มองจากโลก
    let geo_x = j_x_helio - e_x;
    let geo_y = j_y_helio - e_y;
    let geo_z = j_z_helio - e_z;
    
    // Geocentric longitude (ลองจิจูดท้องฟ้า)
    let mut geocentric_long = geo_y.atan2(geo_x).to_degrees();
    
    // ===== 5. การแก้ไขเพิ่มเติม (Perturbations) =====
    // พจน์แก้ไขจากดาวเสาร์ (มีผลมากที่สุดต่อดาวพฤหัสบดี)
    
    // Mean longitude ของดาวเสาร์
    let saturn_mean_long = (49.9485 + 1222.1138 * t) % 360.0;
    
    // Perturbation terms (องศา) - ผลจากดาวเสาร์
    // Great Inequality: การเกิด resonance 5:2 ระหว่างดาวพฤหัสบดีและดาวเสาร์
    let pert_saturn_1 = 0.33033 * (5.0 * saturn_mean_long.to_radians() - 2.0 * j_l.to_radians() + 0.91330).sin();
    let pert_saturn_2 = 0.03304 * (5.0 * saturn_mean_long.to_radians() - 2.0 * j_l.to_radians() - 0.63863).sin();
    
    // ผลจากโลก (น้อยมาก)
    let pert_earth = 0.00204 * (e_l.to_radians() - j_l.to_radians()).sin();
    
    geocentric_long += pert_saturn_1 + pert_saturn_2 + pert_earth;
    
    // ===== 6. Nutation (การส่ายของแกนโลก) =====
    let omega = (125.04 - 1934.136 * t).to_radians();
    let nutation_long = -0.00569 - 0.00479 * omega.sin();
    
    geocentric_long += nutation_long;
    
    // ===== 7. ปรับค่าให้อยู่ในช่วง 0-360 องศา =====
    geocentric_long = geocentric_long % 360.0;
    if geocentric_long < 0.0 { 
        geocentric_long += 360.0; 
    }
    
    geocentric_long
}

// ดาวศุกร์ (๖): ดาววงใน เดินเกาะกลุ่มอาทิตย์ (ความเร็วเฉลี่ยเท่าอาทิตย์ แต่แกว่งได้ 47 องศา)
// ฟังก์ชันคำนวณดาวศุกร์ (๖) - ปรับปรุงแบบละเอียด
// อ้างอิง Epoch: 13 เมษายน 1800 เวลา 6:00 น. (GMT+7) = JD 2378493.458333
fn calculate_venus_position(jd: f64, solar_long: f64) -> f64 {
   
    // Epoch อ้างอิง: 13 เมษายน 1800 เวลา 06:00 GMT+7
    let epoch_1800_jd = 2378493.458333;
    let t = (jd - epoch_1800_jd) / 36525.0; // จำนวนศตวรรษจูเลียนหลัง epoch

    // ===== 1. คำนวณตำแหน่งโลก (Earth/Sun) =====
    // Mean Longitude ของโลก (องศา)
    let mut e_l = 100.46435 + 35999.37249 * t + 0.0003032 * t * t;
    e_l = e_l % 360.0;
    if e_l < 0.0 { e_l += 360.0; }
    
    // Perihelion ของโลก
    let mut e_g = 102.93768 + 0.32327 * t + 0.00015 * t * t;
    e_g = e_g % 360.0;
    if e_g < 0.0 { e_g += 360.0; }
    
    // Mean Anomaly ของโลก
    let e_m = (e_l - e_g).to_radians();
    
    // Equation of Center สำหรับโลก
    let e_eq_center = 1.914600 * e_m.sin() 
                    + 0.019993 * (2.0 * e_m).sin()
                    + 0.000290 * (3.0 * e_m).sin();
    
    // True Longitude ของโลก
    let e_v = e_l + e_eq_center;
    let e_v_rad = e_v.to_radians();
    
    // ระยะห่างโลก-ดวงอาทิตย์ (AU)
    let e_ecc = 0.016708617 - 0.000042037 * t;
    let e_r = 1.000001018 * (1.0 - e_ecc * e_ecc) / (1.0 + e_ecc * e_m.cos());

    // ===== 2. คำนวณวงโคจรดาวศุกร์ (Heliocentric) =====
    
    // Mean Longitude ของดาวศุกร์ ณ Epoch (องศา)
    // ค่าเริ่มต้น ณ 13 เม.ย. 1800 เวลา 06:00 น. (GMT+7): 331.03 องศา
    // Mean Motion: 58543.7986 องศา/ศตวรรษ (ปรับให้ได้ 174.25° ณ 3 ต.ค. 1987)
    let mut v_l = 331.03 + 58575.336 * t + 0.00052556 * t * t;
    v_l = v_l % 360.0;
    if v_l < 0.0 { v_l += 360.0; }
    
    // Perihelion ของดาวศุกร์
    let mut v_perihelion = 131.5637 + 0.04818 * t + 0.00013955 * t * t;
    v_perihelion = v_perihelion % 360.0;
    if v_perihelion < 0.0 { v_perihelion += 360.0; }
    
    // Ascending Node ของดาวศุกร์
    let mut v_node = 76.6799 - 0.04107 * t - 0.00013812 * t * t;
    v_node = v_node % 360.0;
    if v_node < 0.0 { v_node += 360.0; }
    
    // Inclination (ความเอียงของวงโคจร)
    let v_inc = 3.39471 - 0.00078 * t; // องศา (เอียงปานกลาง)
    
    // Mean Anomaly ของดาวศุกร์
    let v_m = (v_l - v_perihelion).to_radians();
    
    // Eccentricity ของดาวศุกร์ (แปรตามเวลา)
    // ดาวศุกร์มี eccentricity น้อยมาก (0.0068) - วงโคจรเกือบกลมสนิท!
    let v_ecc = 0.00682069 - 0.000047766 * t + 0.0000000975 * t * t;
    
    // Equation of Center สำหรับดาวศุกร์ (3 พจน์ - เพราะ eccentricity น้อยมาก)
    let v_eq_center = 0.77967 * v_m.sin()
                    + 0.00052 * (2.0 * v_m).sin()
                    + 0.00004 * (3.0 * v_m).sin();
    
    // True Anomaly
    let v_v_true = v_l + v_eq_center;
    
    // Argument of Latitude (u = v + ω - Ω)
    let v_arg_lat = (v_v_true - v_node).to_radians();
    
    // ระยะห่างดาวศุกร์-ดวงอาทิตย์ (AU)
    let v_a = 0.723332; // Semi-major axis
    let v_r = v_a * (1.0 - v_ecc * v_ecc) / (1.0 + v_ecc * v_m.cos());
    
    // ===== 3. คำนวณตำแหน่ง 3 มิติและแปลงเป็นระนาบอุปราศี =====
    
    let v_inc_rad = v_inc.to_radians();
    let v_node_rad = v_node.to_radians();
    
    // แปลงเป็นพิกัด heliocentric ecliptic
    let x_orb = v_r * v_arg_lat.cos();
    let y_orb = v_r * v_arg_lat.sin();
    
    // แปลงจากระนาบวงโคจรไปสู่ระนาบอุปราศี (ecliptic)
    let v_x_helio = (x_orb * v_node_rad.cos() - y_orb * v_inc_rad.cos() * v_node_rad.sin());
    let v_y_helio = (x_orb * v_node_rad.sin() + y_orb * v_inc_rad.cos() * v_node_rad.cos());
    let v_z_helio = (y_orb * v_inc_rad.sin());
    
    // ===== 4. แปลงเป็น Geocentric (มองจากโลก) =====
    
    // ตำแหน่งโลกในระบบ heliocentric
    let e_x = e_r * e_v_rad.cos();
    let e_y = e_r * e_v_rad.sin();
    let e_z = 0.0; // โลกอยู่ในระนาบอุปราศี
    
    // ตำแหน่งดาวศุกร์ที่มองจากโลก
    let geo_x = v_x_helio - e_x;
    let geo_y = v_y_helio - e_y;
    let geo_z = v_z_helio - e_z;
    
    // Geocentric longitude (ลองจิจูดท้องฟ้า)
    let mut geocentric_long = geo_y.atan2(geo_x).to_degrees();
    
    // ===== 5. การแก้ไขเพิ่มเติม (Perturbations) =====
    // พจน์แก้ไขจากดาวพฤหัสบดี (มีผลเล็กน้อย)
    
    // Mean longitude ของดาวพฤหัสบดี
    let jupiter_mean_long = (34.3515 + 3034.9056 * t) % 360.0;
    
    // Perturbation terms (องศา)
    let pert_jupiter = 0.00313 * (2.0 * jupiter_mean_long.to_radians() - 7.0 * v_l.to_radians() + 3.29).sin()
                     + 0.00198 * (3.0 * jupiter_mean_long.to_radians() - 8.0 * v_l.to_radians() + 2.33).sin();
    
    // ผลจากโลก (การรบกวนเล็กน้อย)
    let pert_earth = 0.00106 * (5.0 * e_l.to_radians() - 3.0 * v_l.to_radians() - 1.51).sin();
    
    geocentric_long += pert_jupiter + pert_earth;
    
    // ===== 6. Nutation (การส่ายของแกนโลก) =====
    let omega = (125.04 - 1934.136 * t).to_radians();
    let nutation_long = -0.00569 - 0.00479 * omega.sin();
    
    geocentric_long += nutation_long;
    
    // ===== 7. ปรับค่าให้อยู่ในช่วง 0-360 องศา =====
    geocentric_long = geocentric_long % 360.0;
    if geocentric_long < 0.0 { 
        geocentric_long += 360.0; 
    }
    
    geocentric_long

}

// ดาวเสาร์ (๗): ดาวที่เดินช้าที่สุดในดาวเดิม (ประมาณ 2.5 ปีต่อราศี)
fn calculate_saturn_position(jd: f64) -> f64 {
    let horakhun = jd - 2378600.45833;
    let mut mean_saturn = (102.32 + (0.033459 * horakhun)) % 360.0;
    // ปรับแก้สมการจุดศูนย์กลาง ประมาณ 6.5 องศา
    let anomaly = (mean_saturn - 90.0).to_radians();
    let mut true_saturn = mean_saturn + (6.58 * anomaly.sin());
    if true_saturn < 0.0 { true_saturn += 360.0; }
    true_saturn % 360.0
}

// ราหู (๘): เดินถอยหลังเสมอ (Retrograde) ประมาณ 1.5 ปีต่อราศี
fn calculate_rahu_position(jd: f64) -> f64 {
    let horakhun = jd - 2378600.45833;
    // สังเกตว่าความเร็วติดลบ และค่าเริ่มต้นอยู่ที่ประมาณ 110 องศา
    let mut rahu = (6.78 - (0.052955 * horakhun)) % 360.0;
    if rahu < 0.0 { rahu += 360.0; }
    rahu
}

// เกตุไทย (๙): เดินหน้าคงที่ (ความเร็ว 1 ราศี ประมาณ 2 เดือน)
fn calculate_ketu_thai_position(jd: f64) -> f64 {
    let horakhun = jd - 2378600.45833;
    // เกตุไทยเดินหน้าเร็วประมาณ 0.15 องศา/วัน
    let mut ketu = (152.57 + (0.145 * horakhun)) % 360.0;
    if ketu < 0.0 { ketu += 360.0; }
    ketu
}

// ดาวมฤตยู (๐): เดินช้ามาก (ความเร็ว 0.0117 องศา/วัน) ประมาณ 7 ปีต่อราศี
fn calculate_uranus_position(jd: f64) -> f64 {
    let horakhun = jd - 2378600.45833; // Epoch 1800
    // 25.0 คือตำแหน่งเริ่มต้นโดยประมาณ ณ ปี 1800
    let mut uranus = (152.87 + (0.011726 * horakhun)) % 360.0;
    if uranus < 0.0 { uranus += 360.0; }
    uranus
}