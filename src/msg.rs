use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Value {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct ValueResp {
    pub value: u64, //u64 unsigned integer
}

// #[entry_point]
// pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
//     use QueryMsg::*;

//     match msg {
//         Value {} => {
//             let resp = QueryResp {
//                 message: "Hello World".to_owned(),
//             };

//             to_binary(&resp)
//         }
//     }
// }
