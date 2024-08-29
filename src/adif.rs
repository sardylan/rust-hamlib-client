/*
 * Copyright (C) 2024 Luca Cireddu <sardylan@gmail.com>
 *
 * This program is free software: you can redistribute it and/or modify it under
 * the terms of the GNU General Public License as published by the Free Software
 * Foundation, version 3.
 *
 * This program is distributed in the hope that it will be useful, but WITHOUT
 * ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
 * FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along with
 * this program. If not, see <https://www.gnu.org/licenses/>.
 *
 */

use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Mode {
    AM,
    ARDOP,
    ATV,
    CHIP,
    CLO,
    CONTESTI,
    CW,
    DIGITALVOICE,
    DOMINO,
    DYNAMIC,
    FAX,
    FM,
    FSK441,
    FT8,
    HELL,
    ISCAT,
    JT4,
    JT6M,
    JT9,
    JT44,
    JT65,
    MFSK,
    MSK144,
    MT63,
    OLIVIA,
    OPERA,
    PAC,
    PAX,
    PKT,
    PSK,
    PSK2K,
    Q15,
    QRA64,
    ROS,
    RTTY,
    RTTYM,
    SSB,
    SSTV,
    T10,
    THOR,
    THRB,
    TOR,
    V4,
    VOI,
    WINMOR,
    WSPR,
    None,
}

impl Display for Mode {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum PropagationMode {
    AS,
    AUE,
    AUR,
    BS,
    ECH,
    EME,
    ES,
    F2,
    FAI,
    GWAVE,
    INTERNET,
    ION,
    IRL,
    LOS,
    MS,
    RPT,
    RS,
    SAT,
    TEP,
    TR,
    None,
}

impl Display for PropagationMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AS => { write!(f, "Aircraft Scatter") }
            Self::AUE => { write!(f, "Aurora-E") }
            Self::AUR => { write!(f, "Aurora") }
            Self::BS => { write!(f, "Back scatter") }
            Self::ECH => { write!(f, "EchoLink") }
            Self::EME => { write!(f, "Earth-Moon-Earth") }
            Self::ES => { write!(f, "Sporadic E") }
            Self::F2 => { write!(f, "F2 Reflection") }
            Self::FAI => { write!(f, "Field Aligned Irregularities") }
            Self::GWAVE => { write!(f, "Ground Wave") }
            Self::INTERNET => { write!(f, "Internet-assisted") }
            Self::ION => { write!(f, "Ionoscatter") }
            Self::IRL => { write!(f, "IRLP") }
            Self::LOS => { write!(f, "Line of Sight (includes transmission through obstacles such as walls)") }
            Self::MS => { write!(f, "Meteor scatter") }
            Self::RPT => { write!(f, "Terrestrial or atmospheric repeater or transponder") }
            Self::RS => { write!(f, "Rain scatter") }
            Self::SAT => { write!(f, "Satellite") }
            Self::TEP => { write!(f, "Trans-equatorial") }
            Self::TR => { write!(f, "Tropospheric ducting") }
            Self::None => { write!(f, "") }
        }
    }
}
