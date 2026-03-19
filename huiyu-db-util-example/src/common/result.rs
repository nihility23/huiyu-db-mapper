use std::marker::PhantomData;
use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize,Debug)]
pub(crate) struct Res<T>{
    pub(crate) code: i32,
    pub(crate) msg: Option<String>,
    pub(crate) data: Option<T>,
}

impl<T> Res<T>{
    pub(crate) fn new(code: i32, msg: String, data: T) -> Res<T>{
        Self{
            code,
            msg: Some(msg),
            data: Some(data)
        }
    }

    pub(crate) fn data(&self) -> &Option<T>{
        &self.data
    }

    pub(crate) fn success(t: T) ->Self<>{
        Self::new(
            0, "".to_string(), t
        )
    }

    pub(crate) fn success_without_res() ->Res<()>{
        Res::new(
            0, "".to_string(), ()
        )
    }
    pub(crate) fn fail(code: i32, msg: &str) ->Res<T>{
        Res {
            code,
            msg: Some(msg.to_string()),
            data: None,
        }
    }
}

