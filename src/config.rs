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
    TeamNeedsUser(String),
    TeamHasEmptyName,
    MachineHasEmptyName,
    MachineHasEmptyService(String),
    DuplicateUserNameForTeams(String, Vec<String>),
    DuplicateMachineNames(String),
    ServiceNotFullyConfigured(String, String, String),
    DuplicateServiceName(String, String),
}

impl Error for ConversionError {}

impl Display for ConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OneTeamConfigurationWithMultipleTeams => {
                write!(
                    f,
                    "multiple teams specified with a one team ip address configuration"
                )
            }
            Self::XInManualIP(machine) => {
                write!(
                    f,
                    "an ip address template was provided when a full ip address was expected (machine: {machine})"
                )
            }
            Self::NoXInTemplateIP(machine) => {
                write!(
                    f,
                    "a full ip address was provided when an ip address template was expected (machine: {machine})"
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
                    "a machine does not have an ip address offset specified (machine {machine})"
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
                    "duplicate ip address {ip} specified for machines {m1} and {m2}"
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
            Self::MachineHasEmptyName => {
                write!(f, "there can't be any machines with no name")
            }
            Self::MachineHasEmptyService(machine) => {
                write!(f, "machine '{machine}' has a service with no name")
            }
            Self::DuplicateMachineNames(machine) => {
                write!(f, "multiple machines with the same name '{machine}'")
            }
            Self::ServiceNotFullyConfigured(machine, service, err) => {
                write!(
                    f,
                    "the service {service} on machine {machine} was not fully configured: {err}"
                )
            }
            Self::DuplicateServiceName(machine, service) => {
                write!(
                    f,
                    "the machine {machine} has multiple services named {service}"
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
                self.username,
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
pub struct FinalConfiguration {
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
    #[serde(skip_serializing_if = "Option::is_none")]
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

#[derive(Deserialize, Serialize, Eq, PartialEq, Debug, Clone, Default)]
pub struct DnsCheckInfo {
    pub matching_content: String,
    pub qtype: String,
    pub domain: String,
}

#[derive(Deserialize, Serialize, Eq, PartialEq, Debug, Clone, Default)]
pub struct DockerCheckInfo {
    pub matching_content: String,
    pub image: String,
}

#[derive(Deserialize, Serialize, Eq, PartialEq, Debug, Clone, Default)]
pub struct ElasticsearchCheckInfo {
    pub matching_content: String,
    pub index: String,
    pub doc_type: String,
}

#[derive(Deserialize, Serialize, Eq, PartialEq, Debug, Clone, Default)]
pub struct FtpCheckInfo {
    pub matching_content: String,
    pub remotefilepath: String,
    pub filecontents: String,
}

#[derive(Deserialize, Serialize, Eq, PartialEq, Debug, Clone, Default)]
pub struct HttpCheckInfo {
    pub matching_content: String,
    pub useragent: String,
    pub vhost: String,
    pub uri: String,
}

#[derive(Deserialize, Serialize, Eq, PartialEq, Debug, Clone, Default)]
pub struct ImapCheckInfo {
    pub matching_content: String,
    pub domain: String,
}

#[derive(Deserialize, Serialize, Eq, PartialEq, Debug, Clone, Default)]
pub struct LdapCheckInfo {
    pub matching_content: String,
    pub domain: String,
    pub base_dn: String,
}

#[derive(Deserialize, Serialize, Eq, PartialEq, Debug, Clone, Default)]
pub struct SqlCheckInfo {
    pub matching_content: String,
    pub database: String,
    pub command: String,
}

#[derive(Deserialize, Serialize, Eq, PartialEq, Debug, Clone, Default)]
pub struct NfsCheckInfo {
    pub matching_content: String,
    pub remotefilepath: String,
    pub filecontents: String,
}

#[derive(Deserialize, Serialize, Eq, PartialEq, Debug, Clone, Default)]
pub struct PopCheckInfo {
    pub matching_content: String,
    pub domain: String,
}

#[derive(Deserialize, Serialize, Eq, PartialEq, Debug, Clone, Default)]
pub struct SmbCheckInfo {
    pub matching_content: String,
    pub remote_name: String,
    pub share: String,
    pub file: String,
    pub hash: String,
}

#[derive(Deserialize, Serialize, Eq, PartialEq, Debug, Clone, Default)]
pub struct SmtpCheckInfo {
    pub matching_content: String,
    pub touser: String,
    pub subject: String,
    pub body: String,
}

#[derive(Deserialize, Serialize, Eq, PartialEq, Debug, Clone, Default)]
pub struct RemoteCommandCheckInfo {
    pub matching_content: String,
    pub commands: String,
}

#[derive(Deserialize, Serialize, Eq, PartialEq, Debug, Clone)]
#[serde(tag = "type")]
pub enum ServiceDefinition {
    Dns {
        environment: Vec<DnsCheckInfo>,
    },
    Docker {
        environment: Vec<DockerCheckInfo>,
    },
    Elasticsearch {
        environment: Vec<ElasticsearchCheckInfo>,
    },
    Ftp {
        environment: Vec<FtpCheckInfo>,
    },
    Http {
        environment: Vec<HttpCheckInfo>,
    },
    Https {
        environment: Vec<HttpCheckInfo>,
    },
    Icmp {
        environment: Option<String>,
    },
    Imap {
        environment: Vec<ImapCheckInfo>,
    },
    Imaps {
        environment: Vec<ImapCheckInfo>,
    },
    Ldap {
        environment: Vec<LdapCheckInfo>,
    },
    Mssql {
        environment: Vec<SqlCheckInfo>,
    },
    Mysql {
        environment: Vec<SqlCheckInfo>,
    },
    Nfs {
        environment: Vec<NfsCheckInfo>,
    },
    Pop3 {
        environment: Vec<PopCheckInfo>,
    },
    Pop3s {
        environment: Vec<PopCheckInfo>,
    },
    PostgreSql {
        environment: Vec<SqlCheckInfo>,
    },
    Rdp {
        environment: Option<String>,
    },
    Smb {
        environment: Vec<SmbCheckInfo>,
    },
    Smtp {
        environment: Vec<SmtpCheckInfo>,
    },
    Smtps {
        environment: Vec<SmtpCheckInfo>,
    },
    Ssh {
        environment: Vec<RemoteCommandCheckInfo>,
    },
    Vnc {
        environment: Option<String>,
    },
    WinRm {
        environment: Vec<RemoteCommandCheckInfo>,
    },
    Wordpress {
        environment: Vec<HttpCheckInfo>,
    },
}

macro_rules! service_definition_check {
    (($machine_name:expr, $service_name:expr, $properties:expr), (matching_content => ($($mc_check_expr:expr => $mc_error:expr),*), $($field:ident => ($($check:expr => $error:expr),*)),*)) => {{
        $properties
            .iter()
            .map(|iter_item| {
                let errs = [
                    $(if ($mc_check_expr)(&iter_item.matching_content) { vec![$mc_error.to_string()] } else { vec![] }),*,
                    $( /* $field */ $(if ($check)(&iter_item.$field) { vec![$error.to_string()] } else { vec![] }),*),*
                ].concat();
                if errs.is_empty() {
                    Ok(Environment {
                        matching_content: iter_item.matching_content.clone(),
                        properties: vec![
                            $(EnvironmentProperties {
                                name: stringify!($field).to_string(),
                                value: iter_item.$field.clone()
                            }),*
                        ]
                    })
                } else {
                    Err(crate::config::ConversionError::ServiceNotFullyConfigured($machine_name.to_string(), $service_name.to_string(), errs.join(", ")))
                }
            })
            .collect::<Result<Vec<_>, _>>()
    }};
}

impl ServiceDefinition {
    pub fn environments(
        &self,
        mname: &str,
        sname: &str,
    ) -> Result<Vec<Environment>, ConversionError> {
        match self {
            ServiceDefinition::Dns { environment: dns } => service_definition_check! {
                (mname, sname, dns),
                (
                    matching_content => (
                        str::is_empty => "Service match cannot be empty"
                    ),
                    qtype => (
                        str::is_empty => "DNS query type cannot be empty"
                    ),
                    domain => (
                        str::is_empty => "Domain queried cannot be empty"
                    )
                )
            },
            ServiceDefinition::Elasticsearch {
                environment: elasticsearch,
            } => service_definition_check! {
                (mname, sname, elasticsearch),
                (
                    matching_content => (
                        str::is_empty => "Service match cannot be empty"
                    ),
                    index => (
                        str::is_empty => "Index cannot be empty"
                    ),
                    doc_type => (
                        str::is_empty => "Document type cannot be empty"
                    )
                )
            },
            ServiceDefinition::Ftp { environment: ftp } => service_definition_check! {
                (mname, sname, ftp),
                (
                    matching_content => (
                        str::is_empty => "Service match cannot be empty"
                    ),
                    filecontents => (
                        str::is_empty => "File contents cannot be empty"
                    ),
                    remotefilepath => (
                        str::is_empty => "Remote file path cannot be empty"
                    )
                )
            },
            ServiceDefinition::Http { environment: http }
            | ServiceDefinition::Https { environment: http }
            | ServiceDefinition::Wordpress { environment: http } => {
                service_definition_check! {
                    (mname, sname, http),
                    (
                        matching_content => (
                            str::is_empty => "Service match cannot be empty"
                        ),
                        useragent => (
                            str::is_empty => "User agent cannot be empty"
                        ),
                        vhost => (
                            str::is_empty => "Virtual host cannot be empty"
                        ),
                        uri => (
                            str::is_empty => "URI cannot be empty"
                        )
                    )
                }
            }
            ServiceDefinition::Imap { environment: imap }
            | ServiceDefinition::Imaps { environment: imap } => {
                service_definition_check! {
                    (mname, sname, imap),
                    (
                        matching_content => (
                            str::is_empty => "Service match cannot be empty"
                        ),
                        domain => (
                            str::is_empty => "IMAP domain cannot be empty"
                        )
                    )
                }
            }
            ServiceDefinition::Ldap { environment: ldap } => service_definition_check! {
                (mname, sname, ldap),
                (
                    matching_content => (
                        str::is_empty => "Service match cannot be empty"
                    ),
                    domain => (
                        str::is_empty => "LDAP domain cannot be empty"
                    ),
                    base_dn => (
                        str::is_empty => "Base DN cannot be empty"
                    )
                )
            },
            ServiceDefinition::Mssql { environment: sql }
            | ServiceDefinition::Mysql { environment: sql }
            | ServiceDefinition::PostgreSql { environment: sql } => service_definition_check! {
                (mname, sname, sql),
                (
                    matching_content => (
                        str::is_empty => "Service match cannot be empty"
                    ),
                    database => (
                        str::is_empty => "Database cannot be empty"
                    ),
                    command => (
                        str::is_empty => "Command to execute cannot be empty"
                    )
                )
            },
            ServiceDefinition::Nfs { environment: nfs } => service_definition_check! {
                (mname, sname, nfs),
                (
                    matching_content => (
                        str::is_empty => "Service match cannot be empty"
                    ),
                    remotefilepath => (
                        str::is_empty => "Remote file path cannot be empty"
                    ),
                    filecontents => (
                        str::is_empty => "File contents cannot be empty"
                    )
                )
            },
            ServiceDefinition::Pop3 { environment: pop }
            | ServiceDefinition::Pop3s { environment: pop } => {
                service_definition_check! {
                    (mname, sname, pop),
                    (
                        matching_content => (
                            str::is_empty => "Service match cannot be empty"
                        ),
                        domain => (
                            str::is_empty => "Domain cannot be empty"
                        )
                    )
                }
            }
            ServiceDefinition::Smb { environment: smb } => service_definition_check! {
                (mname, sname, smb),
                (
                    matching_content => (
                        str::is_empty => "Service match cannot be empty"
                    ),
                    remote_name => (
                        str::is_empty => "Remote name cannot be empty"
                    ),
                    share => (
                        str::is_empty => "Share name cannot be empty"
                    ),
                    file => (
                        str::is_empty => "File name cannot be empty"
                    ),
                    hash => (
                        str::is_empty => "File hash cannot be empty",
                        |hash: &str| hash.len() != 64 => "File hash must be 64 characters"
                    )
                )
            },
            ServiceDefinition::Smtp { environment: smtp }
            | ServiceDefinition::Smtps { environment: smtp } => {
                service_definition_check! {
                    (mname, sname, smtp),
                    (
                        matching_content => (
                            str::is_empty => "Service match cannot be empty"
                        ),
                        touser => (
                            str::is_empty => "'To' destination email cannot be empty",
                            |email: &str| !email.contains('@') => "Email must contain an '@' symbol"
                        )
                    )
                }
            }
            ServiceDefinition::Ssh { environment: cmd }
            | ServiceDefinition::WinRm { environment: cmd } => {
                service_definition_check! {
                    (mname, sname, cmd),
                    (
                        matching_content => (
                            str::is_empty => "Service match must be empty"
                        ),
                        commands => (
                            str::is_empty => "Commands cannot be empty"
                        )
                    )
                }
            }
            ServiceDefinition::Icmp { environment: None } => Ok(vec![Environment {
                matching_content: "1 packets transmitted, 1 received".to_string(),
                properties: vec![],
            }]),
            ServiceDefinition::Rdp { environment: None } => Ok(vec![Environment {
                matching_content: "SUCCESS$".to_string(),
                properties: vec![],
            }]),
            ServiceDefinition::Vnc { environment: None } => Ok(vec![Environment {
                matching_content: "ACCOUNT FOUND".to_string(),
                properties: vec![],
            }]),
            ServiceDefinition::Icmp {
                environment: Some(matcher),
            }
            | ServiceDefinition::Rdp {
                environment: Some(matcher),
            }
            | ServiceDefinition::Vnc {
                environment: Some(matcher),
            } => Ok(vec![Environment {
                matching_content: matcher.clone(),
                properties: vec![],
            }]),
            _ => Ok(vec![]),
        }
    }

    pub fn check_name(&self) -> &'static str {
        match self {
            ServiceDefinition::Dns { .. } => "DNSCheck",
            ServiceDefinition::Docker { .. } => "DockerCheck",
            ServiceDefinition::Elasticsearch { .. } => "ElasticsearchCheck",
            ServiceDefinition::Ftp { .. } => "FTPCheck",
            ServiceDefinition::Http { .. } => "HTTPCheck",
            ServiceDefinition::Https { .. } => "HTTPSCheck",
            ServiceDefinition::Icmp { .. } => "ICMPCheck",
            ServiceDefinition::Imap { .. } => "IMAPCheck",
            ServiceDefinition::Imaps { .. } => "IMAPSCheck",
            ServiceDefinition::Ldap { .. } => "LDAPCheck",
            ServiceDefinition::Mssql { .. } => "MSSQLCheck",
            ServiceDefinition::Mysql { .. } => "MYSQLCheck",
            ServiceDefinition::Nfs { .. } => "NFSCheck",
            ServiceDefinition::Pop3 { .. } => "POP3Check",
            ServiceDefinition::Pop3s { .. } => "POP3SCheck",
            ServiceDefinition::PostgreSql { .. } => "PostgreSQLCheck",
            ServiceDefinition::Rdp { .. } => "RDPCheck",
            ServiceDefinition::Smb { .. } => "SMBCheck",
            ServiceDefinition::Smtp { .. } => "SMTPCheck",
            ServiceDefinition::Smtps { .. } => "SMTPSCheck",
            ServiceDefinition::Ssh { .. } => "SSHCheck",
            ServiceDefinition::Vnc { .. } => "VNCCheck",
            ServiceDefinition::WinRm { .. } => "WinRMCheck",
            ServiceDefinition::Wordpress { .. } => "WordpressCheck",
        }
    }
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
#[serde(tag = "scheme")]
pub enum IpGeneratorScheme {
    OneTeam,
    ReplaceXWithId,
    ReplaceXWithIdTimesMultiplierPlusOffset { multiplier: u8 },
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
                return Err(ConversionError::XInManualIP(machine_name.to_owned()));
            } else if let Some(other_machine) = used_ips.get(ip_template) {
                if other_machine != machine_name {
                    return Err(ConversionError::DuplicateIPs(
                        ip_template.to_owned(),
                        machine_name.to_owned(),
                        other_machine.to_owned(),
                    ));
                }
            }

            used_ips.insert(ip_template.to_owned(), machine_name.to_owned());

            Ok(ip_template.to_string())
        }
        ReplaceXWithId => {
            if !ip_template.chars().any(|c| c == 'x' || c == 'X') {
                return Err(ConversionError::NoXInTemplateIP(machine_name.to_owned()));
            } else {
                let ip = ip_template
                    .replace('X', &id.to_string())
                    .replace('x', &id.to_string());

                if let Some(other_machine) = used_ips.get(&ip) {
                    if other_machine != machine_name {
                        return Err(ConversionError::DuplicateIPs(
                            ip,
                            machine_name.to_owned(),
                            other_machine.to_owned(),
                        ));
                    }
                }

                used_ips.insert(ip.to_owned(), machine_name.to_owned());
                Ok(ip)
            }
        }
        ReplaceXWithIdTimesMultiplierPlusOffset { multiplier } => {
            let Some(ip_offset) = ip_offset else {
                return Err(ConversionError::OffsetNotSpecified(machine_name.to_owned()));
            };
            let ip = multiplier * id + ip_offset;
            if ip_template.chars().any(|c| c == 'x' || c == 'X') {
                Ok(ip_template
                    .replace('X', &ip.to_string())
                    .replace('x', &ip.to_string()))
            } else {
                Err(ConversionError::NoXInTemplateIP(machine_name.to_owned()))
            }
        }
    }
}

pub fn convert_editor_to_final(
    config: &ConfigurationEditor,
) -> Result<(FinalConfiguration, ConfigurationEditor), ConversionError> {
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

    if let IpGeneratorScheme::ReplaceXWithIdTimesMultiplierPlusOffset { multiplier: mult } =
        config.ip_generator
    {
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

    {
        let mut machine_names: HashSet<&str> = HashSet::new();

        for machine in &config.machines {
            if machine.name.is_empty() {
                return Err(ConversionError::MachineHasEmptyName);
            }

            if machine_names.contains(&*machine.name) {
                return Err(ConversionError::DuplicateMachineNames(machine.name.clone()));
            }

            machine_names.insert(&*machine.name);
        }
    }

    fn services_generator(
        conversion_state: &mut ConversionState,
        config: &ConfigurationEditor,
        team: &BlueTeamEditor,
    ) -> Result<Vec<ServiceConfig>, ConversionError> {
        Ok(config
            .machines
            .iter()
            .map(|machine| -> Result<Vec<ServiceConfig>, ConversionError> {
                {
                    let mut service_names: HashSet<&str> = HashSet::new();

                    for service in &machine.services {
                        if service.name.is_empty() {
                            return Err(ConversionError::MachineHasEmptyService(
                                machine.name.clone(),
                            ));
                        }

                        if service_names.contains(&*service.name) {
                            return Err(ConversionError::DuplicateServiceName(
                                machine.name.clone(),
                                service.name.clone(),
                            ));
                        }

                        service_names.insert(&*service.name);
                    }
                }

                Ok(machine
                    .services
                    .iter()
                    .map(|service| -> Result<ServiceConfig, ConversionError> {
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
                            environments: service
                                .definition
                                .environments(&machine.name, &service.name)?,
                        })
                    })
                    .collect::<Result<Vec<_>, ConversionError>>()?)
            })
            .collect::<Result<Vec<_>, ConversionError>>()?
            .concat())
    }

    {
        let mut blue_ids_map: HashMap<u8, Vec<&str>> = HashMap::new();

        for team in &config.blue_teams {
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
                services: services_generator(&mut conversion_state, &config, team)?,
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

    Ok((
        FinalConfiguration {
            teams: [red_white, blue].concat(),
        },
        config,
    ))
}
