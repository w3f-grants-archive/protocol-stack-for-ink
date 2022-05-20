

/// Message element type define
/// U64, I64, U128, I128 will be decoded as `InkString`
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode, Clone)]
// #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
pub enum MsgType{
    InkString,
    InkU8,
    InkU16,
    InkU32,
    InkU64,
    InkU128,
    InkI8,
    InkI16,
    InkI32,
    InkI64,
    InkI128,
    UserData,
}

impl ::scale_info::TypeInfo for MsgType {
    type Identity = Self;

    fn type_info() -> ::scale_info::Type {
        ::scale_info::Type::builder()
                        .path(::scale_info::Path::new("MsgType", module_path!()))
                        .variant(
                            ::scale_info::build::Variants::new()
                                .variant("InkString", |v| v.index(0))
                                .variant("InkU8", |v| v.index(1))
                                .variant("InkU16", |v| v.index(2))
                                .variant("InkU32", |v| v.index(3))
                                .variant("InkU64", |v| v.index(4))
                                .variant("InkU128", |v| v.index(5))
                                .variant("InkI8", |v| v.index(6))
                                .variant("InkI16", |v| v.index(7))
                                .variant("InkI32", |v| v.index(8))
                                .variant("InkI64", |v| v.index(9))
                                .variant("InkI128", |v| v.index(10))
                                .variant("UserData", |v| v.index(11))
                        )
    }
}

/// Message Item, used for describing the information composed with a single element
/// @member `n`: item unique ID, which is used for user applications to communicate user-defined informations
/// @member `t`: item type
/// @member `v`: the information data, encoded by `scale::Encode::encode_to`
#[derive(Debug, Eq, scale::Encode, scale::Decode, Clone)]
// #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
pub struct MessageItem{
    pub n: u128,
    pub t: MsgType,
    pub v: ink_prelude::vec::Vec<u8>,
}

impl PartialEq for MessageItem {
    fn eq(&self, other: &MessageItem) -> bool{
        return self.n == other.n;
    }
}

impl ::scale_info::TypeInfo for MessageItem {
    type Identity = Self;

    fn type_info() -> ::scale_info::Type {
        ::scale_info::Type::builder()
                        .path(::scale_info::Path::new("MessageItem", module_path!()))
                        .composite(::scale_info::build::Fields::named()
                        .field(|f| f.ty::<u128>().name("n").type_name("u128"))
                        .field(|f| f.ty::<MsgType>().name("t").type_name("MsgType"))
                        .field(|f| f.ty::<ink_prelude::vec::Vec<u8>>().name("v").type_name("ink_prelude::vec::Vec<u8>"))
                    )
    }
}

/// Message vector, used for describing the information composed with an array of elements
/// @member `n`: item unique ID, which is used for user applications to communicate user-defined informations
/// @member `t`: item type. This is the element type of the inner array
/// @member `v`: the information data, the inner array element is encoded by `scale::Encode::encode_to`
#[derive(Debug, Eq, scale::Encode, scale::Decode, Clone)]
// #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
pub struct MessageVec{
    pub n: u128,
    pub t: MsgType,
    pub v: ink_prelude::vec::Vec<ink_prelude::vec::Vec<u8>>,
}

impl PartialEq for MessageVec {
    fn eq(&self, other: &MessageVec) -> bool{
        return self.n == other.n;
    }
}

impl ::scale_info::TypeInfo for MessageVec {
    type Identity = Self;

    fn type_info() -> ::scale_info::Type {
        ::scale_info::Type::builder()
                        .path(::scale_info::Path::new("MessageVec", module_path!()))
                        .composite(::scale_info::build::Fields::named()
                        .field(|f| f.ty::<u128>().name("n").type_name("u128"))
                        .field(|f| f.ty::<MsgType>().name("t").type_name("MsgType"))
                        .field(|f| f.ty::<ink_prelude::vec::Vec<ink_prelude::vec::Vec<u8>>>().name("v").type_name("ink_prelude::vec::Vec<ink_prelude::vec::Vec<u8>>"))
                    )
    }
}

/// Message Payload
/// @member items: a vector of `MessageItem`
/// @member vecs: a vector of `MessageVec`
#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
// #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
pub struct MessagePayload{
    pub items: Option<ink_prelude::vec::Vec<MessageItem>>,
    pub vecs: Option<ink_prelude::vec::Vec<MessageVec>>,
}

impl ::scale_info::TypeInfo for MessagePayload {
    type Identity = Self;

    fn type_info() -> ::scale_info::Type {
        ::scale_info::Type::builder()
                        .path(::scale_info::Path::new("MessagePayload", module_path!()))
                        .composite(::scale_info::build::Fields::named()
                        .field(|f| f.ty::<Option<ink_prelude::vec::Vec<MessageItem>>>().name("items").type_name("Option<ink_prelude::vec::Vec<MessageItem>>"))
                        .field(|f| f.ty::<Option<ink_prelude::vec::Vec<MessageVec>>>().name("vecs").type_name("Option<ink_prelude::vec::Vec<MessageVec>>"))
                    )
    }
}

impl MessagePayload{
    pub fn new() -> MessagePayload{
        MessagePayload {
            items: None,
            vecs: None,
        }
    }

    /// for `item`
    pub fn add_item(&mut self, msg_item: MessageItem)-> bool{
        if let Some(item) = &mut self.items {
            if item.contains(&msg_item){
                return false;
            }

            item.push(msg_item);
            true
        } else{
            let mut item_vec = ink_prelude::vec::Vec::new();
            item_vec.push(msg_item);
            self.items = Some(item_vec);
            true
        }
    }

    pub fn get_item(&self, msg_n: u128) -> Option<&MessageItem>{
        if let Some(item) = &self.items {
            for it in item.iter() {
                if it.n == msg_n {
                    return Some(it);
                }
            }
        }

        None
    }

    /// for `vecs`
    pub fn add_vec(&mut self, msg_vec: MessageVec) -> bool{
        if let Some(m_vec) = &mut self.vecs {
            if m_vec.contains(&msg_vec){
                return false;
            }
            
            m_vec.push(msg_vec);
            true
        } else {
            let mut vec_one = ink_prelude::vec::Vec::new();
            vec_one.push(msg_vec);
            self.vecs = Some(vec_one);
            true
        }
    }

    pub fn get_vec(&self, msg_n: u128) -> Option<&MessageVec>{
        if let Some(m_vec) = &self.vecs {
            for it in m_vec.iter() {
                if it.n == msg_n {
                    return Some(it);
                }
            }
        }

        None
    }
}


#[cfg(test)]
mod test {
    
}