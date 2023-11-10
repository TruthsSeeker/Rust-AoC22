enum Element {
    Int(i32),
    List(Vec<Element>)
}

struct Packet(Vec<Element>);

impl TryFrom<&str> for Packet {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        todo!()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_from() {
        let packet = Packet::try_from("[1,1,3,1,1]").unwrap();
        assert_eq!(packet.0.len(), 5);
    }
}