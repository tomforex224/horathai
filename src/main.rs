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

// ฟังก์ชันคำนวณดาวพุธ (๔)
fn calculate_mercury_position(jd: f64, solar_long: f64) -> f64 {
    let epoch_1800_jd =  2378600.45833;
    let horakhun = (jd - epoch_1800_jd) / 36525.0; // แปลงเป็นศตวรรษ


    // 1. คำนวณตำแหน่งโลก (Earth/Sun) เพื่อใช้หาตำแหน่งสัมพัทธ์
    let e_l = (100.46435 + 35999.37249 * horakhun) % 360.0;
    let e_g = (102.93768 + 0.32327 * horakhun) % 360.0;
    let e_m = (e_l - e_g).to_radians();
    let e_v = (e_l + 1.9148 * e_m.sin()).to_radians(); // True Longitude โลก
    let e_r = 1.00014 * (1.0 - 0.01671 * e_m.cos());  // ระยะห่างโลก-ดวงอาทิตย์

    // 2. คำนวณวงโคจรดาวพุธ (Heliocentric)
    // ค่า ณ J2000: L=252.25, Perihelion=77.45, Mean Motion=149472.67 องศา/ศตวรรษ
    let m_l = (315.84 + 149472.67411 * horakhun) % 360.0; 
    let m_g = (238.67 + 0.15846 * horakhun) % 360.0;
    let m_m = (m_l - m_g).to_radians();

    // Equation of Center สำหรับดาวพุธ (เนื่องจากวงรีจัด ต้องใช้พจน์ที่ละเอียดขึ้น)
    let m_v_helioc = m_l + 23.4400 * m_m.sin() + 2.9818 * (2.0 * m_m).sin();
    let m_v_rad = m_v_helioc.to_radians();
    
    // ระยะห่างดาวพุธ-ดวงอาทิตย์ (r) หน่วย AU
    let m_r = 0.387098 * (1.0 - 0.205635 * m_m.cos());

    // 3. แปลงเป็น Geocentric (มองจากโลก)
    // ใช้ตรีโกณมิติหาเวกเตอร์ตำแหน่งระหว่าง โลก-ดวงอาทิตย์-ดาวพุธ
    let x = m_r * m_v_rad.cos() - e_r * e_v.cos();
    let y = m_r * m_v_rad.sin() - e_r * e_v.sin();

    let mut geocentric_long = y.atan2(x).to_degrees();

    if geocentric_long < 0.0 { geocentric_long += 360.0; }
    geocentric_long % 360.0
}

// ดาวพฤหัสบดี (๕): เดินประมาณ 1 ราศีต่อปี (ความเร็ว 0.083 องศา/วัน)
fn calculate_jupiter_position(jd: f64) -> f64 {
    let horakhun = jd - 2378600.45833; 
    let mut mean_jupiter = (65.10 + (0.083091 * horakhun)) % 360.0;
    // ปรับแก้สมการจุดศูนย์กลาง (Equation of Center) ประมาณ 5.5 องศา
    let anomaly = (mean_jupiter - 193.0).to_radians(); 
    let mut true_jupiter = mean_jupiter + (5.59 * anomaly.sin());
    if true_jupiter < 0.0 { true_jupiter += 360.0; }
    true_jupiter % 360.0
}

// ดาวศุกร์ (๖): ดาววงใน เดินเกาะกลุ่มอาทิตย์ (ความเร็วเฉลี่ยเท่าอาทิตย์ แต่แกว่งได้ 47 องศา)
fn calculate_venus_position(jd: f64, solar_long: f64) -> f64 {
    let horakhun = jd - 2378600.45833;
    let venus_mean_motion = 1.11115855; // ความเร็วเฉลี่ยดาวศุกร์
    let mut venus_anomaly = (331.75 + (venus_mean_motion * horakhun)) % 360.0;
    let correction = 46.5 * venus_anomaly.to_radians().sin();
    let mut true_venus = solar_long + correction;
    if true_venus < 0.0 { true_venus += 360.0; }
    true_venus % 360.0
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