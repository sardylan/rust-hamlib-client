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
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
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

impl From<&str> for Mode {
    fn from(value: &str) -> Self {
        Mode::from_str(value).unwrap()
    }
}

impl From<&&str> for Mode {
    fn from(value: &&str) -> Self {
        Mode::from_str(value).unwrap()
    }
}

impl FromStr for Mode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AM" => Ok(Mode::AM),
            "ARDOP" => Ok(Mode::ARDOP),
            "ATV" => Ok(Mode::ATV),
            "CHIP" => Ok(Mode::CHIP),
            "CLO" => Ok(Mode::CLO),
            "CONTESTI" => Ok(Mode::CONTESTI),
            "CW" => Ok(Mode::CW),
            "DIGITALVOICE" => Ok(Mode::DIGITALVOICE),
            "DOMINO" => Ok(Mode::DOMINO),
            "DYNAMIC" => Ok(Mode::DYNAMIC),
            "FAX" => Ok(Mode::FAX),
            "FM" => Ok(Mode::FM),
            "FSK441" => Ok(Mode::FSK441),
            "FT8" => Ok(Mode::FT8),
            "HELL" => Ok(Mode::HELL),
            "ISCAT" => Ok(Mode::ISCAT),
            "JT4" => Ok(Mode::JT4),
            "JT6M" => Ok(Mode::JT6M),
            "JT9" => Ok(Mode::JT9),
            "JT44" => Ok(Mode::JT44),
            "JT65" => Ok(Mode::JT65),
            "MFSK" => Ok(Mode::MFSK),
            "MSK144" => Ok(Mode::MSK144),
            "MT63" => Ok(Mode::MT63),
            "OLIVIA" => Ok(Mode::OLIVIA),
            "OPERA" => Ok(Mode::OPERA),
            "PAC" => Ok(Mode::PAC),
            "PAX" => Ok(Mode::PAX),
            "PKT" => Ok(Mode::PKT),
            "PSK" => Ok(Mode::PSK),
            "PSK2K" => Ok(Mode::PSK2K),
            "Q15" => Ok(Mode::Q15),
            "QRA64" => Ok(Mode::QRA64),
            "ROS" => Ok(Mode::ROS),
            "RTTY" => Ok(Mode::RTTY),
            "RTTYM" => Ok(Mode::RTTYM),
            "SSB" => Ok(Mode::SSB),
            "SSTV" => Ok(Mode::SSTV),
            "T10" => Ok(Mode::T10),
            "THOR" => Ok(Mode::THOR),
            "THRB" => Ok(Mode::THRB),
            "TOR" => Ok(Mode::TOR),
            "V4" => Ok(Mode::V4),
            "VOI" => Ok(Mode::VOI),
            "WINMOR" => Ok(Mode::WINMOR),
            "WSPR" => Ok(Mode::WSPR),
            _ => Ok(Mode::None),
        }
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
            Self::AS => {
                write!(f, "Aircraft Scatter")
            }
            Self::AUE => {
                write!(f, "Aurora-E")
            }
            Self::AUR => {
                write!(f, "Aurora")
            }
            Self::BS => {
                write!(f, "Back scatter")
            }
            Self::ECH => {
                write!(f, "EchoLink")
            }
            Self::EME => {
                write!(f, "Earth-Moon-Earth")
            }
            Self::ES => {
                write!(f, "Sporadic E")
            }
            Self::F2 => {
                write!(f, "F2 Reflection")
            }
            Self::FAI => {
                write!(f, "Field Aligned Irregularities")
            }
            Self::GWAVE => {
                write!(f, "Ground Wave")
            }
            Self::INTERNET => {
                write!(f, "Internet-assisted")
            }
            Self::ION => {
                write!(f, "Ionoscatter")
            }
            Self::IRL => {
                write!(f, "IRLP")
            }
            Self::LOS => {
                write!(
                    f,
                    "Line of Sight (includes transmission through obstacles such as walls)"
                )
            }
            Self::MS => {
                write!(f, "Meteor scatter")
            }
            Self::RPT => {
                write!(f, "Terrestrial or atmospheric repeater or transponder")
            }
            Self::RS => {
                write!(f, "Rain scatter")
            }
            Self::SAT => {
                write!(f, "Satellite")
            }
            Self::TEP => {
                write!(f, "Trans-equatorial")
            }
            Self::TR => {
                write!(f, "Tropospheric ducting")
            }
            Self::None => {
                write!(f, "")
            }
        }
    }
}
