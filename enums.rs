fn main() {
    // Struct way:
    // enum IpAddrKind {
    //     V4,
    //     V6,
    // }

    // let _four = IpAddrKind::V4;
    // let _six = IpAddrKind::V6;

    // fn route(ip_kind: IpAddrKind) {

    // }

    // route(IpAddrKind::V4);

    // struct IpAddr {
    //     kind: IpAddrKind,
    //     address: String,
    // }

    // let home: IpAddr = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    // Enum way
    //  enum IpAddr {
    //     V4(String),
    //     V6(String),
    // }

    // let home = IpAddr::V4(String::from("127.0.0.1")); 

    // Enhanced enums
    enum IpAddr {
        V4(u8,u8,u8,u8),
        V6(String),
    }

    let home = IpAddr::V4(127,0,0,1); 
    match home {
        IpAddr::V4(a, b, c, d) => {
            println!("IPv4 address: {}.{}.{}.{}", a, b, c, d);
        }
        IpAddr::V6(addr) => {
            println!("IPv6 address: {}", addr);
        }
    }
}