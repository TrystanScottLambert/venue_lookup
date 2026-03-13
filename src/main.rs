use std::io::{self, Write, stdin, stdout};

/// Simple fuzzy match: checks if all characters in the pattern appear
/// in order in the target (case-insensitive). Returns a score (lower = better)
/// based on gap penalty, or None if no match.
fn fuzzy_score(pattern: &str, target: &str) -> Option<i32> {
    let pattern_lower: Vec<char> = pattern.to_lowercase().chars().collect();
    let target_lower: Vec<char> = target.to_lowercase().chars().collect();

    if pattern_lower.is_empty() {
        return Some(0);
    }

    let mut pi = 0;
    let mut score: i32 = 0;
    let mut last_match: Option<usize> = None;
    let mut first_match: Option<usize> = None;

    for (ti, tc) in target_lower.iter().enumerate() {
        if pi < pattern_lower.len() && *tc == pattern_lower[pi] {
            if first_match.is_none() {
                first_match = Some(ti);
            }
            // Bonus for consecutive matches
            if let Some(last) = last_match {
                if ti == last + 1 {
                    score -= 5; // reward consecutive
                } else {
                    score += (ti - last) as i32; // penalise gaps
                }
            }
            // Bonus for matching at word boundary
            if ti == 0 || matches!(target_lower.get(ti - 1), Some(' ' | '.' | ':' | '[' | '-')) {
                score -= 10;
            }
            last_match = Some(ti);
            pi += 1;
        }
    }

    if pi == pattern_lower.len() {
        // Penalise matches that start late
        score += first_match.unwrap_or(0) as i32;
        Some(score)
    } else {
        None
    }
}

const VENUES: &[(&str, &str)] = &[
    (
        "103.G01",
        "103.G01 HACKETT HALL.  Margaret Fairweather Education Space",
    ),
    ("103.G09", "103.G09 HACKETT HALL.  Fay Gale Studio"),
    ("106.131", "106.131 ARTS.Learning Space"),
    ("106.132", "106.132 ARTS.Learning Space"),
    ("106.133", "106.133 ARTS.Phillipa Maddern Seminar Room"),
    ("106.154", "106.154 ARTS.Electronic Music Laboratory"),
    ("106.159", "106.159 ARTS.Austin Lecture Theatre"),
    ("106.160", "106.160 ARTS.Lecture Room 8"),
    ("106.161", "106.161 ARTS.Lecture Room 9"),
    ("106.162", "106.162 ARTS.Lecture Room 10"),
    ("106.231", "106.231 ARTS.Technical Training Lab"),
    ("106.233", "106.233 ARTS.Mac Studio"),
    ("106.243", "106.243 ARTS.Classics Reading Room"),
    ("106.G01", "106.G01 ARTS.  Seminar Room"),
    ("106.G03", "106.G03 ARTS.  Seminar room"),
    ("106.G05", "106.G05 ARTS.  Seminar Room"),
    ("106.G07", "106.G07 ARTS.  Seminar Room"),
    ("106.G27", "106.G27 ARTS.  Seminar Room"),
    ("106.G28", "106.G28 ARTS.  Seminar Room"),
    ("106.G29", "106.G29 ARTS.  Seminar Room"),
    ("106.G57", "106.G57 ARTS.  Alexander Lecture Theatre"),
    ("106.G58", "106.G58 ARTS.  Murdoch Lecture Theatre"),
    ("106.G59", "106.G59 ARTS.  Fox Lecture Theatre"),
    ("106.G60", "106.G60 ARTS.  Arts Lecture Room 4"),
    ("106.G61", "106.G61 ARTS.  Arts Lecture Room 5"),
    ("106.G62", "106.G62 ARTS.  Arts Lecture Room 6"),
    ("139.106", "139.106 REID LIBRARY.Seminar Room"),
    ("142.G05", "142.G05 MUSIC.  Tunley Lecture Theatre"),
    ("143.G11", "143.G11 OCTAGON.  Lecture Theatre"),
    (
        "210.G106",
        "210.G106 WILSMORE TATTERSALL.  Tattersall Lecture Theatre",
    ),
    (
        "210.G108",
        "210.G108 WILSMORE TATTERSALL.  Wilsmore Lecture Theatre",
    ),
    ("211.113", "211.113 BAYLISS.Laboratory A"),
    ("211.115", "211.115 BAYLISS.Laboratory B"),
    ("211.119", "211.119 BAYLISS.Laboratory C"),
    ("211.120", "211.120 BAYLISS.Seminar Room"),
    ("211.121", "211.121 BAYLISS.Laboratory D"),
    ("211.130", "211.130 BAYLISS. Computer Lab"),
    ("211.131", "211.131 BAYLISS 3rd Yr Biochemistry Laboratory"),
    ("211.142", "211.142 BAYLISS.Second Year Biochemistry Lab"),
    ("211.215", "211.215 BAYLISS.Seminar Room"),
    ("211.217", "211.217 BAYLISS.Laboratory E"),
    ("211.219", "211.219 BAYLISS.Laboratory F"),
    ("211.223", "211.223 BAYLISS.Chemistry Laboratory"),
    ("211.225", "211.225 BAYLISS.Chemistry Laboratory"),
    ("211.240", "211.240 BAYLISS.Physical Chemistry Lab"),
    ("211.G33", "211.G33 BAYLISS.Lecture Theatre"),
    ("211.G35", "211.G35 BAYLISS.Seminar Room"),
    (
        "222.109",
        "222.109 EZONE NORTH.Learning Studio 1 (Chunyart Noorook)",
    ),
    ("222.110", "222.110 EZONE NORTH.Learning Studio 2"),
    ("222.111", "222.111 EZONE NORTH.Learning Studio 3"),
    ("222.113", "222.113 EZONE NORTH.  Industry Hub"),
    (
        "222.209",
        "222.209 EZONE NORTH.Learning Studio 1 (Bungarra Noorook)",
    ),
    ("222.210", "222.210 EZONE NORTH.Learning Studio"),
    ("222.211", "222.211 EZONE NORTH.Learning Studio 3"),
    ("223.123", "223.123 MATHEMATICS.Computer Laboratory"),
    ("223.151", "223.151 MATHEMATICS.Lycopodium Design Studio"),
    ("223.152", "223.152 MATHEMATICS.Design Studio"),
    ("223.G02", "223.G02 MATHEMATICS.  Lecture Room 3"),
    (
        "223.G17",
        "223.G17 MATHEMATICS.  Cheryl Praeger Lecture Room",
    ),
    ("223.G18", "223.G18 MATHEMATICS.  Blakers Lecture Room"),
    ("223.G19", "223.G19 MATHEMATICS.  Lecture Room 2"),
    (
        "223.G40",
        "223.G40 MATHEMATICS.  Weatherburn Lecture Theatre",
    ),
    ("224.105", "224.105 CIVIL & MECH ENG.Seminar Room"),
    ("224.111", "224.111 CIVIL & MECH ENG.Computer Lab"),
    ("224.151", "224.151 CIVIL & MECH ENG.Lecture Room"),
    ("224.157", "224.157 CIVIL & MECH ENG.Gas Dynamics Lab"),
    ("224.207", "224.207 CIVIL & MECH ENG.Computer Lab"),
    ("224.245", "224.245 CIVIL & MECH ENG.Lecture Room"),
    ("224.261", "224.261 CIVIL & MECH ENG.Lecture Room"),
    ("224.267", "224.267 CIVIL & MECH ENG.Drawing Room"),
    (
        "224.G04",
        "224.G04 CIV MECH ENGINEERING.  H & M Clough Lecture Theatre",
    ),
    (
        "224.G06",
        "224.G06 CIV MECH ENGINEERING.  H & M Clough Lecture Theatre",
    ),
    ("224.G11", "224.G11 CIV MECH ENGINEERING.  Lecture Room"),
    ("224.G13", "224.G13 CIV MECH ENGINEERING.  Lecture Room"),
    (
        "225.107",
        "225.107 GEOGRAPHY & GEOLOGY.Woolnough Lecture Theatre",
    ),
    ("225.123", "225.123 GEOGRAPHY & GEOLOGY.Geography Lab"),
    (
        "225.131",
        "225.131 GEOGRAPHY & GEOLOGY.Gentilli Lecture Theatre",
    ),
    (
        "225.140",
        "225.140 GEOGRAPHY & GEOLOGY.North Undergraduate Computer Lab",
    ),
    (
        "225.G21",
        "225.G21 GEOGRAPHY & GEOLOGY.  Webb Lecture Theatre",
    ),
    ("226.151", "226.151 ELECTRONIC ENG.Computer Lab 1"),
    ("226.271", "226.271 ELECTRONIC ENG.Computer Lab"),
    ("227.101", "227.101 SANDERS.Computer Lab 1"),
    ("227.216", "227.216 SANDERS.Computer Lab"),
    ("227.220", "227.220 SANDERS.Seminar Room"),
    ("227.G05", "227.G05 SANDERS.  Computer lab"),
    ("227.G06", "227.G06 SANDERS.  Seminar Room"),
    ("227.G09", "227.G09 SANDERS.  Seminar Room"),
    ("227.G10", "227.G10 SANDERS.  Seminar Room"),
    ("235.201", "235.201 GENERAL PURPOSE 3.Seminar Room"),
    ("235.202", "235.202 GENERAL PURPOSE 3.Lecture Room"),
    (
        "235.G01",
        "235.G01 GENERAL PURPOSE 3.  Simmonds Lecture Theatre",
    ),
    ("238.203", "238.203 MYERS ST.Seminar Room"),
    ("238.205", "238.205 MYERS ST.Case Study Room 2"),
    ("238.206", "238.206 MYERS ST.Lecture Theatre"),
    ("241.124", "241.124 COMPUTER SCIENCE.Seminar Room"),
    ("241.201", "241.201 COMPUTER SCIENCE.Computer Lab"),
    ("241.203", "241.203 COMPUTER SCIENCE.Computer Lab"),
    ("241.205", "241.205 COMPUTER SCIENCE.Computer Lab"),
    ("241.207", "241.207 COMPUTER SCIENCE.Seminar Room"),
    ("241.228", "241.228 COMPUTER SCIENCE.Tutorial Room"),
    (
        "241.G09",
        "241.G09 COMPUTER SCIENCE.  Internet of Things Lab",
    ),
    ("241.G11", "241.G11 COMPUTER SCIENCE.  Cybersecurity Lab"),
    ("242.102", "242.102 CO2 RESEARCH.Chemical Engineering Lab"),
    ("242.104", "242.104 CO2 RESEARCH.Laboratory"),
    ("245.116", "245.116 PHYSICS.First Year Dark Lab"),
    ("245.119", "245.119 PHYSICS.Magnetic Lab"),
    ("245.120", "245.120 PHYSICS.Laboratory"),
    ("245.121", "245.121 PHYSICS.Air Tables Laboratory"),
    ("245.128", "245.128 PHYSICS.Laboratory"),
    ("245.215", "245.215 PHYSICS.Crawford Lecture Room"),
    ("245.216", "245.216 PHYSICS.Physics Seminar Room"),
    ("245.217", "245.217 PHYSICS.Physics Tutorial Room"),
    ("245.226", "245.226 PHYSICS.Third Year Laboratory"),
    ("245.243", "245.243 PHYSICS.Clews Lecture Theatre"),
    ("245.501", "245.501 PHYSICS.Meeting Room"),
    ("245.554", "245.554 PHYSICS.Laboratory"),
    ("245.G41", "245.G41 PHYSICS.  Ross Lecture Theatre"),
    ("272.111", "272.111 ROBERT STREET.Third Year Lab"),
    ("272.G16", "272.G16 ROBERT STREET.  Lecture Theatre"),
    ("275.101", "275.101 EZONE CENTRAL.Meeting Room"),
    ("275.105", "275.105 EZONE CENTRAL.Learning Studio 1 (Carda)"),
    ("275.106", "275.106 EZONE CENTRAL.Learning Studio 2"),
    ("275.201", "275.201 EZONE CENTRAL.Meeting Room"),
    (
        "275.206",
        "275.206 EZONE CENTRAL.Giumelli Learning Studio 1 (Carda Noorook)",
    ),
    (
        "275.207",
        "275.207 EZONE CENTRAL.Giumelli Learning Studio 2",
    ),
    (
        "275.209",
        "275.209 EZONE CENTRAL.Giumelli Learning Studio 3",
    ),
    (
        "275.210",
        "275.210 EZONE CENTRAL.Giumelli Learning Studio 4 (Bidjuk Noorook)",
    ),
    (
        "275.212",
        "275.212 EZONE CENTRAL.Giumelli Learning Studio 5",
    ),
    (
        "275.213",
        "275.213 EZONE CENTRAL.Giumelli Learning Studio 6",
    ),
    ("338.105", "338.105 LAW.Seminar Room 3"),
    ("338.106", "338.106 LAW.Lecture Theatre"),
    ("338.207", "338.207 LAW.Seminar Room 4"),
    ("338.G05", "338.G05 LAW.  Lecture Room 3"),
    ("338.G06", "338.G06 LAW.Moot Court"),
    ("338.G07", "338.G07 LAW.  Lecture Room 2"),
    ("338.G31", "338.G31 LAW.  Lecture Room 1"),
    ("344.181", "344.181 ANATOMY.Seminar Room"),
    ("345.101", "345.101 CURNOW.Seminar Room"),
    ("345.207", "345.207 CURNOW.Tutorial Room"),
    ("345.209", "345.209 CURNOW.Laboratory"),
    ("346.208", "346.208 PHYSIOLOGY.Seminar Room"),
    ("346.210", "346.210 PHYSIOLOGY.Laboratory"),
    ("347.233", "347.233 PSYCHOLOGY.Lecture Room"),
    ("347.314", "347.314 PSYCHOLOGY.Learning Space"),
    ("347.G26", "347.G26 PSYCHOLOGY.  Computer Lab"),
    ("347.G40", "347.G40 PSYCHOLOGY.  Seminar Room"),
    ("347.G41", "347.G41 PSYCHOLOGY.  Seminar Room"),
    ("347.G42", "347.G42 PSYCHOLOGY.  Seminar Room"),
    ("348.G11", "348.G11 BILYA MARLEE.  Boolong (Pelican) Room"),
    ("351.193", "351.193 SOCIAL SCI & LAWLINK.Archeology Lab"),
    ("351.294", "351.294 SOCIAL SCI & LAWLINK.PG Teaching Room"),
    ("351.373", "351.373 SOCIAL SCI & LAWLINK.Conference Room"),
    ("352.110", "352.110 SOCIAL SCI & STU CEN.Seminar Room"),
    (
        "352.125",
        "352.125 SOCIAL SCI & STU CEN.Forensic Anthropology Lab",
    ),
    ("352.129", "352.129 SOCIAL SCI & STU CEN.Lecture Room 2"),
    ("352.149", "352.149 SOCIAL SCI & STU CEN.Seminar Room"),
    ("352.161", "352.161 SOCIAL SCI & STU CEN.Seminar Room"),
    ("352.217", "352.217 SOCIAL SCI & STU CEN.Seminar Room"),
    ("352.2202", "352.2202 SOCIAL SCI & STU CEN.Seminar Room"),
    ("352.2203", "352.2203 SOCIAL SCI & STU CEN.Seminar Room"),
    ("352.G130", "352.G130 SOCIAL SCIENCE.  Lecture Theatre"),
    ("352.G25", "352.G25 SOCIAL SCIENCE.  Seminar Room"),
    ("352.G28", "352.G28 SOCIAL SCIENCE.  Lecture Room"),
    (
        "401.G013",
        "401.G013 AGRICULTURE NORTH.  Alan Robson Lecture Theatre",
    ),
    ("409.108", "409.108 BOTANY & BIOLOGY.Laboratory"),
    ("409.109", "409.109 BOTANY & BIOLOGY.Laboratory"),
    ("409.205", "409.205 BOTANY & BIOLOGY.Laboratory"),
    ("409.206", "409.206 BOTANY & BIOLOGY.Laboratory"),
    ("409.214", "409.214 BOTANY & BIOLOGY.Seminar Room"),
    (
        "420.G10",
        "420.G10 ZOOLOGY  .  Jennifer M Arnold Lecture Theatre",
    ),
    (
        "441.101",
        "441.101 BUSINESS SCHOOL.Don Voelte & Nancy Keegan Case Study Room",
    ),
    (
        "441.124",
        "441.124 BUSINESS SCHOOL.Rosemarie Nathanson Financial Markets Room",
    ),
    (
        "441.142",
        "441.142 BUSINESS SCHOOL.Sir Rod Eddington Case Study Room",
    ),
    ("441.160", "441.160 BUSINESS SCHOOL.Hawaiian Tutorial Room"),
    ("441.161", "441.161 BUSINESS SCHOOL.Cullity Tutorial Room"),
    (
        "441.162",
        "441.162 BUSINESS SCHOOL.John Poynton Tutorial Room",
    ),
    ("441.163", "441.163 BUSINESS SCHOOL.Mitsui Tutorial Room"),
    (
        "441.164",
        "441.164 BUSINESS SCHOOL.Professor Philip Brown Tutorial Room",
    ),
    (
        "441.201",
        "441.201 BUSINESS SCHOOL.Adrian & Michela Fini Case Study Room",
    ),
    (
        "441.242",
        "441.242 BUSINESS SCHOOL.John Poynton AO Case Study Room",
    ),
    (
        "441.260",
        "441.260 BUSINESS SCHOOL.Sir Bruce Mackinlay Tutorial Room",
    ),
    (
        "441.262",
        "441.262 BUSINESS SCHOOL.Rick Crabb Tutorial Room",
    ),
    ("441.263", "441.263 BUSINESS SCHOOL.Tutorial Room"),
    ("441.264", "441.264 BUSINESS SCHOOL.Tutorial Room"),
    (
        "441.G02",
        "441.G02 BUSINESS SCHOOL.  Tony Howarth AO Case Study Room",
    ),
    (
        "441.G42",
        "441.G42 BUSINESS SCHOOL.  Michael Chaney AO Case Study Room",
    ),
    (
        "441.G89",
        "441.G89 BUSINESS SCHOOL.  Stan Perron AC Lecture Theatre",
    ),
    (
        "441.G91",
        "441.G91 BUSINESS SCHOOL.  Westfarmers Lecture Theatre",
    ),
    (
        "444.102",
        "444.102 HUMAN MOVEMENT.John Bloomfield Lecture Theatre",
    ),
    (
        "444.1104",
        "444.1104 HUMAN MOVEMENT.Exercise Physiology Laboratory",
    ),
    (
        "444.1105",
        "444.1105 HUMAN MOVEMENT.Exercise Biochemistry Lab",
    ),
    ("444.131", "444.131 HUMAN MOVEMENT.Exercise Gaming Lab"),
    ("444.134", "444.134 HUMAN MOVEMENT.UG Computer Lab"),
    ("444.198", "444.198 HUMAN MOVEMENT.Robin Gray Lecture Room"),
    ("453.124", "453.124 IOMRC.Woodside Lounge"),
    ("453.202", "453.202 IOMRC.Meeting Room"),
    ("453.G07", "453.G07 IOMRC.  Auditorium"),
    ("502.118", "502.118 QEII-M BLOCK.Seminar Room"),
    (
        "502.B09",
        "502.B09 QEII - M BLOCK.  Cameron Tutorial Room B",
    ),
    ("502.B10", "502.B10 QEII - M BLOCK.  Tutorial Room F"),
    (
        "503.101",
        "503.101 QEII-L BLOCK.Geoffrey Shellam Seminar Room",
    ),
    (
        "507.G02",
        "507.G02 QEII - P BLOCK.  FJ Clarke Lecture Theatre",
    ),
    (
        "507.G14",
        "507.G14 QEII - P BLOCK.  Mary Lockett Lecture Theatre",
    ),
    ("658.G040", "658.G040 KEN & JULIE MICHAEL.  Lecture Theatre"),
    ("658.G041", "658.G041 KEN & JULIE MICHAEL.  Conference Room"),
    ("661.106", "661.106 PARK AVE MAIN.Tutorial Room"),
    ("661.119", "661.119 PARK AVE MAIN.Clinical Suite 2"),
    (
        "661.121",
        "661.121 PARK AVE MAIN.Patient Prep Room Clinical Suite 3",
    ),
    ("661.125", "661.125 PARK AVE MAIN.Simulation Space"),
    ("661.G20", "661.G20 PARK AVENUE.  Lecture Theatre"),
    ("686.102", "686.102 HEW ROBERTS.  Lecture Theatre"),
    ("686.G01", "686.G01 HEW ROBERTS.  Seminar Room"),
    (
        "AGRI: [  G017]",
        "AGRI: [  G017] Central Undergrad Computer Lab",
    ),
    ("AGRI: [  G205]", "AGRI: [  G205] WET Lab C"),
    ("AGRI: [  G215]", "AGRI: [  G215] WET Lab D"),
    ("AGRI: [  G221]", "AGRI: [  G221] WET Lab B"),
    ("AGRI: [  G235]", "AGRI: [  G235] WET Lab A"),
    ("AHBL: [  G02]", "AHBL: [  G02] Dental Lab"),
    ("AHBL: [  G03]", "AHBL: [  G03] Histology Computer Lab"),
    ("AHBL: [  G05]", "AHBL: [  G05] Dissecting Room"),
    ("AHBL: [  G29]", "AHBL: [  G29] Tutorial Room"),
    ("AHBL: [  G30]", "AHBL: [  G30] Tutorial Room"),
    ("AHBL: [  G35]", "AHBL: [  G35] Tutorial Room"),
    (
        "ALBGSMRF: [  G03]",
        "ALBGSMRF: [  G03] GSMRF Boardroom (10)",
    ),
    (
        "ALBGSMRF: [  G04]",
        "ALBGSMRF: [  G04] GSMRF Meeting Room (8)",
    ),
    ("ALBPP: [ 105]", "ALBPP: [ 105] Tutorial Room (10)"),
    ("ALBPP: [ 111]", "ALBPP: [ 111] Teaching Computer Lab (15)"),
    ("ALBPP: [ 112]", "ALBPP: [ 112] Seminar Room (10)"),
    ("ALBSCI: [ 2002]", "ALBSCI: [ 2002] WET Lab (10)"),
    ("ALBSCI: [ 2006]", "ALBSCI: [ 2006] Laboratory (24)"),
    ("ALBSCI: [ 201]", "ALBSCI: [ 201] Tutorial (11)"),
    ("ALBSCI: [ 301]", "ALBSCI: [ 301] Tutorial (10)"),
    ("ALBSCI: [ 302]", "ALBSCI: [ 302] Seminar A (10)"),
    ("ALBSCI: [ 303]", "ALBSCI: [ 303] Seminar B (10)"),
    ("ALVA: [  G22]", "ALVA: [  G22] ALVA Lecture Room"),
    ("ALVA: [ 101A]", "ALVA: [ 101A] Visual Arts Studio"),
    ("ALVA: [ 101B]", "ALVA: [ 101B] Visual Arts Studio"),
    ("ALVA: [ 108]", "ALVA: [ 108] Mac Computer Lab"),
    ("ALVA: [ 110]", "ALVA: [ 110] Visual Arts Graphics Workshop"),
    ("ALVA: [ 205]", "ALVA: [ 205] Seminar Room"),
    ("ALVA: [ 206]", "ALVA: [ 206] Design Studio"),
    ("ALVA: [ 210]", "ALVA: [ 210] Design Studio"),
    ("ALVA: [ 215]", "ALVA: [ 215] Design Studio"),
    ("ALVA: [ 219]", "ALVA: [ 219] Design Studio"),
    ("ALVA: [ 220]", "ALVA: [ 220] Seminar Room"),
    ("ALVA: [ 305]", "ALVA: [ 305] Seminar Room"),
    ("ALVA: [ 306]", "ALVA: [ 306] Design Studio"),
    ("ALVA: [ 310]", "ALVA: [ 310] Design Studio"),
    ("ALVA: [ 315]", "ALVA: [ 315] Design Studio"),
    ("ALVA: [ 319]", "ALVA: [ 319] Design Studio"),
    ("ALVA: [ 320]", "ALVA: [ 320] Seminar Room"),
    ("ALVA: [ 405]", "ALVA: [ 405] Computer Lab"),
    ("ALVA: [ 408]", "ALVA: [ 408] Computer Lab"),
    ("ALVA: [ 411]", "ALVA: [ 411] Design Studio"),
    ("ALVA: [ 412]", "ALVA: [ 412] Design Studio"),
    ("ALVAST: [  G01A]", "ALVAST: [  G01A] The Hub Studio"),
    ("ALVAST: [  G01B]", "ALVAST: [  G01B] The Hub Studio"),
    ("ALVAST: [  G01]", "ALVAST: [  G01] The Hub Studio"),
    ("ARTS: [  G23]", "ARTS: [  G23] Meeting Room"),
    (
        "ARTS: [ 227]",
        "ARTS: [ 227] General purpose teaching space",
    ),
    ("BAYL: [  G32]", "BAYL: [  G32] Biochemistry Computer Lab"),
    ("BMARLEE: [  G01]", "BMARLEE: [  G01] Yaagin room"),
    ("BMARLEE: [  UG12]", "BMARLEE: [  UG12] Waitch Room"),
    ("BOBI: [  G06]", "BOBI: [  G06] WET Lab"),
    ("BOBI: [  G07]", "BOBI: [  G07] WET Lab"),
    (
        "BOBIA1: [  G11]",
        "BOBIA1: [  G11] East Undergrad Computer Lab",
    ),
    (
        "BOBIA2: [  G09]",
        "BOBIA2: [  G09] Experimental Outdoor Teaching Space",
    ),
    (
        "BUSN: [  G01]",
        "BUSN: [  G01] Professor David Plowman Syndicate Room",
    ),
    (
        "BUSN: [  G03]",
        "BUSN: [  G03] Professor Geoff Soutar Syndicate Room",
    ),
    ("BUSN: [  G04.G05]", "BUSN: [  G04.G05] Seminar Room"),
    (
        "BUSN: [  G08]",
        "BUSN: [  G08] Professor Andre Morkel Syndicate Room",
    ),
    ("BUSN: [  G85]", "BUSN: [  G85] Mitsubishi Computer Lab 1"),
    ("BUSN: [  G86]", "BUSN: [  G86] Mitsubishi Computer Lab 2"),
    ("BUSN: [  G87]", "BUSN: [  G87] Mitsubishi Computer Lab 3"),
    ("BUSN: [  G88]", "BUSN: [  G88] Prof Izan Seminar Room"),
    ("BUSN: [ 228]", "BUSN: [ 228] Syndicate Room 2"),
    (
        "BUSN: [ 229]",
        "BUSN: [ 229] Schaffer Corporations Syndicate Room",
    ),
    (
        "BUSN: [ 230]",
        "BUSN: [ 230] James McClement's Syndicate Room",
    ),
    ("BUSN: [ 231]", "BUSN: [ 231] McCusker Syndicate Room"),
    ("CLMT1: [  G10]", "CLMT1: [  G10] Kurrajong Seminar Room"),
    ("CSCK: [  G40]", "CSCK: [  G40] Office"),
    ("CSSE: [  G01 ]", "CSSE: [  G01 ]  IDEA Lab"),
    ("CSSE: [  G01A]", "CSSE: [  G01A] IDEA Lab"),
    ("CSSE: [  G01B]", "CSSE: [  G01B] IDEA Lab"),
    ("CURN: [ 203A]", "CURN: [ 203A] Tutorial Room"),
    ("CURN: [ 203B]", "CURN: [ 203B] Tutorial Room"),
    ("CURN: [ 203C]", "CURN: [ 203C] Tutorial Room"),
    ("CURN: [ 203D]", "CURN: [ 203D] Tutorial Room"),
    ("CURN: [ 203E]", "CURN: [ 203E] Tutorial Room"),
    ("DOLPHIN: [ G01]", "DOLPHIN: [ G01] Dolphin Theatre"),
    ("EDUC: [  G08]", "EDUC: [  G08] Lecture Room"),
    ("EDUC: [  G09]", "EDUC: [  G09] Lecture Room"),
    ("EDUC: [  G10]", "EDUC: [  G10] Lecture Room"),
    ("EDUC: [  G11]", "EDUC: [  G11] Learning Space"),
    ("EDUC: [  G19]", "EDUC: [  G19] Informal Study Space"),
    ("EDUC: [  G26]", "EDUC: [  G26] Computer Lab"),
    ("EDUC: [ 103]", "EDUC: [ 103] Learning Space"),
    ("EDUC: [ 105]", "EDUC: [ 105] Learning Space"),
    ("EDUC: [ 113]", "EDUC: [ 113] Seminar Room"),
    (
        "EDUC: [ 117]",
        "EDUC: [ 117] ALVA BIM Master Suit Computer Lab",
    ),
    ("EDUC: [ 120]", "EDUC: [ 120] ALVA Master Studio"),
    ("EDUC: [ 234]", "EDUC: [ 234] Conference Room"),
    ("EDUC: [ 235]", "EDUC: [ 235] Meeting Room (0)"),
    ("ENCM: [  G01]", "ENCM: [  G01] Clough Robotics Lab"),
    ("ENCM: [  G16]", "ENCM: [  G16] Nuwar Lab"),
    ("ENCM: [  G23]", "ENCM: [  G23] CAD/CAM Computer Lab"),
    ("ENCM: [  G27]", "ENCM: [  G27] Manufacturing Lab"),
    ("ENCM: [  G50K]", "ENCM: [  G50K] Thermodynamics Lab"),
    ("ENCM: [  G50M]", "ENCM: [  G50M] Mechanical Lab"),
    ("ENCM: [  G53]", "ENCM: [  G53] Wind Tunnel Lab"),
    ("ENCM: [  G60]", "ENCM: [  G60] Mechanical Workshop"),
    ("ENCM: [  G93D]", "ENCM: [  G93D] WET Lab Research"),
    ("ENCM: [ 109]", "ENCM: [ 109] Lecture Room"),
    ("ENCM: [ 113]", "ENCM: [ 113] Lecture Room"),
    ("ENCM: [ 207A]", "ENCM: [ 207A] North Civil Computer Room A"),
    ("ENCM: [ 207B]", "ENCM: [ 207B] South Civil Computer Room B"),
    (
        "EZONECENT: [  G04]",
        "EZONECENT: [  G04] Student Project Area",
    ),
    (
        "EZONECENT: [  G05]",
        "EZONECENT: [  G05] Materials Testing Lab",
    ),
    ("EZONECENT: [  G07]", "EZONECENT: [  G07] Hydraulics Lab"),
    ("EZONENTH: [  G03]", "EZONENTH: [  G03] EZONE Social"),
    (
        "EZONENTH: [  G10]",
        "EZONENTH: [  G10] Specialist Hardware Lab",
    ),
    ("EZONENTH: [  G11]", "EZONENTH: [  G11] Thermodynamics Lab"),
    ("GGGL: [  G01]", "GGGL: [  G01] 1st Year Geology Lab"),
    ("GGGL: [  G09]", "GGGL: [  G09] Masters Seminar Room"),
    ("GGGL: [  G13]", "GGGL: [  G13] Mapping Lab"),
    ("GGGL: [  G19]", "GGGL: [  G19] Geography Seminar Room 1"),
    ("GPB3: [  G09]", "GPB3: [  G09] Seminar Room"),
    ("IRWIN: [  G11]", "IRWIN: [  G11] Convocation Council Room"),
    ("LAWS: [  G12]", "LAWS: [  G12] E- Moot Court"),
    ("LOVH: [  G02]", "LOVH: [  G02] Love House Seminar Room"),
    (
        "MATH: [  G51]",
        "MATH: [  G51] Monadelphous Integrated Learning Centre Lab",
    ),
    (
        "MATH: [  G52]",
        "MATH: [  G52] Monadelphous Integrated Learning Centre Lab",
    ),
    ("MATH: [ 123AB]", "MATH: [ 123AB] Computer Lab - Nets A**B"),
    ("MATH: [ 123A]", "MATH: [ 123A] Computer Lab - Net A"),
    ("MATH: [ 123B]", "MATH: [ 123B] Computer Lab - Net B"),
    ("MATH: [ 123CD]", "MATH: [ 123CD] Computer Lab - Nets C**D"),
    ("MATH: [ 123C]", "MATH: [ 123C] Computer Lab - Net C"),
    ("MATH: [ 123D]", "MATH: [ 123D] Computer Lab - Net D"),
    ("MUSI: [  G10]", "MUSI: [  G10] Callaway Music Auditorium"),
    ("MUSI: [  G12]", "MUSI: [  G12]  Practice Room"),
    ("MUSI: [  G14]", "MUSI: [  G14] Eileen Joyce Studio"),
    ("MUSI: [  G15]", "MUSI: [  G15] Wigmore Studio"),
    ("MUSI: [  G18]", "MUSI: [  G18] Music Keyboard Lab"),
    ("MUSI: [  G20]", "MUSI: [  G20] Percussion Studio"),
    ("NCAFE: [  G17]", "NCAFE: [  G17] Computer Lab"),
    ("NCLF: [  G01]", "NCLF: [  G01] Learning Space"),
    ("NCLF: [  G02]", "NCLF: [  G02] Learning Space"),
    ("OCTA: [  G17]", "OCTA: [  G17] Bradley Studio"),
    (
        "OHCWA: [  G15]",
        "OHCWA: [  G15] Kenneth J G Sutherland Lecture Theatre",
    ),
    ("OUTD: [  G01]", "OUTD: [  G01] Exercise Lab 3"),
    ("OUTD: [  JAMES]", "OUTD: [  JAMES] James Oval"),
    ("PHSL: [  G01]", "PHSL: [  G01] WET Lab"),
    ("PHSL: [  G11]", "PHSL: [  G11] WET Lab"),
    ("PHSL: [ 201]", "PHSL: [ 201] Seminar Room"),
    ("PHYS: [ 139A]", "PHYS: [ 139A] DRY Lab"),
    ("PHYS: [ 139B]", "PHYS: [ 139B] DRY Lab"),
    ("PKAV: [  G09]", "PKAV: [  G09] Clinical Teaching Room"),
    ("PKAV: [  G11]", "PKAV: [  G11] DRY ORTHO Lab Research"),
    ("PKAV: [  G12]", "PKAV: [  G12]  DRY GAIT Lab Research"),
    ("PKAV: [  G14A]", "PKAV: [  G14A] WET Lab Research"),
    ("PKAVGP: [  G02]", "PKAVGP: [  G02] Special Teaching"),
    (
        "PSYC: [ 145A]",
        "PSYC: [ 145A] Psychology Computer Lab (North)",
    ),
    ("PSYC: [ 220]", "PSYC: [ 220] Meeting Room (0)"),
    ("QE2L: [  G01]", "QE2L: [  G01] Microbiology Lab"),
    ("QE2L: [  G02]", "QE2L: [  G02] Microbiology Lab"),
    ("QE2L: [  G03]", "QE2L: [  G03] Microbiology Lab"),
    ("QE2L: [  G05]", "QE2L: [  G05] Microbiology Lab"),
    ("QE2L: [  G15]", "QE2L: [  G15] Microbiology Lab"),
    ("QE2L: [  G17]", "QE2L: [  G17] Microbiology Lab"),
    ("QE2L: [  G22]", "QE2L: [  G22] Microbiology Lab"),
    (
        "QE2M: [  B05]",
        "QE2M: [  B05] FA Hadley Museum Basement Lab",
    ),
    ("QE2M: [  G05 ]", "QE2M: [  G05 ]  WET Lab Research"),
    ("QE2M: [  G08]", "QE2M: [  G08] WET Lab Teaching"),
    ("QE2M: [  G14]", "QE2M: [  G14] Pathology Conference Room"),
    (
        "QE2M: [  G15]",
        "QE2M: [  G15] E-learning Computer Suite 3 - Red",
    ),
    (
        "QE2M: [  G17]",
        "QE2M: [  G17] E-learning Computer Suite 2 - Green",
    ),
    (
        "QE2M: [  G19]",
        "QE2M: [  G19] E-learning Computer Suite 1 - Blue",
    ),
    (
        "QE2M: [  G28]",
        "QE2M: [  G28] WET Lab Teaching Pharmacology",
    ),
    ("QE2MDLIB: [  G02]", "QE2MDLIB: [  G02] e-learning suite"),
    ("QE2N: [  G01]", "QE2N: [  G01] Banksia Seminar Room"),
    ("QE2N: [  G02]", "QE2N: [  G02] Teaching Room 2"),
    ("QE2N: [  G03]", "QE2N: [  G03] Teaching Room 3"),
    ("QE2N: [  G04]", "QE2N: [  G04] Kimberley Seminar Room"),
    ("QE2N: [  G05]", "QE2N: [  G05] Pilbara Seminar Room"),
    ("QE2N: [  G06]", "QE2N: [  G06] Goldfields Seminar Room"),
    ("QE2N: [  G07]", "QE2N: [  G07] Wheatbelt Seminar Room"),
    ("QE2N: [  G08]", "QE2N: [  G08] Teaching Room 8"),
    ("QE2N: [  G09]", "QE2N: [  G09] Teaching Room 9"),
    ("QE2N: [  G10]", "QE2N: [  G10] Seminar Room 10"),
    ("QE2N: [ 101]", "QE2N: [ 101] Problem Based Learning"),
    ("QE2N: [ 112]", "QE2N: [ 112] Seminar room"),
    ("QE2N: [ 113]", "QE2N: [ 113] Multipurpose Teaching Space"),
    ("QE2N: [ 114]", "QE2N: [ 114] Multipurpose Teaching Space"),
    ("QE2P: [  G08]", "QE2P: [  G08] Clinical Consult Room"),
    ("QE2P: [  G09]", "QE2P: [  G09] Clinical Consult Room"),
    ("QE2P: [  G11]", "QE2P: [  G11] Clinical Consult Room"),
    ("RBST: [  G01]", "RBST: [  G01] 2nd Year Lab"),
    ("RBST: [  G03]", "RBST: [  G03] CET Resource Room"),
    ("REIDLIB: [  G05]", "REIDLIB: [  G05] The Circle"),
    (
        "REIDLIB: [  G09]",
        "REIDLIB: [  G09] Hemsley Learning Suite",
    ),
    ("SAND: [  G04]", "SAND: [  G04] Seminar Room"),
    ("SAND: [ 101A]", "SAND: [ 101A] Seminar Room"),
    ("SSCI: [ 128]", "SSCI: [ 128] Seminar Room"),
    ("SSCI: [ 148]", "SSCI: [ 148] Tutorial Room"),
    ("SSCI: [ 150]", "SSCI: [ 150] Seminar Room"),
    ("SSCI: [ 2201]", "SSCI: [ 2201] Seminar Room"),
    ("SSCI: [ 229]", "SSCI: [ 229] Seminar Room"),
    ("SSCI: [ 233]", "SSCI: [ 233] Seminar Room"),
    ("SSEH: [  G03]", "SSEH: [  G03] Seminar Room"),
    ("SSEH: [  G05]", "SSEH: [  G05] Research Lab 1"),
    (
        "SSEH: [  G100]",
        "SSEH: [  G100] Health & Rehabilitation Clinic",
    ),
    ("SSEH: [  G19]", "SSEH: [  G19] Biomechanics Lab"),
    ("SSEH: [  G40]", "SSEH: [  G40] Weights Rm - Exercise Lab 2"),
    ("SSEH: [ 101]", "SSEH: [ 101] Gym - Exercise Lab 1"),
    ("UNICLUB: [ 137]", "UNICLUB: [ 137] Auditorium"),
    ("UNIHALL: [ JG26]", "UNIHALL: [ JG26] Boardroom"),
    ("UNIHALL: [ JG27]", "UNIHALL: [ JG27] Innovation Space"),
    ("UWA FieldWork 01 (200)", "UWA FieldWork 01 (200)"),
    ("UWA FieldWork 02 (200)", "UWA FieldWork 02 (200)"),
    ("UWA FieldWork 03 (200)", "UWA FieldWork 03 (200)"),
    ("UWA FieldWork 04 (200)", "UWA FieldWork 04 (200)"),
    ("UWA FieldWork 05 (200)", "UWA FieldWork 05 (200)"),
    ("UWA FieldWork 07 (200)", "UWA FieldWork 07 (200)"),
    ("UWA FieldWork 08 (200)", "UWA FieldWork 08 (200)"),
    ("UWA FieldWork 09 (200)", "UWA FieldWork 09 (200)"),
    ("UWA FieldWork 10 (200)", "UWA FieldWork 10 (200)"),
    ("UWA Online Interactive 00", "UWA Online Interactive 00"),
    ("UWA Online Interactive 01", "UWA Online Interactive 01"),
    ("UWA Online Interactive 02", "UWA Online Interactive 02"),
    ("UWA Online Interactive 03", "UWA Online Interactive 03"),
    ("UWA Online Interactive 09", "UWA Online Interactive 09"),
    ("UWA Online Interactive 10", "UWA Online Interactive 10"),
    ("UWA Online Interactive 11", "UWA Online Interactive 11"),
    ("UWA Online Interactive 12", "UWA Online Interactive 12"),
    ("UWA Online Interactive 13", "UWA Online Interactive 13"),
    ("UWA Online Interactive 14", "UWA Online Interactive 14"),
    ("UWA Online Interactive 15", "UWA Online Interactive 15"),
    ("UWA Online Interactive 16", "UWA Online Interactive 16"),
    ("UWA Online Interactive 19", "UWA Online Interactive 19"),
    ("UWA Online Interactive 20", "UWA Online Interactive 20"),
    ("WSC: [  101]", "WSC: [  101] Water Sports Complex Room 1"),
    ("WSC: [  103]", "WSC: [  103] Water Sports Complex Room 3"),
    (
        "WSC: [  109A]",
        "WSC: [  109A] Water Sports Complex Room 9A",
    ),
    (
        "WSC: [  109B]",
        "WSC: [  109B] Water Sports Complex Room 9B",
    ),
    ("WSC: [  G02]", "WSC: [  G02] Water Sports Complex"),
    ("ZOOL: [  G06]", "ZOOL: [  G06] Meeting Room"),
    ("ZOOL: [  G30]", "ZOOL: [  G30] WET Lab Research"),
    ("ZOOL: [  G31]", "ZOOL: [  G31] 2nd Year Lab"),
];

const MAX_RESULTS: usize = 15;

/// Strip the venue code prefix from the display name.
/// "106.G57 ARTS.  Alexander Lecture Theatre" -> "ARTS - Alexander Lecture Theatre"
/// "AGRI: [  G017] Central Undergrad Computer Lab" -> "AGRI - Central Undergrad Computer Lab"
/// "UWA FieldWork 01 (200)" -> "UWA FieldWork 01 (200)"
fn display_name(full: &str) -> String {
    if let Some(dot_pos) = full.find('.') {
        // Numeric prefix format: "106.G57 ARTS.  Alexander Lecture Theatre"
        // Find the building name after the room number
        let after_code = &full[dot_pos + 1..];
        // Skip the room part (e.g. "G57 ") to find building name
        if let Some(space_pos) = after_code.find(' ') {
            let rest = after_code[space_pos..].trim_start();
            // rest is like "ARTS.  Alexander Lecture Theatre" or "ARTS. Computer Lab"
            if let Some(dot2) = rest.find('.') {
                let building = &rest[..dot2];
                let room_name = rest[dot2 + 1..].trim();
                if room_name.is_empty() {
                    return building.to_string();
                }
                return format!("{building} - {room_name}");
            }
            // No second dot (e.g. "BAYLISS 3rd Yr Biochemistry Laboratory")
            return rest.to_string();
        }
        return full.to_string();
    }

    if let Some(colon_pos) = full.find(':') {
        // "AGRI: [  G017] Central Undergrad Computer Lab"
        let building = full[..colon_pos].trim();
        if let Some(bracket_end) = full.find(']') {
            let room_name = full[bracket_end + 1..].trim();
            if room_name.is_empty() {
                return building.to_string();
            }
            return format!("{building} - {room_name}");
        }
        return building.to_string();
    }

    full.to_string()
}

fn clear_screen() {
    print!("\x1b[2J\x1b[H");
}

fn main() {
    let mut query = String::new();

    // Switch to raw-ish mode using stty
    let _ = std::process::Command::new("stty")
        .arg("-echo")
        .arg("-icanon")
        .arg("min")
        .arg("1")
        .spawn()
        .and_then(|mut c| c.wait());

    let result = run_loop(&mut query);

    // Restore terminal
    let _ = std::process::Command::new("stty")
        .arg("echo")
        .arg("icanon")
        .spawn()
        .and_then(|mut c| c.wait());

    print!("\x1b[?25h");
    clear_screen();

    if let Some((code, _name)) = result {
        let copied = copy_to_clipboard(&code);
        if copied {
            println!("\x1b[1;33m{code}\x1b[0m copied to clipboard");
        } else {
            println!("\x1b[1;33m{code}\x1b[0m");
            println!("\x1b[90m(could not copy to clipboard automatically)\x1b[0m");
        }
    }
}

fn copy_to_clipboard(text: &str) -> bool {
    use std::process::{Command, Stdio};

    // macOS: pbcopy
    if let Ok(mut child) = Command::new("pbcopy")
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
    {
        if let Some(mut stdin) = child.stdin.take() {
            use std::io::Write;
            let _ = stdin.write_all(text.as_bytes());
        }
        return child.wait().map(|s| s.success()).unwrap_or(false);
    }

    // Linux (X11): xclip
    if let Ok(mut child) = Command::new("xclip")
        .args(["-selection", "clipboard"])
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
    {
        if let Some(mut stdin) = child.stdin.take() {
            use std::io::Write;
            let _ = stdin.write_all(text.as_bytes());
        }
        return child.wait().map(|s| s.success()).unwrap_or(false);
    }

    // Linux (X11) alt: xsel
    if let Ok(mut child) = Command::new("xsel")
        .args(["--clipboard", "--input"])
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
    {
        if let Some(mut stdin) = child.stdin.take() {
            use std::io::Write;
            let _ = stdin.write_all(text.as_bytes());
        }
        return child.wait().map(|s| s.success()).unwrap_or(false);
    }

    // Linux (Wayland): wl-copy
    if let Ok(mut child) = Command::new("wl-copy")
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
    {
        if let Some(mut stdin) = child.stdin.take() {
            use std::io::Write;
            let _ = stdin.write_all(text.as_bytes());
        }
        return child.wait().map(|s| s.success()).unwrap_or(false);
    }

    // Fallback: OSC 52 (works in iTerm2, kitty, alacritty, WezTerm, Windows Terminal)
    let b64 = base64_encode(text.as_bytes());
    print!("\x1b]52;c;{b64}\x07");
    let _ = stdout().flush();
    true
}

fn base64_encode(input: &[u8]) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::new();
    for chunk in input.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = if chunk.len() > 1 { chunk[1] as u32 } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };
        let triple = (b0 << 16) | (b1 << 8) | b2;
        out.push(CHARS[((triple >> 18) & 0x3F) as usize] as char);
        out.push(CHARS[((triple >> 12) & 0x3F) as usize] as char);
        if chunk.len() > 1 {
            out.push(CHARS[((triple >> 6) & 0x3F) as usize] as char);
        } else {
            out.push('=');
        }
        if chunk.len() > 2 {
            out.push(CHARS[(triple & 0x3F) as usize] as char);
        } else {
            out.push('=');
        }
    }
    out
}

fn run_loop(query: &mut String) -> Option<(String, String)> {
    let mut selected: usize = 0;
    let mut prev_query = String::new();

    loop {
        // Get matches
        let matches: Vec<(i32, &str, &str)> = if query.is_empty() {
            VENUES
                .iter()
                .take(MAX_RESULTS)
                .map(|(c, n)| (0, *c, *n))
                .collect()
        } else {
            let mut scored: Vec<_> = VENUES
                .iter()
                .filter_map(|(code, name)| fuzzy_score(query, name).map(|s| (s, *code, *name)))
                .collect();
            scored.sort_by_key(|(s, _, _)| *s);
            scored.into_iter().take(MAX_RESULTS).collect()
        };

        // Reset selection if query changed
        if *query != prev_query {
            selected = 0;
            prev_query = query.clone();
        }

        // Clamp selection
        if !matches.is_empty() && selected >= matches.len() {
            selected = matches.len() - 1;
        }

        // Render
        clear_screen();
        println!("\x1b[1;36mUWA Venue Finder\x1b[0m");
        println!(
            "\x1b[90mType to search. \x1b[0m\x1b[90m↑↓ navigate. Enter = select. Ctrl+C = quit.\x1b[0m\n"
        );
        print!("\x1b[1m> \x1b[0m{}", query);
        let _ = stdout().flush();

        println!("\n");
        if matches.is_empty() {
            println!("  \x1b[90mNo matches found.\x1b[0m");
        } else {
            for (i, (_score, code, name)) in matches.iter().enumerate() {
                let label = display_name(name);
                if i == selected {
                    println!("  \x1b[7m \x1b[33m{code:<20}\x1b[0m\x1b[7m {label} \x1b[0m");
                } else {
                    println!("  \x1b[33m{code:<20}\x1b[0m \x1b[90m{label}\x1b[0m");
                }
            }
        }

        let _ = stdout().flush();

        // Read one byte
        let mut buf = [0u8; 1];
        if stdin().read_exact(&mut buf).is_err() {
            return None;
        }

        match buf[0] {
            3 => return None, // Ctrl+C
            10 | 13 => {
                // Enter — select highlighted match
                if !matches.is_empty() {
                    let (_, code, name) = matches[selected];
                    return Some((code.to_string(), name.to_string()));
                }
            }
            127 | 8 => {
                // Backspace
                query.pop();
            }
            27 => {
                // Escape sequence — read the next two bytes
                let mut seq = [0u8; 2];
                if stdin().read_exact(&mut seq).is_ok() {
                    if seq[0] == b'[' {
                        match seq[1] {
                            b'A' => {
                                // Up arrow
                                if selected > 0 {
                                    selected -= 1;
                                }
                            }
                            b'B' => {
                                // Down arrow
                                if !matches.is_empty() && selected < matches.len() - 1 {
                                    selected += 1;
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
            b if b >= 32 => {
                query.push(b as char);
            }
            _ => {}
        }
    }
}

use std::io::Read;
