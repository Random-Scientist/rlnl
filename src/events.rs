use sealed::sealed;
pub mod ingame;
pub mod loading;
pub mod sync;

pub struct ConstEvent<const N: u8>;

#[sealed(pub(crate))]
pub trait TypedEvent {
    type Data;
    //fn parse(&self, data: &[u8]) -> Self::Data;
}

macro_rules! register_event_type {
    ($ty:ty, $( $event:ident ),+ ) => {
        $(
            #[::sealed::sealed]
            impl crate::events::TypedEvent for crate::events::ConstEvent<{ crate::event_code::NetworkEvent::$event as u8 }> {
                type Data = $ty;
            }
        )+
    };
}
pub(crate) use register_event_type;
