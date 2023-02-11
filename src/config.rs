// config.rs: Configuration file format for easy serializing and deserializing
//
// Copyright (C) 2023 Andrew Rioux
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fmt::Display,
};

use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum ConversionError {
    NoIpFormatProvided,
    OneTeamConfigurationWithMultipleTeams,
    XInManualIP(String),
    NoXInTemplateIP(String),
    MultNotBigEnough(u8, u8),
    OffsetNotSpecified(String),
    MissingOffset(String),
    DuplicateOffsets(Vec<String>),
    DuplicateIPs(String, String, String),
    EmptyUsernameOrPassword(String, String),
    DuplicateBlueTeamIDs(u8, Vec<String>),
    ZeroBlueTeamID(String),
    TeamNeedsUser(String),
    TeamHasEmptyName,
    DuplicateUserNameForTeams(String, Vec<String>),
}

impl Error for ConversionError {}

impl Display for ConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoIpFormatProvided => {
                write!(
                    f,
                    "no ip address was provided for a machine with manual override"
                )
            }
            Self::OneTeamConfigurationWithMultipleTeams => {
                write!(
                    f,
                    "multiple teams specified with a one team ip address configuration"
                )
            }
            Self::XInManualIP(machine) => {
                write!(
                    f,
                    "an ip address template was provided when a full ip address was expected (machine: {})",
                    machine
                )
            }
            Self::NoXInTemplateIP(machine) => {
                write!(
                    f,
                    "a full ip address was provided when an ip address template was expected (machine: {})",
                    machine
                )
            }
            Self::MultNotBigEnough(mcount, mult) => {
                write!(
                    f,
                    "the multiplier specified was not big enough to account for all the machines on the network (multiplier {mult}, machine count {mcount})"
                )
            }
            Self::OffsetNotSpecified(machine) => {
                write!(
                    f,
                    "a machine does not have an ip address offset specified (machine {})",
                    machine
                )
            }
            Self::DuplicateOffsets(machines) => {
                write!(
                    f,
                    "multiple machines share the same offsets ({})",
                    machines.join(", ")
                )
            }
            Self::DuplicateIPs(ip, m1, m2) => {
                write!(
                    f,
                    "duplicate ip address {} specified for machines {} and {}",
                    ip, m1, m2
                )
            }
            Self::MissingOffset(m) => {
                write!(f, "machine {m} is missing an offset")
            }
            Self::EmptyUsernameOrPassword(w, u) => {
                write!(f, "empty username or password at {w} (username: {u})")
            }
            Self::DuplicateBlueTeamIDs(id, names) => {
                write!(
                    f,
                    "duplicate blue team member IDs for teams {} ({id})",
                    names.join(", ")
                )
            }
            Self::ZeroBlueTeamID(name) => {
                write!(f, "blue team {name} has an ID of 0")
            }
            Self::TeamNeedsUser(team) => {
                write!(f, "team {team} is missing at least one user account")
            }
            Self::TeamHasEmptyName => {
                write!(f, "one of the teams has no name")
            }
            Self::DuplicateUserNameForTeams(name, teams) => {
                write!(
                    f,
                    "the username {name} is repeated across teams {}",
                    teams.join(", ")
                )
            }
        }
    }
}

#[derive(Deserialize, Serialize, Eq, PartialEq, Debug, Clone)]
pub struct User {
    pub username: String,
    pub password: String,
}

impl User {
    pub fn validate(self, where_: String) -> Result<User, ConversionError> {
        if self.username.is_empty() || self.password.is_empty() {
            return Err(ConversionError::EmptyUsernameOrPassword(
                where_,
                self.username.clone(),
            ));
        }

        Ok(self)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum TeamColor {
    White,
    Red,
    Blue,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Configuration {
    pub editor_info: ConfigurationEditor,
    pub teams: Vec<TeamConfig>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "color")]
pub enum TeamConfig {
    Red {
        name: String,
        users: Vec<User>,
    },
    White {
        name: String,
        users: Vec<User>,
    },
    Blue {
        name: String,
        users: Vec<User>,
        services: Vec<ServiceConfig>,
    },
}

#[derive(Deserialize, Serialize, Eq, PartialEq, Debug, Clone)]
pub struct EnvironmentProperties {
    pub name: String,
    pub value: String,
}

#[derive(Deserialize, Serialize, Eq, PartialEq, Debug, Clone)]
pub struct Environment {
    pub matching_content: String,
    pub properties: Vec<EnvironmentProperties>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ServiceConfig {
    pub name: String,
    pub check_name: String,
    pub host: String,
    pub port: u16,
    pub points: u16,
    pub accounts: Option<Vec<User>>,
    pub environments: Vec<Environment>,
}

#[derive(Deserialize, Serialize, Eq, PartialEq, Debug, Clone)]
pub struct ServiceEditor {
    pub name: String,
    pub port: u16,
    pub points: u16,
    pub definition: ServiceDefinition,
    pub accounts: Option<Vec<User>>,
}

trait ToEnvironment {
    fn to_environment(&self) -> Vec<Environment>;
}

#[derive(Deserialize, Serialize, Eq, PartialEq, Debug, Clone)]
pub struct DNSCheckInfo {
    match_content: String,
    qtype: String,
    domain: String,
}

#[derive(Deserialize, Serialize, Eq, PartialEq, Debug, Clone)]
pub enum ServiceDefinition {
    DNS(DNSCheckInfo),
    Docker,
    Elasticsearch,
    FTP,
    GeneralHTTP,
    GeneralHTTPS,
    ICMP,
    IMAP,
    IMAPS,
    MSSQL,
    MySQL,
    NFS,
    POP3,
    POP3S,
    PostgreSQL,
    RDP,
    SMB,
    SMTP,
    SMTPS,
    SSH,
    VNC,
    WinRM,
    Wordpress,
}

impl ServiceDefinition {
    pub fn environments(&self) -> Vec<Environment> {
        match self {
            _ => vec![],
        }
    }

    pub fn check_name(&self) -> &'static str {
        match self {
            ServiceDefinition::DNS { .. } => "DNSCheck",
            ServiceDefinition::Docker => "DockerCheck",
            ServiceDefinition::Elasticsearch => "ElasticsearchCheck",
            ServiceDefinition::FTP => "FTPCheck",
            ServiceDefinition::GeneralHTTP => "GeneralHTTPCheck",
            ServiceDefinition::GeneralHTTPS => "GeneralHTTPSCheck",
            ServiceDefinition::ICMP => "ICMPCheck",
            ServiceDefinition::IMAP => "IMAPCheck",
            ServiceDefinition::IMAPS => "IMAPSCheck",
            ServiceDefinition::MSSQL => "MSSQLCheck",
            ServiceDefinition::MySQL => "MySQLCheck",
            ServiceDefinition::NFS => "NFSCheck",
            ServiceDefinition::POP3 => "POP3Check",
            ServiceDefinition::POP3S => "POP3SCheck",
            ServiceDefinition::PostgreSQL => "PostgreSQLCheck",
            ServiceDefinition::RDP => "RDPCheck",
            ServiceDefinition::SMB => "SMBCheck",
            ServiceDefinition::SMTP => "SMTPCheck",
            ServiceDefinition::SMTPS => "SMTPSCheck",
            ServiceDefinition::SSH => "SSHCheck",
            ServiceDefinition::VNC => "VNCCheck",
            ServiceDefinition::WinRM => "WinRMCheck",
            ServiceDefinition::Wordpress => "WordpressCheck",
        }
    }
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
pub enum IpGeneratorScheme {
    OneTeam,
    ReplaceXWithId,
    ReplaceXWithIdTimesMultiplierPlusOffset(u8),
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
pub struct RedWhiteTeamEditor {
    pub name: String,
    pub users: Vec<User>,
    pub white_team: bool,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
pub struct BlueTeamEditor {
    pub id: u8,
    pub name: String,
    pub users: Vec<User>,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
pub struct MachineEditor {
    pub name: String,
    pub services: Vec<ServiceEditor>,
    pub ip_template: String,
    pub ip_offset: Option<u8>,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
pub struct ConfigurationEditor {
    pub red_white_teams: Vec<RedWhiteTeamEditor>,
    pub blue_teams: Vec<BlueTeamEditor>,
    pub machines: Vec<MachineEditor>,
    pub ip_generator: IpGeneratorScheme,
}

type ConversionState = HashMap<String, String>;

fn convert_id_to_ip(
    used_ips: &mut ConversionState,
    machine_name: &str,
    ip_template: &str,
    ip_offset: Option<u8>,
    generator: &IpGeneratorScheme,
    id: u8,
) -> Result<String, ConversionError> {
    use IpGeneratorScheme::*;

    match generator {
        OneTeam => {
            if ip_template.chars().any(|c| c == 'x' || c == 'X') {
                Err(ConversionError::XInManualIP(machine_name.to_owned()))
            } else if let Some(other_machine) = used_ips.get(ip_template) {
                Err(ConversionError::DuplicateIPs(
                    ip_template.to_owned(),
                    machine_name.to_owned(),
                    other_machine.to_owned(),
                ))
            } else {
                used_ips.insert(ip_template.to_owned(), machine_name.to_owned());

                Ok(ip_template.to_string())
            }
        }
        ReplaceXWithId => {
            if !ip_template.chars().any(|c| c == 'x' || c == 'X') {
                Err(ConversionError::NoXInTemplateIP(machine_name.to_owned()))
            } else {
                let ip = ip_template
                    .replace("X", &id.to_string())
                    .replace("x", &id.to_string());

                if let Some(other_machine) = used_ips.get(&ip) {
                    Err(ConversionError::DuplicateIPs(
                        ip,
                        machine_name.to_owned(),
                        other_machine.to_owned(),
                    ))
                } else {
                    used_ips.insert(ip.to_owned(), machine_name.to_owned());
                    Ok(ip)
                }
            }
        }
        ReplaceXWithIdTimesMultiplierPlusOffset(mult) => {
            let Some(ip_offset) = ip_offset else {
                return Err(ConversionError::OffsetNotSpecified(machine_name.to_owned()));
            };
            let ip = mult * id + ip_offset;
            if ip_template.chars().any(|c| c == 'x' || c == 'X') {
                Ok(ip_template
                    .replace("X", &ip.to_string())
                    .replace("x", &ip.to_string()))
            } else {
                Err(ConversionError::NoXInTemplateIP(machine_name.to_owned()))
            }
        }
    }
}

pub fn convert_editor_to_final(
    config: &ConfigurationEditor,
) -> Result<Configuration, ConversionError> {
    let config = config.clone();

    let red_white = config
        .red_white_teams
        .iter()
        .map(|team| -> Result<_, ConversionError> {
            Ok(if team.white_team {
                TeamConfig::White {
                    name: if team.name.is_empty() {
                        Err(ConversionError::TeamHasEmptyName)
                    } else {
                        Ok(team.name.clone())
                    }?,
                    users: if team.users.is_empty() {
                        Err(ConversionError::TeamNeedsUser(team.name.clone()))
                    } else {
                        Ok(team.users.clone())
                    }?,
                }
            } else {
                TeamConfig::Red {
                    name: if team.name.is_empty() {
                        Err(ConversionError::TeamHasEmptyName)
                    } else {
                        Ok(team.name.clone())
                    }?,
                    users: if team.users.is_empty() {
                        Err(ConversionError::TeamNeedsUser(team.name.clone()))
                    } else {
                        Ok(team.users.clone())
                    }?,
                }
            })
        })
        .collect::<Result<Vec<_>, ConversionError>>()?;

    if let IpGeneratorScheme::OneTeam = config.ip_generator {
        if config.blue_teams.len() > 1 {
            return Err(ConversionError::OneTeamConfigurationWithMultipleTeams);
        }
    }

    if let IpGeneratorScheme::ReplaceXWithIdTimesMultiplierPlusOffset(mult) = config.ip_generator {
        let mcount = <usize as TryInto<u8>>::try_into(config.machines.len()).unwrap();
        if mult < mcount {
            return Err(ConversionError::MultNotBigEnough(mcount, mult));
        }

        let offsets = match config
            .machines
            .iter()
            .map(|m| {
                m.ip_offset
                    .map(|off| (off, m.name.to_owned()))
                    .ok_or_else(|| m.name.to_owned())
            })
            .collect::<Result<Vec<_>, _>>()
        {
            Ok(offsets) => offsets,
            Err(m) => return Err(ConversionError::MissingOffset(m)),
        };

        let mut offset_unique_detection = HashMap::<u8, Vec<String>>::new();

        for (off, mname) in offsets {
            match offset_unique_detection.get_mut(&off) {
                Some(offset) => {
                    offset.push(mname);
                }
                None => {
                    offset_unique_detection.insert(off, vec![mname]);
                }
            };
        }

        for machine_offsets in offset_unique_detection.values() {
            if machine_offsets.len() > 1 {
                return Err(ConversionError::DuplicateOffsets(machine_offsets.to_vec()));
            }
        }
    }

    let mut conversion_state = ConversionState::new();

    fn services_generator(
        conversion_state: &mut ConversionState,
        config: &ConfigurationEditor,
        team: &BlueTeamEditor,
    ) -> Result<Vec<ServiceConfig>, ConversionError> {
        Ok(config
            .machines
            .iter()
            .map(|machine| {
                machine
                    .services
                    .iter()
                    .map(|service| {
                        Ok(ServiceConfig {
                            name: format!(
                                "{}-{}-{}",
                                machine.name,
                                service.definition.check_name(),
                                service.name
                            ),
                            check_name: service.definition.check_name().to_string(),
                            host: convert_id_to_ip(
                                conversion_state,
                                &machine.name,
                                &machine.ip_template,
                                machine.ip_offset,
                                &config.ip_generator,
                                team.id,
                            )?,
                            port: service.port,
                            points: service.points,
                            accounts: service
                                .accounts
                                .clone()
                                .map(|users| {
                                    users
                                        .into_iter()
                                        .map(|user| {
                                            user.validate(format!(
                                                "service {}-{}",
                                                machine.name, service.name
                                            ))
                                        })
                                        .collect::<Result<Vec<_>, ConversionError>>()
                                })
                                .transpose()?,
                            environments: service.definition.environments(),
                        })
                    })
                    .collect::<Result<Vec<_>, ConversionError>>()
            })
            .collect::<Result<Vec<_>, ConversionError>>()?
            .concat())
    }

    {
        let mut blue_ids_map: HashMap<u8, Vec<&str>> = HashMap::new();

        for team in &config.blue_teams {
            if team.id == 0 {
                return Err(ConversionError::ZeroBlueTeamID(team.name.clone()));
            }

            let name_list_option = blue_ids_map.get_mut(&team.id);

            if let Some(name_list) = name_list_option {
                name_list.push(&team.name);
            } else {
                blue_ids_map.insert(team.id, vec![&team.name]);
            }
        }

        for (id, names) in blue_ids_map {
            if names.len() > 1 {
                return Err(ConversionError::DuplicateBlueTeamIDs(
                    id,
                    names.iter().map(ToString::to_string).collect(),
                ));
            }
        }
    }

    let blue = config
        .blue_teams
        .iter()
        .map(|team| -> Result<TeamConfig, ConversionError> {
            Ok(TeamConfig::Blue {
                name: if team.name.is_empty() {
                    Err(ConversionError::TeamHasEmptyName)
                } else {
                    Ok(team.name.clone())
                }?,
                users: if team.users.is_empty() {
                    Err(ConversionError::TeamNeedsUser(team.name.clone()))
                } else {
                    Ok(team.users.clone())
                }?,
                services: services_generator(&mut conversion_state, &config, &team)?,
            })
        })
        .collect::<Result<Vec<_>, ConversionError>>()?;

    {
        let mut names: HashMap<&str, Vec<&str>> = HashMap::new();

        macro_rules! add_names_branch {
            ($name:expr, $users:expr) => {{
                if let Some(names_list) = names.get_mut(&$name as &str) {
                    names_list.extend($users.iter().map(|user| &*user.username));
                } else {
                    names.insert(
                        &$name as &str,
                        $users
                            .iter()
                            .map(|user| &*user.username)
                            .collect::<Vec<_>>(),
                    );
                }
            }};
        }

        macro_rules! add_names {
            ($teams:expr) => {
                for team in &$teams {
                    match team {
                        TeamConfig::Red {
                            ref name, users, ..
                        } => add_names_branch!(name, users),
                        TeamConfig::White {
                            ref name, users, ..
                        } => add_names_branch!(name, users),
                        TeamConfig::Blue {
                            ref name, users, ..
                        } => add_names_branch!(name, users),
                    };
                }
            };
        }

        add_names!(blue);
        add_names!(red_white);

        for (team_name, user_names) in names {
            if user_names.len() > 1 {
                return Err(ConversionError::DuplicateUserNameForTeams(
                    team_name.to_string(),
                    user_names.iter().map(ToString::to_string).collect(),
                ));
            }
        }
    }

    Ok(Configuration {
        teams: [red_white, blue].concat(),
        editor_info: config,
    })
}
