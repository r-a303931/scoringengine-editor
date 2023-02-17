// machines.rs: Machine configuration menu
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

use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::{
    config::{self, MachineEditor},
    state,
};

macro_rules! count_properties {
    () => (0usize);
    ($p:ident,$($p2:ident,)*) => (1usize + count_properties!($($p2,)*));
}

macro_rules! define_service_environment_editor {
    (Option<$type:ty>, $props:expr, $($property:ident => $property_name:expr),*) => {
        html! {}
    };
    (Vec<$type:ty>, $props:expr, $($property:ident => $property_name:expr),*) => {
        html! {}
    };
}

macro_rules! setup_service {
    (
        ($name:ident, $pretty_name:expr, $service_definition_type:ty),
        ServiceEditor {
            name => $new_name:expr,
            port => $new_port:expr,
            points => $new_points:expr,
            accounts => $new_accounts:expr,
            definition => $new_service:ident
        },
        ($($property:ident => $prop_pretty_name:expr),*)
    ) => {
        setup_service!{
            ($name, $pretty_name, $service_definition_type),
            ServiceEditor {
                name => $new_name,
                port => $new_port,
                points => $new_points,
                accounts => $new_accounts,
                definition => $new_service, vec![]
            },
            ($($property => $prop_pretty_name),*)
        }
    };
    (
        ($name:ident, $pretty_name:expr, $service_definition_type:ty),
        ServiceEditor {
            name => $new_name:expr,
            port => $new_port:expr,
            points => $new_points:expr,
            accounts => $new_accounts:expr,
            definition => $new_service:ident, $new_service_params:expr
        },
        ($($property:ident => $prop_pretty_name:expr),*)
    ) => {
        mod $name {
            use crate::config::{self, ServiceEditor};
            use yew::prelude::*;
            use web_sys::HtmlInputElement;

            #[derive(Properties, PartialEq)]
            pub struct NewServiceComponentProps {
                pub name_filter: AttrValue,
                pub handle_pickup: Callback<config::ServiceEditor>,
                pub handle_dragend: Callback<()>,
            }

            #[function_component]
            pub fn NewServiceComponent(props: &NewServiceComponentProps) -> Html {
                let ondragstart = {
                    let handle_pickup = props.handle_pickup.clone();

                    Callback::from(move |_| {
                        handle_pickup.emit(ServiceEditor {
                            name: $new_name.to_string(),
                            port: $new_port,
                            points: $new_points,
                            accounts: $new_accounts,
                            definition: config::ServiceDefinition::$new_service { environment: $new_service_params },
                        });
                    })
                };

                let ondragend = {
                    let handle_dragend = props.handle_dragend.clone();

                    Callback::from(move |_| {
                        handle_dragend.emit(());
                    })
                };

                html! {
                    <div
                        draggable={"true"}
                        class={classes!(
                            "new-service",
                            Some("hidden").filter(|_| !$pretty_name.to_lowercase().contains(&props.name_filter.to_lowercase()))
                        )}
                        {ondragstart}
                        {ondragend}
                    >
                        <h3>
                            { $pretty_name }
                        </h3>

                        <div class="service-details">
                            if $new_port != 0 {
                                <div class="service-detail">
                                    <span>{ "Default port: " }</span>
                                    { ($new_port).to_string() }
                                </div>
                            }

                            <div class="service-detail">
                                <span>{ "Default points: " }</span>
                                { ($new_points).to_string() }
                            </div>

                            <div class="service-detail">
                                <span>{ "Accounts: " }</span>
                                { if {
                                    let new_accounts: Option<Vec<config::User>> = $new_accounts;
                                    new_accounts.is_some()
                                } {
                                    "Yes"
                                } else {
                                    "No"
                                } }
                            </div>
                        </div>

                        if count_properties!($($property,)*) != 0 {
                            <div class="service-environment">
                                <h4>
                                    { "Service properties:" }
                                </h4>

                                $(
                                    <div class="new-service-property">
                                        { $prop_pretty_name }
                                    </div>
                                )*
                            </div>
                        }
                    </div>
                }
            }

            #[derive(Properties, PartialEq)]
            pub struct ServiceEditorProps {
                pub update_service: Callback<config::ServiceEditor>,
                pub delete_service: Callback<()>,
                pub name: String,
                pub port: u16,
                pub points: u16,
                pub accounts: Option<Vec<config::User>>,
                pub service_definition: $service_definition_type
            }

            #[function_component]
            pub fn ServiceEditorComponent(props: &ServiceEditorProps) -> Html {
                let delete_service = {
                    let delete_service = props.delete_service.clone();

                    Callback::from(move |_| delete_service.emit(()))
                };

                let service_editor_error = use_state(Option::<AttrValue>::default);

                #[derive(Copy, Clone)]
                enum Tabs {
                    Essentials,
                    Environments,
                    Accounts
                }

                let current_tab_index = use_state(|| Tabs::Essentials);

                let tab_click_handler = |new_tab: Tabs| -> Callback<MouseEvent> {
                    let current_tab_index = current_tab_index.clone();

                    Callback::from(move |_| {
                        current_tab_index.set(new_tab);
                    })
                };

                let service_port_ref = use_node_ref();

                let set_service_port = {
                    let service_editor_error = service_editor_error.clone();
                    let service_port_ref = service_port_ref.clone();
                    let update_service = props.update_service.clone();
                    let name = props.name.clone();
                    let points = props.points;
                    let accounts = props.accounts.clone();
                    let service = props.service_definition.clone();

                    Callback::from(move |_| {
                        let Some(input) = service_port_ref.cast::<HtmlInputElement>() else { return; };

                        match input.value().parse::<u16>() {
                            Ok(port) => {
                                service_editor_error.set(None);
                                update_service.emit(config::ServiceEditor {
                                    name: name.clone(),
                                    port,
                                    points,
                                    accounts: accounts.clone(),
                                    definition: config::ServiceDefinition::$new_service {
                                        environment: service.clone()
                                    }
                                });
                            }
                            Err(e) => {
                                service_editor_error.set(Some(format!("Error parsing service port: {e:?}").into()));
                            }
                        }
                    })
                };

                let service_points_ref = use_node_ref();

                let set_service_points = {
                    let service_editor_error = service_editor_error.clone();
                    let service_points_ref = service_points_ref.clone();
                    let update_service = props.update_service.clone();
                    let name = props.name.clone();
                    let port = props.port;
                    let accounts = props.accounts.clone();
                    let service = props.service_definition.clone();

                    Callback::from(move |_| {
                        let Some(input) = service_points_ref.cast::<HtmlInputElement>() else { return; };

                        match input.value().parse::<u16>() {
                            Ok(points) => {
                                service_editor_error.set(None);
                                update_service.emit(config::ServiceEditor {
                                    name: name.clone(),
                                    port,
                                    points,
                                    accounts: accounts.clone(),
                                    definition: config::ServiceDefinition::$new_service {
                                        environment: service.clone()
                                    }
                                });
                            }
                            Err(e) => {
                                service_editor_error.set(Some(format!("Error parsing service port: {e:?}").into()));
                            }
                        }
                    })
                };

                let service_name_ref = use_node_ref();

                let set_service_name = {
                    let service_name_ref = service_name_ref.clone();
                    let update_service = props.update_service.clone();
                    let port = props.port;
                    let points = props.points;
                    let accounts = props.accounts.clone();
                    let service = props.service_definition.clone();

                    Callback::from(move |_| {
                        let Some(input) = service_name_ref.cast::<HtmlInputElement>() else { return; };
                        let new_service = config::ServiceEditor {
                            name: input.value().clone(),
                            port,
                            points,
                            accounts: accounts.clone(),
                            definition: config::ServiceDefinition::$new_service {
                                environment: service.clone()
                            }
                        };

                        update_service.emit(new_service);
                    })
                };

                let add_account = {
                    let update_service = props.update_service.clone();
                    let name = props.name.clone();
                    let port = props.port;
                    let points = props.points;
                    let accounts = props.accounts.clone();
                    let service = props.service_definition.clone();

                    Callback::from(move |_| {
                        let accounts = accounts.clone().map(|accounts| {
                            let mut accounts = accounts.clone();
                            accounts.push(config::User {
                                username: "".to_owned(),
                                password: "Chiapet1!".to_owned()
                            });
                            accounts
                        });
                        let new_service = config::ServiceEditor {
                            name: name.clone(),
                            port,
                            points,
                            accounts,
                            definition: config::ServiceDefinition::$new_service {
                                environment: service.clone()
                            }
                        };
                        update_service.emit(new_service);
                    })
                };

                #[derive(Properties, PartialEq)]
                struct AccountEditorProps {
                    pub update_user: Callback<config::User>,
                    pub delete_user: Callback<()>,
                    pub user: config::User,
                }

                #[function_component]
                fn AccountEditor(props: &AccountEditorProps) -> Html {
                    let username_ref = use_node_ref();

                    let username_change = {
                        let update_user = props.update_user.clone();
                        let user = props.user.clone();
                        let username_ref = username_ref.clone();

                        Callback::from(move |_| {
                            let Some(input) = username_ref.cast::<HtmlInputElement>() else { return; };
                            let mut new_user = user.clone();
                            new_user.username = input.value();
                            update_user.emit(new_user);
                        })
                    };

                    let password_ref = use_node_ref();

                    let password_change = {
                        let update_user = props.update_user.clone();
                        let user = props.user.clone();
                        let password_ref = password_ref.clone();

                        Callback::from(move |_| {
                            let Some(input) = password_ref.cast::<HtmlInputElement>() else { return; };
                            let mut new_user = user.clone();
                            new_user.password = input.value();
                            update_user.emit(new_user);
                        })
                    };

                    let delete_user = {
                        let delete_user = props.delete_user.clone();

                        Callback::from(move |_| delete_user.emit(()))
                    };

                    html! {
                        <div class="service-user">
                            <div class="service-user-row">
                                <div>
                                    { "Username" }
                                </div>

                                <div>
                                    <input
                                        value={props.user.username.clone()}
                                        ref={username_ref}
                                        onchange={username_change}
                                    />
                                </div>
                            </div>

                            <div class="service-user-row">
                                <div>
                                    { "Password" }
                                </div>

                                <div>
                                    <input
                                        value={props.user.password.clone()}
                                        onchange={password_change}
                                        ref={password_ref}
                                    />
                                </div>
                            </div>

                            <div class="service-user-row">
                                <div />

                                <div>
                                    <a href="#" onclick={delete_user}>
                                        { "Delete user" }
                                    </a>
                                </div>
                            </div>
                        </div>
                    }
                }

                let accounts = props.accounts.clone().unwrap_or(vec![]);
                let accounts = accounts.iter().enumerate().map(|(i, account)| {
                    let update_service = props.update_service.clone();
                    let name = props.name.clone();
                    let port = props.port;
                    let points = props.points;
                    let service = props.service_definition.clone();
                    let accounts = props.accounts.clone();

                    let update_user = {
                        let update_service = update_service.clone();
                        let name = name.clone();
                        let port = port;
                        let points = points;
                        let service = service.clone();
                        let accounts = accounts.clone();

                        Callback::from(move |account| {
                            let accounts = accounts.as_ref().map(|accounts| {
                                let mut new_accounts = accounts.clone();
                                new_accounts[i] = account;
                                new_accounts
                            });
                            update_service.emit(ServiceEditor {
                                name: name.clone(),
                                port,
                                points,
                                definition: config::ServiceDefinition::$new_service {
                                    environment: service.clone()
                                },
                                accounts
                            })
                        })
                    };

                    let delete_user = {
                        let update_service = update_service.clone();
                        let name = name.clone();
                        let port = port;
                        let points = points;
                        let service = service.clone();
                        let accounts = accounts.clone();

                        Callback::from(move |_| {
                            let accounts = accounts.as_ref().map(|accounts| {
                                let mut new_accounts = accounts.clone();
                                new_accounts.remove(i);
                                new_accounts
                            });
                            update_service.emit(ServiceEditor {
                                name: name.clone(),
                                port,
                                points,
                                definition: config::ServiceDefinition::$new_service {
                                    environment: service.clone()
                                },
                                accounts
                            })
                        })
                    };

                    html! {
                        <AccountEditor
                            key={i}
                            user={account.clone()}
                            {update_user}
                            {delete_user}
                        />
                    }
                });

                html! {
                    <div class="machine-service">
                        <div class="machine-service-header">
                            <h3>
                                { $pretty_name } { ":" }
                            </h3>

                            <a href="#" onclick={delete_service}>
                                { "Remove service" }
                            </a>
                        </div>

                        if let Some(err) = &*service_editor_error {
                            <div class="error">
                                { err }
                            </div>
                        }

                        <div class="machine-service-properties">
                            <div class="service-properties-tabs">
                                <a
                                    class={classes!(
                                        "service-properties-tab",
                                        Some("selected").filter(|_| matches!(*current_tab_index, Tabs::Essentials))
                                    )}
                                    onclick={tab_click_handler(Tabs::Essentials)}
                                >
                                    { "Basic properties" }
                                </a>

                                <a
                                    class={classes!(
                                        "service-properties-tab",
                                        Some("selected").filter(|_| matches!(*current_tab_index, Tabs::Environments))
                                    )}
                                    onclick={tab_click_handler(Tabs::Environments)}
                                >
                                    { "Checks" }
                                </a>

                                <a
                                    class={classes!(
                                        "service-properties-tab",
                                        Some("selected").filter(|_| matches!(*current_tab_index, Tabs::Accounts)),
                                        Some("hidden").filter(|_| {
                                            let accounts: Option<Vec<config::User>> = $new_accounts;
                                            accounts.is_none()
                                        })
                                    )}
                                    onclick={tab_click_handler(Tabs::Accounts)}
                                >
                                    { "Accounts" }
                                </a>
                            </div>

                            <div
                                class={classes!(
                                    "service-properties-pane",
                                    Some("hidden").filter(|_| !matches!(*current_tab_index, Tabs::Essentials))
                                )}
                            >
                                <div class="service-property">
                                    <div class="service-property-name">
                                        { "Service name:" }
                                    </div>

                                    <div class="service-property-value">
                                        <input
                                            ref={service_name_ref}
                                            value={props.name.clone()}
                                            onchange={set_service_name}
                                        />
                                    </div>
                                </div>

                                <div class="service-property">
                                    <div class="service-property-name">
                                        { "Service port:" }
                                    </div>

                                    <div class="service-property-value">
                                        <input
                                            ref={service_port_ref}
                                            value={props.port.to_string()}
                                            onchange={set_service_port}
                                        />
                                    </div>
                                </div>

                                <div class="service-property">
                                    <div class="service-property-name">
                                        { "Points:" }
                                    </div>

                                    <div class="service-property-value">
                                        <input
                                            ref={service_points_ref}
                                            value={props.points.to_string()}
                                            onchange={set_service_points}
                                        />
                                    </div>
                                </div>
                            </div>

                            <div
                                class={classes!(
                                    "service-properties-pane",
                                    Some("hidden").filter(|_| !matches!(*current_tab_index, Tabs::Environments))
                                )}
                            >
                                { "Environments" }
                            </div>

                            <div
                                class={classes!(
                                    "service-properties-pane",
                                    Some("hidden").filter(|_| !matches!(*current_tab_index, Tabs::Accounts))
                                )}
                            >
                                <a href="#" onclick={add_account} class="add-user">
                                    { "Add account" }
                                </a>

                                { for accounts }
                            </div>
                        </div>
                    </div>
                }
            }
        }
    };
}

macro_rules! setup_general_service_editor {
    ($($case:ident => $mod:ident),*) => {
        #[derive(Properties, PartialEq)]
        struct ServiceEditorComponentProps {
            pub update_service: Callback<config::ServiceEditor>,
            pub delete_service: Callback<()>,
            pub service_to_edit: config::ServiceEditor,
        }

        #[function_component]
        fn ServiceEditorComponent(props: &ServiceEditorComponentProps) -> Html {
            match &props.service_to_edit.definition {
                $(
                    config::ServiceDefinition::$case { environment } => html! {
                        <$mod::ServiceEditorComponent
                            update_service={props.update_service.clone()}
                            delete_service={props.delete_service.clone()}
                            name={props.service_to_edit.name.clone()}
                            port={props.service_to_edit.port}
                            points={props.service_to_edit.points}
                            accounts={props.service_to_edit.accounts.clone()}
                            service_definition={environment.clone()}
                        />
                    }
                ),*
            }
        }

        #[derive(Properties, PartialEq)]
        struct ServiceListComponentProps {
            pub name_filter: AttrValue,
            pub handle_pickup: Callback<config::ServiceEditor>,
            pub handle_dragend: Callback<()>,
        }

        #[function_component]
        fn NewServiceListComponent(props: &ServiceListComponentProps) -> Html {
            html! {
                <>
                    $(
                        <$mod::NewServiceComponent
                            name_filter={props.name_filter.clone()}
                            handle_pickup={props.handle_pickup.clone()}
                            handle_dragend={props.handle_dragend.clone()}
                        />
                    )*
                </>
            }
        }
    };
}

setup_service! {
    (dns, "DNS", Vec<config::DnsCheckInfo>),
    ServiceEditor {
        name => "DNS",
        port => 53,
        points => 150,
        accounts => None,
        definition => Dns
    },
    (
        qtype => "Query type",
        domain => "Domain"
    )
}
setup_service! {
    (docker, "Docker", Vec<config::DockerCheckInfo>),
    ServiceEditor {
        name => "Docker",
        port => 2375,
        points => 100,
        accounts => None,
        definition => Docker
    },
    ()
}
setup_service! {
    (elasticsearch, "Elasticsearch", Vec<config::ElasticsearchCheckInfo>),
    ServiceEditor {
        name => "Elasticsearch",
        port => 9200,
        points => 100,
        accounts => None,
        definition => Elasticsearch
    },
    (
        index => "Index",
        doc_type => "Document type"
    )
}
setup_service! {
    (ftp, "FTP", Vec<config::FtpCheckInfo>),
    ServiceEditor {
        name => "FTP",
        port => 21,
        points => 150,
        accounts => Some(vec![]),
        definition => Ftp
    },
    (
        remotefilepath => "Remote file path",
        filecontents => "File contents"
    )
}
setup_service! {
    (http, "HTTP", Vec<config::HttpCheckInfo>),
    ServiceEditor {
        name => "HTTP",
        port => 80,
        points => 150,
        accounts => None,
        definition => Http
    },
    (
        useragent => "Browser user agent",
        vhost => "Remote host name",
        uri => "Request URI"
    )
}
setup_service! {
    (https, "HTTPS", Vec<config::HttpCheckInfo>),
    ServiceEditor {
        name => "HTTPS",
        port => 80,
        points => 150,
        accounts => None,
        definition => Https
    },
    (
        useragent => "Browser user agent",
        vhost => "Remote host name",
        uri => "Request URI"
    )
}
setup_service! {
    (icmp, "ICMP Ping", Option<String>),
    ServiceEditor {
        name => "ICMP",
        port => 0,
        points => 25,
        accounts => None,
        definition => Icmp, None
    },
    ()
}
setup_service! {
    (imap, "IMAP", Vec<config::ImapCheckInfo>),
    ServiceEditor {
        name => "IMAP",
        port => 143,
        points => 100,
        accounts => Some(vec![]),
        definition => Imap
    },
    (
        domain => "Email domain"
    )
}
setup_service! {
    (imaps, "IMAPS", Vec<config::ImapCheckInfo>),
    ServiceEditor {
        name => "IMAPS",
        port => 143,
        points => 100,
        accounts => Some(vec![]),
        definition => Imap
    },
    (
        domain => "Email domain"
    )
}
setup_service! {
    (ldap, "LDAP", Vec<config::LdapCheckInfo>),
    ServiceEditor {
        name => "LDAP",
        port => 389,
        points => 50,
        accounts => Some(vec![]),
        definition => Ldap
    },
    (
        domain => "LDAP domain",
        base_dn => "Base DN"
    )
}
setup_service! {
    (mssql, "MSSQL", Vec<config::SqlCheckInfo>),
    ServiceEditor {
        name => "MSSQL",
        port => 1433,
        points => 100,
        accounts => Some(vec![]),
        definition => Mssql
    },
    (
        database => "Test database",
        command => "Test command"
    )
}
setup_service! {
    (mysql, "MySQL", Vec<config::SqlCheckInfo>),
    ServiceEditor {
        name => "MySQL",
        port => 1433,
        points => 100,
        accounts => Some(vec![]),
        definition => Mysql
    },
    (
        database => "Test database",
        command => "Test command"
    )
}
setup_service! {
    (nfs, "NFS", Vec<config::NfsCheckInfo>),
    ServiceEditor {
        name => "NFS",
        port => 0,
        points => 150,
        accounts => None,
        definition => Nfs
    },
    (
        remotefilepath => "Remote file path",
        filecontents => "File contents"
    )
}
setup_service! {
    (pop3, "POP3", Vec<config::PopCheckInfo>),
    ServiceEditor {
        name => "POP3",
        port => 110,
        points => 100,
        accounts => Some(vec![]),
        definition => Pop3
    },
    (
        domain => "Email domain"
    )
}
setup_service! {
    (pop3s, "POP3S", Vec<config::PopCheckInfo>),
    ServiceEditor {
        name => "POP3S",
        port => 110,
        points => 100,
        accounts => Some(vec![]),
        definition => Pop3
    },
    (
        domain => "Email domain"
    )
}
setup_service! {
    (postgres, "PostgreSQL", Vec<config::SqlCheckInfo>),
    ServiceEditor {
        name => "PostgreSQL",
        port => 5432,
        points => 100,
        accounts => Some(vec![]),
        definition => PostgreSql
    },
    (
        database => "Test database",
        command => "Test command"
    )
}
setup_service! {
    (rdp, "RDP", Option<String>),
    ServiceEditor {
        name => "RDP",
        port => 3389,
        points => 100,
        accounts => Some(vec![]),
        definition => Rdp, None
    },
    ()
}
setup_service! {
    (smb, "SMB", Vec<config::SmbCheckInfo>),
    ServiceEditor {
        name => "SMB",
        port => 445,
        points => 100,
        accounts => Some(vec![]),
        definition => Smb
    },
    (
        remote_name => "Computer name",
        share => "Share name",
        file => "File name",
        hash => "SHA256 hash of file"
    )
}
setup_service! {
    (smtp, "SMTP", Vec<config::SmtpCheckInfo>),
    ServiceEditor {
        name => "SMTP",
        port => 25,
        points => 100,
        accounts => Some(vec![]),
        definition => Smtp
    },
    (
        touser => "Send to",
        subject => "Email subject",
        body => "Email body"
    )
}
setup_service! {
    (smtps, "SMTPS", Vec<config::SmtpCheckInfo>),
    ServiceEditor {
        name => "SMTPS",
        port => 25,
        points => 100,
        accounts => Some(vec![]),
        definition => Smtps
    },
    (
        touser => "Send to",
        subject => "Email subject",
        body => "Email body"
    )
}
setup_service! {
    (ssh, "SSH", Vec<config::RemoteCommandCheckInfo>),
    ServiceEditor {
        name => "SSH",
        port => 22,
        points => 100,
        accounts => Some(vec![]),
        definition => Ssh
    },
    (
        commands => "Commands"
    )
}
setup_service! {
    (vnc, "VNC", Option<String>),
    ServiceEditor {
        name => "VNC",
        port => 5900,
        points => 100,
        accounts => Some(vec![]),
        definition => Vnc, None
    },
    ()
}
setup_service! {
    (winrm, "WinRM", Vec<config::RemoteCommandCheckInfo>),
    ServiceEditor {
        name => "WinRM",
        port => 0,
        points => 100,
        accounts => Some(vec![]),
        definition => WinRm
    },
    (
        commands => "Commands"
    )
}
setup_service! {
    (wordpress, "Wordpress", Vec<config::HttpCheckInfo>),
    ServiceEditor {
        name => "Wordpress",
        port => 80,
        points => 100,
        accounts => Some(vec![]),
        definition => Wordpress
    },
    (
        useragent => "Browser user agent",
        vhost => "Remote host name",
        uri => "Request URI"
    )
}

setup_general_service_editor! {
    Dns => dns,
    Docker => docker,
    Elasticsearch => elasticsearch,
    Ftp => ftp,
    Http => http,
    Https => https,
    Icmp => icmp,
    Imap => imap,
    Imaps => imaps,
    Ldap => ldap,
    Mssql => mssql,
    Mysql => mysql,
    Nfs => nfs,
    Pop3 => pop3,
    Pop3s => pop3s,
    PostgreSql => postgres,
    Rdp => rdp,
    Smb => smb,
    Smtp => smtp,
    Smtps => smtps,
    Ssh => ssh,
    Vnc => vnc,
    WinRm => winrm,
    Wordpress => wordpress
}

#[derive(Properties, PartialEq)]
pub struct MachineServiceListEditorProps {
    pub update_services: Callback<Vec<config::ServiceEditor>>,
    pub services: Vec<config::ServiceEditor>,
}

#[function_component]
pub fn MachineServiceListEditor(props: &MachineServiceListEditorProps) -> Html {
    let services_vec = props.services.clone();

    let services = props.services.iter().enumerate().map(|(i, service)| {
        let service_to_edit = service.clone();

        let update_service = {
            let update_services = props.update_services.clone();
            let new_services = services_vec.clone();
            Callback::from(move |new_service| {
                let mut new_services = new_services.clone();
                new_services[i] = new_service;
                update_services.emit(new_services);
            })
        };

        let delete_service = {
            let update_services = props.update_services.clone();
            let new_services = services_vec.clone();
            Callback::from(move |_| {
                let mut new_services = new_services.clone();
                new_services.remove(i);
                update_services.emit(new_services);
            })
        };

        html! {
            <ServiceEditorComponent
                key={i}
                {update_service}
                {delete_service}
                {service_to_edit}
            />
        }
    });

    html! {
        { for services }
    }
}

#[derive(Properties, PartialEq)]
struct MachineEditorProps {
    i: u8,
    machine: MachineEditor,
}

#[function_component]
fn MachineEditorComponent(props: &MachineEditorProps) -> Html {
    let editor_state = use_context::<crate::state::EditorStateContext>().unwrap();
    let config = editor_state.force_init().2;

    let machine_editor_error = use_state(Option::<String>::default);

    let editing_name = use_state(bool::default);
    let editing_name_ref = use_node_ref();

    let start_editing_name = {
        let editing_name = editing_name.clone();
        Callback::from(move |_| editing_name.set(true))
    };

    let stop_editing_name = {
        let editing_name = editing_name.clone();
        Callback::from(move |_| editing_name.set(false))
    };

    {
        let editing_name_ref = editing_name_ref.clone();
        let editing_name_ref2 = editing_name_ref.clone();
        use_effect_with_deps(
            move |_| {
                let Some(input) = editing_name_ref.cast::<HtmlInputElement>() else { return; };
                input.focus().unwrap();
            },
            editing_name_ref2.get(),
        );
    }

    let update_name = {
        let editor_state = editor_state.clone();
        let editing_name = editing_name.clone();
        let editing_name_ref = editing_name_ref.clone();
        let machine_editor_error = machine_editor_error.clone();
        let machine = props.machine.clone();
        let i = props.i;

        Callback::from(move |_| {
            machine_editor_error.set(None);
            let Some(input) = editing_name_ref.cast::<HtmlInputElement>() else { return; };
            let mut new_machine = machine.clone();
            new_machine.name = input.value().clone();
            editor_state.dispatch(state::EditorMessage::UpdateMachine(i, new_machine));
            editing_name.set(false);
        })
    };

    let ip_template_ref = use_node_ref();

    let on_ip_template_change = {
        let ip_template_ref = ip_template_ref.clone();
        let machine_editor_error = machine_editor_error.clone();
        let editor_state = editor_state.clone();
        let i = props.i;
        let machine = props.machine.clone();

        Callback::from(move |_| {
            machine_editor_error.set(None);
            let Some(input) = ip_template_ref.cast::<HtmlInputElement>() else { return; };
            let mut new_machine = machine.clone();
            new_machine.ip_template = input.value().clone();
            editor_state.dispatch(state::EditorMessage::UpdateMachine(i, new_machine));
        })
    };

    let ip_offset_ref = use_node_ref();

    let on_ip_offset_change = {
        let ip_offset_ref = ip_offset_ref.clone();
        let machine_editor_error = machine_editor_error.clone();
        let editor_state = editor_state.clone();
        let i = props.i;
        let machine = props.machine.clone();

        Callback::from(move |_| {
            let Some(input) = ip_offset_ref.cast::<HtmlInputElement>() else { return; };

            match input.value().parse::<u8>() {
                Ok(offset) => {
                    let mut new_machine = machine.clone();
                    new_machine.ip_offset = Some(offset);
                    editor_state.dispatch(state::EditorMessage::UpdateMachine(i, new_machine));
                }
                Err(e) => {
                    machine_editor_error.set(Some(format!("Parse error: {e:?}")));
                }
            }
        })
    };

    let delete_machine = {
        let editor_state = editor_state.clone();
        let i = props.i;

        Callback::from(move |_| {
            editor_state.dispatch(state::EditorMessage::RemoveMachine(i));
        })
    };

    let ondragover = {
        let editor_state = editor_state.clone();
        let machine_name = props.machine.name.clone();
        let i = props.i;
        let is_editing = *editing_name;

        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            if editor_state.force_init().5.is_none() || machine_name.is_empty() || is_editing {
                return;
            }
            editor_state.dispatch(state::EditorMessage::HoverOverMachine(i));
        })
    };

    let ondragleave = {
        let editor_state = editor_state.clone();

        Callback::from(move |_| {
            editor_state.dispatch(state::EditorMessage::StopHoveringOverMachines);
        })
    };

    let ondrop = {
        let editor_state = editor_state.clone();
        let i = props.i;
        let is_name_empty = props.machine.name.is_empty();
        let machine_editor_error = machine_editor_error.clone();

        Callback::from(move |e: DragEvent| {
            e.prevent_default();

            if is_name_empty {
                machine_editor_error.set(Some(
                    "Please give the machine a name before adding services".to_owned(),
                ));

                return;
            }

            editor_state.dispatch(state::EditorMessage::DropService(i));
        })
    };

    let update_services = {
        let editor_state = editor_state.clone();
        let i = props.i;
        let name = props.machine.name.clone();
        let ip_offset = props.machine.ip_offset.clone();
        let ip_template = props.machine.ip_template.clone();

        Callback::from(move |new_services| {
            editor_state.dispatch(state::EditorMessage::UpdateMachine(
                i,
                MachineEditor {
                    name: name.clone(),
                    ip_offset: ip_offset.clone(),
                    ip_template: ip_template.clone(),
                    services: new_services,
                },
            ))
        })
    };

    let hovering_class = Some("hovering").filter(|_| {
        editor_state
            .force_init()
            .4
            .map(|hovering| hovering == props.i)
            .unwrap_or(false)
    });

    html! {
        <div {ondragover} {ondragleave} {ondrop} class={classes!("machine", hovering_class)}>
            <div class="machine-header">
                <div class="machine-name">
                    if *editing_name || props.machine.name.is_empty() {
                        <input
                            ref={editing_name_ref}
                            onchange={update_name}
                            onblur={stop_editing_name}
                            value={props.machine.name.clone()}
                            placeholder="Machine name"
                        />
                    } else {
                        <h3 onclick={start_editing_name}>
                            { props.machine.name.clone() }
                        </h3>
                    }
                </div>

                <a href="#" onclick={delete_machine}>
                    { "Delete machine" }
                </a>
            </div>

            if let Some(err) = &*machine_editor_error {
                <div class="machine-error">
                    {err}
                </div>
            }

            <div class="machine-body">
                <div class="machine-properties">
                    <div class="machine-property">
                        <div class="machine-property-name">
                            if matches!(config.ip_generator, config::IpGeneratorScheme::OneTeam) {
                                { "IP address:" }
                            } else {
                                { "IP template:" }
                            }
                        </div>

                        <div class="machine-property-value">
                            <input
                                value={props.machine.ip_template.clone()}
                                ref={ip_template_ref}
                                onchange={on_ip_template_change}
                            />
                        </div>
                    </div>

                    <div
                        class={classes!(
                            "machine-property",
                            Some("hidden")
                                .filter(|_| !matches!(config.ip_generator, config::IpGeneratorScheme::ReplaceXWithIdTimesMultiplierPlusOffset { .. }))
                        )}
                    >
                        <div class="machine-property-name">
                            { "IP multiplier offset:" }
                        </div>

                        <div class="machine-property-value">
                            <input
                                value={props.machine.ip_offset.map(|off| off.to_string()).unwrap_or_default()}
                                ref={ip_offset_ref}
                                onchange={on_ip_offset_change}
                            />
                        </div>
                    </div>
                </div>

                <div class="machine-services">
                    <MachineServiceListEditor
                        {update_services}
                        services={props.machine.services.clone()}
                    />
                </div>
            </div>
        </div>
    }
}

#[function_component]
pub fn MachineConfiguration() -> Html {
    let editor_state = use_context::<crate::state::EditorStateContext>().unwrap();
    let config = editor_state.force_init().2;

    let handle_pickup = {
        let editor_state = editor_state.clone();

        Callback::from(move |new_service| {
            editor_state.dispatch(state::EditorMessage::PickupService(new_service));
        })
    };

    let handle_dragend = {
        let editor_state = editor_state.clone();

        Callback::from(move |()| {
            editor_state.dispatch(state::EditorMessage::StopHoveringOverMachines);
        })
    };

    let add_machine = {
        let editor_state = editor_state.clone();

        Callback::from(move |_| {
            editor_state.dispatch(state::EditorMessage::AddMachine(MachineEditor {
                name: "".to_owned(),
                ip_offset: None,
                ip_template: "".to_owned(),
                services: vec![],
            }));
        })
    };

    let machine_list = config.machines.iter().enumerate().map(|(i, machine)| {
        let i: u8 = i.try_into().unwrap();

        html! {
            <MachineEditorComponent
                key={i}
                {i}
                machine={machine.clone()}
            />
        }
    });

    let name_filter = use_state(AttrValue::default);

    let set_name_filter_ref = use_node_ref();

    let set_name = {
        let name_filter = name_filter.clone();
        let set_name_filter_ref = set_name_filter_ref.clone();

        Callback::from(move |_| {
            let Some(input) = set_name_filter_ref.cast::<HtmlInputElement>() else { return; };
            name_filter.set(input.value().into());
        })
    };

    html! {
        <main id="machines">
            <div class="service-list-header">
                <input
                    ref={set_name_filter_ref}
                    value={&*name_filter}
                    oninput={set_name}
                    placeholder="Search services..."
                />
            </div>

            <div class="service-list">
                <NewServiceListComponent
                    name_filter={&*name_filter}
                    {handle_pickup}
                    {handle_dragend}
                />
            </div>

            <div class="machine-list-header">
                <a href="#" onclick={add_machine}>
                    { "Add machine" }
                </a>
            </div>

            <div class="machine-list">
                { for machine_list }
            </div>
        </main>
    }
}
