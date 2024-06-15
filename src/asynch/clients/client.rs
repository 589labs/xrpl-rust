use crate::models::{
    requests::{Request, ServerState},
    results::{ServerState as ServerStateResult, XRPLResponse},
};
#[cfg(feature = "std")]
use crate::utils::get_random_id;
use alloc::borrow::Cow;
use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::CommonFields;

#[allow(async_fn_in_trait)]
pub trait Client<'a> {
    async fn request_impl<
        Res: Serialize + for<'de> Deserialize<'de>,
        Req: Serialize + for<'de> Deserialize<'de> + Request<'a>,
    >(
        &'a self,
        request: Req,
    ) -> Result<XRPLResponse<'_, Res, Req>>;

    fn set_request_id<
        Res: Serialize + for<'de> Deserialize<'de>,
        Req: Serialize + for<'de> Deserialize<'de> + Request<'a>,
    >(
        &'a self,
        request: &mut Req,
    ) -> Cow<'_, str> {
        let common_fields = request.get_common_fields();
        let request_id: Cow<'_, str> = match common_fields.id.clone() {
            Some(id) => id,
            None => {
                #[cfg(feature = "std")]
                {
                    let mut rng = rand::thread_rng();
                    Cow::Owned(get_random_id(&mut rng))
                }
                #[cfg(not(feature = "std"))]
                todo!("get_random_id is not yet implemented for no_std. Please provide an `id` in the request.");
            }
        };
        request.get_common_fields_mut().id = Some(request_id.clone());
        request_id
    }

    async fn get_common_fields(&'a self) -> Result<CommonFields<'a>> {
        let server_state = self
            .request_impl::<ServerStateResult, _>(ServerState::new(None))
            .await?;
        let state = server_state.result.unwrap().state;
        let common_fields = CommonFields {
            network_id: state.network_id,
            build_version: Some(state.build_version),
        };

        Ok(common_fields)
    }
}
