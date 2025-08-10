# WSLNetMan UI Design Plan

## Main Window Structure

The main window will be a tabbed interface with the following tabs:
1. Networks
2. Firewall Rules
3. Routing Rules
4. Docker Networks
5. Packet Sender

## Component Hierarchy

```
MainWindow
├── TabWidget
│   ├── NetworkTab
│   │   ├── NetworkTree
│   │   ├── NetworkDetails
│   │   └── PortTable
│   ├── FirewallTab
│   │   └── FirewallTable
│   ├── RoutingTab
│   │   └── RoutingTable
│   ├── DockerTab
│   │   └── DockerNetworkList
│   └── PacketSenderTab
│       ├── NetworkSelector
│       ├── PacketTypeSelector
│       ├── DestinationInput
│       ├── SendButton
│       └── ResponseDisplay
└── StatusBar
```

## Main UI Layout (.slint)

```slint
import { VerticalBox, HorizontalBox, TabWidget, ScrollView, Table, Text, Button, ComboBox, LineEdit } from "std-widgets.slint";

export component MainWindow inherits Window {
    width: 1024px;
    height: 768px;
    title: "WSL Network Manager";
    
    in property <[NetworkInterface]> network-interfaces: [];
    in property <[PortInfo]> ports: [];
    in property <[FirewallRule]> firewall-rules: [];
    in property <[Route]> routes: [];
    in property <[DockerNetwork]> docker-networks: [];
    in property <string> response-text: "";
    
    out property <int> selected-network-index: -1;
    out property <string> selected-network-type: "";
    out property <string> packet-type: "ping";
    out property <string> destination: "";
    out property <string> selected-interface: "";
    
    callback network-selected(int) -> ();
    callback send-packet() -> ();
    callback refresh-data() -> ();
    
    VerticalBox {
        HorizontalBox {
            Button {
                text: "Refresh";
                clicked => {
                    refresh-data();
                }
            }
            Rectangle { background: blue; }
        }
        
        TabWidget {
            NetworkTab {
                VerticalBox {
                    HorizontalBox {
                        Text {
                            text: "Networks";
                            font-size: 18px;
                            font-weight: bold;
                        }
                    }
                    
                    HorizontalBox {
                        ScrollView {
                            width: 300px;
                            TreeView {
                                model: network-interfaces;
                                delegate: NetworkItemDelegate {
                                    // Display network name and IPs
                                }
                                current-item-changed(event) => {
                                    selected-network-index = event.row;
                                    network-selected(event.row);
                                }
                            }
                        }
                        
                        ScrollView {
                            width: 700px;
                            Table {
                                model: ports;
                                columns: [
                                    { title: "Process ID", element: port-process-id },
                                    { title: "Process Name", element: port-process-name },
                                    { title: "Protocol", element: port-protocol },
                                    { title: "Port", element: port-number },
                                    { title: "Direction", element: port-direction },
                                    { title: "Network", element: port-network }
                                ];
                            }
                        }
                    }
                }
            }
            
            FirewallTab {
                VerticalBox {
                    Text {
                        text: "Firewall Rules";
                        font-size: 18px;
                        font-weight: bold;
                    }
                    
                    ScrollView {
                        Table {
                            model: firewall-rules;
                            columns: [
                                { title: "Name", element: rule-name },
                                { title: "Enabled", element: rule-enabled },
                                { title: "Direction", element: rule-direction },
                                { title: "Action", element: rule-action },
                                { title: "Protocol", element: rule-protocol },
                                { title: "Local Address", element: rule-local-address },
                                { title: "Remote Address", element: rule-remote-address }
                            ];
                        }
                    }
                }
            }
            
            RoutingTab {
                VerticalBox {
                    Text {
                        text: "Routing Table";
                        font-size: 18px;
                        font-weight: bold;
                    }
                    
                    ScrollView {
                        Table {
                            model: routes;
                            columns: [
                                { title: "Destination", element: route-destination },
                                { title: "Gateway", element: route-gateway },
                                { title: "Interface", element: route-interface },
                                { title: "Metric", element: route-metric }
                            ];
                        }
                    }
                }
            }
            
            DockerTab {
                VerticalBox {
                    Text {
                        text: "Docker Networks";
                        font-size: 18px;
                        font-weight: bold;
                    }
                    
                    ScrollView {
                        Table {
                            model: docker-networks;
                            columns: [
                                { title: "Name", element: docker-name },
                                { title: "Driver", element: docker-driver },
                                { title: "Scope", element: docker-scope },
                                { title: "Subnet", element: docker-subnet }
                            ];
                        }
                    }
                }
            }
            
            PacketSenderTab {
                VerticalBox {
                    HorizontalBox {
                        Text {
                            text: "Packet Sender";
                            font-size: 18px;
                            font-weight: bold;
                        }
                    }
                    
                    HorizontalBox {
                        Text { text: "Source Network:"; }
                        ComboBox {
                            model: network-interfaces;
                            current-value <=> selected-interface;
                        }
                    }
                    
                    HorizontalBox {
                        Text { text: "Packet Type:"; }
                        ComboBox {
                            model: ["ping", "HTTP over TCP"];
                            current-value <=> packet-type;
                        }
                    }
                    
                    HorizontalBox {
                        Text { text: "Destination:"; }
                        LineEdit {
                            placeholder-text: "Enter IP address or hostname";
                            text <=> destination;
                        }
                    }
                    
                    Button {
                        text: "Send Packet";
                        clicked => {
                            send-packet();
                        }
                    }
                    
                    ScrollView {
                        Text {
                            text: response-text;
                            wrap: true;
                        }
                    }
                }
            }
        }
    }
    
    component NetworkItemDelegate inherits Rectangle {
        HorizontalBox {
            Text {
                text: model-data.name;
            }
            Text {
                text: model-data.ip-addresses.join(", ");
            }
        }
    }
    
    component PortProcessId inherits Text {
        text: model-data.process-id;
    }
    
    component PortProcessName inherits Text {
        text: model-data.process-name;
    }
    
    // ... other table cell components
}
```

## Data Models

### NetworkInterface Model
```slint
struct NetworkInterface {
    name: string,
    ip-addresses: [string],
    is-up: bool,
    is-loopback: bool,
}
```

### PortInfo Model
```slint
struct PortInfo {
    process-id: string,
    process-name: string,
    protocol: string,
    port: string,
    direction: string,
    network: string,
}
```

### FirewallRule Model
```slint
struct FirewallRule {
    name: string,
    enabled: string,
    direction: string,
    action: string,
    protocol: string,
    local-address: string,
    remote-address: string,
}
```

### Route Model
```slint
struct Route {
    destination: string,
    gateway: string,
    interface: string,
    metric: string,
}
```

### DockerNetwork Model
```slint
struct DockerNetwork {
    name: string,
    driver: string,
    scope: string,
    subnet: string,
}
```

## UI Interactions

1. **Network Selection**:
   - When a network is selected in the NetworkTree, the PortTable updates to show ports for that network
   - Network details are displayed in a separate panel

2. **Packet Sending**:
   - User selects source network from dropdown
   - User selects packet type (ping or HTTP)
   - User enters destination
   - Clicking "Send Packet" triggers the backend implementation
   - Response is displayed in the response area

3. **Data Refresh**:
   - Clicking "Refresh" button updates all data models
   - Status bar shows loading state during refresh

## Styling Considerations

1. **Color Scheme**:
   - Use a professional dark/light theme
   - Different colors for different network types (Windows, WSL, Docker)
   - Visual indication of network status (up/down)

2. **Responsive Design**:
   - Layout should adapt to different window sizes
   - Scrollable areas for large data sets
   - Proper spacing and alignment

3. **Accessibility**:
   - Sufficient contrast for text
   - Keyboard navigation support
   - Clear visual feedback for interactions